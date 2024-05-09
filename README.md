# cw-constructor

Equip nft traits for a token and render metadata on the fly

# Projects

## Core

- [🧱 `core`](./core) - Core contract to bind NFTs-as-traits

## Example

<img src=".github/logo.png" alt="Fiend Frens" width="200">

Fiend Frens demo collection, that uses `cw-constructor` contract.

### Contracts

- [`cw-minter`](./example/contracts/cw-minter) - _Base_ minter contract to use for base and trait tokens

#### 💎 Base token
- [`cw721-fiend-frens`](./example/contracts/cw721-fiend-frens) - Main `cw721` contract that serves as the base token
- [ `cw-fiend-frens-minter`](./example/contracts/cw-fiend-frens-minter) - Minter contract for the base token

#### 📎 Trait token
- [`cw721-fiend-frens-trait`](./example/contracts/cw721-fiend-frens-trait) - A `cw721` contract that serves as a trait token
- [`cw-fiend-frens-trait-minter`](./example/contracts/cw-fiend-frens-trait-minter) - Minter contract for trait tokens

#### 🔩 Constructor
- [`cw-fiend-frens-constructor`](./example/contracts/cw-fiend-frens-constructor) - Constructor contract to manage applied trait NFTs

### Helpers
- [🚀 `scripts`](./example/scripts) - Scripts to upload on IPFS / deploy example NFTs

### Client
- [🖥️ `client`](./example/client) - Frontend to view / equip / unequip traits


# Scripts

### Test
```sh
sh ./scripts/test.sh
```

### Build for release
```sh
sh ./scripts/build.sh
```

### Optimize build wasm artifacts
```sh
sh ./scripts/optimize.sh
```

### Generate schema json
```sh
sh ./scripts/schema.sh
```
