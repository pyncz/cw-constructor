use cosmwasm_schema::cw_serde;

#[cw_serde]
#[derive(Default)]
pub struct Extension {
    pub option: u32,
}
