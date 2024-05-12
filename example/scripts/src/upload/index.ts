// Upload to IPFS
import fs from 'fs';
import { dirname, resolve } from 'path';
import { fileURLToPath } from 'url';

import { config as loadEnv } from 'dotenv';
import PinataSdk from '@pinata/sdk';
import { consola } from 'consola';
import { requireEnv } from '../utils';

const __dirname = dirname(fileURLToPath(import.meta.url));

// Load env
const { parsed: env } = loadEnv();
const PINATA_API_KEY = requireEnv('PINATA_API_KEY', env);
const PINATA_API_SECRET = requireEnv('PINATA_API_SECRET', env);

// Run script
let count = 0;
const pinata = new PinataSdk({ pinataApiKey: PINATA_API_KEY, pinataSecretApiKey: PINATA_API_SECRET });

try {
  consola.start('Uploading images to IPFS...');

  const assetsPath = resolve(__dirname, '../../assets');
  const dirs = fs.readdirSync(assetsPath);
  for (const collection of dirs) {
    consola.info({ message: `Processing ${collection}`, level: 1 });

    const variants = fs.readdirSync(`${assetsPath}/${collection}`);
    for (const variant of variants) {
      consola.log(`Processing ${collection}/${variant}`);

      // Upload to Pinata
      const variantPath = `${assetsPath}/${collection}/${variant}`;
      const imageStream = fs.createReadStream(`${variantPath}/image.png`);
      const { IpfsHash: ipfsCid } = await pinata.pinFileToIPFS(imageStream, {
        pinataMetadata: {
          name: `${collection}-${variant}`,
          project: collection,
          variant,
        },
      });
      const ipfsPath = `ipfs://${ipfsCid}`;
      consola.success(`Uploaded to ${ipfsPath}`);
      count++;

      // Read related metadata
      const metadataPath = `${variantPath}/metadata.json`;
      const metadataContent = fs.readFileSync(metadataPath, 'utf-8');
      const metadata = JSON.parse(metadataContent);
      // Update metadata with image cid
      metadata.image = ipfsPath;
      fs.writeFileSync(metadataPath, `${JSON.stringify(metadata, null, 2)}\n`);
    }
  }
} catch (e) {
  consola.error(e);
}

if (count) {
  consola.success({ message: `${count} file(s) uploaded!`, level: 1 });
} else {
  consola.info({ message: 'No files uploaded.', level: 1 });
}
