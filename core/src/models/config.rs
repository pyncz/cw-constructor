use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct SlotConfig<A: Into<String> = Addr> {
    /// Name of the slot
    pub name: String,
    /// Addresses of the contracts which tokens are allowed to be used as traits for this slot
    pub allowed_contracts: Vec<A>,

    // flags:
    /// If allow to add multiple traits for this slot at the same time
    pub allow_multiple: bool,
}

#[cw_serde]
pub struct ContractInfo<A: Into<String> = Addr> {
    /// Address of the base cw721 token to be extended
    pub base_token: A,
    /// Config allowed slots
    pub slots: Vec<SlotConfig<A>>,
    /// Addresses of the accounts authorized to alter the contract config
    pub admins: Vec<A>,
}
