# cw-constructor

Equip nft traits for a token and render metadata on the fly

# Projects

## Core

- [🧱 `core`](./core) - Core contract to bind NFTs-as-traits

## Example

<div align="center">
  <img src=".github/cover.jpg" alt="Fiend Frens" height="200">
</div>

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
- [🚀 `scripts`](./example/scripts) - Scripts to upload assets on IPFS / instantiate example contracts etc

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

### Store built wasm code
```sh
ACCOUNT=dev CHAIN_ID=constantine-3 GAS_ADJUSTMENT=1.4 DRY_RUN=1 sh ./scripts/store.sh target/wasm32-unknown-unknown/release/cw721_fiend_frens_trait.wasm
```

...where the arguments are:
| arg | description  | default |
| - | - | - |
| `$1` | `wasm` file to store onchain | - |
| `$ACCOUNT` | `archwayd` account's name / address to use as the sender | - |
| `$CHAIN_ID` | chain ID, e.g. `archway-1`, `constantine-3` or `titus-3` | `constantine-3` |
| `$GAS_ADJUSTMENT` | gas adjustment coefficient | `1.4` |
| `$DRY_RUN` | `1` if dry-run store tx without broadcasting it, `0` otherwise (make sure to provide *address* instead of account *name* in `$ACCOUNT` for dry-run!) | `0` |
