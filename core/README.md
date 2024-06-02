# `cw-constructor`

## Use as a dependency:

```toml
[dependencies]
cw_constructor = "0.1.0"
```

## API

### Query msgs

#### `ContractInfo`
Get constructor contract's config
```rust
ContractInfoMsg {}
```

#### `Traits`
Get filtered trait tokens, e.g. by related base token's `token_id` or name of the `slot`
```rust
TraitsMsg {
  slot: Option<String>,
  token_id: Option<String>,
}
```

#### `Tokens`
Get filtered base tokens, e.g. by `token_id` or `address` of applied trait
```rust
TokensMsg {
  token_id: Option<String>,
  address: Option<String>,
}
```

#### `Info`
Get *aggregated* and *separate* metadata of the base token and its applied trait tokens
```rust
InfoMsg {
  token_id: String,
}
```

### Execute msgs

#### `Equip`
Equip new trait tokens to the base token
```rust
EquipMsg {
  token_id: String,
  traits: Vec<TokenConfig {
    address: String,
    token_id: String,
  }>
}
```

#### `Unequip`
Remove equipped trait tokens
```rust
UnequipMsg {
  traits: Vec<TokenConfig {
    address: String,
    token_id: String,
  }>
}
```
