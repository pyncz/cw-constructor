use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct WeightedOption<T> {
    pub weight: u16,
    pub value: T,
}
