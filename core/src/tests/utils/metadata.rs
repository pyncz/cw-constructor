use crate::models::metadata::MergeWithTraitExtension;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Extension {
    pub name: String,
    pub value: u128,
}

#[cw_serde]
pub struct MergedExtension {
    pub value: u128,
}

impl From<Extension> for MergedExtension {
    fn from(extension: Extension) -> Self {
        MergedExtension {
            value: extension.value,
        }
    }
}

#[cw_serde]
pub struct TraitExtension {
    pub value: u128,
}

impl MergeWithTraitExtension<TraitExtension, Extension> for MergedExtension {
    fn merge(&mut self, extension: &TraitExtension, _original_extension: &Extension) {
        self.value = self.value + extension.value;
    }
}
