use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal as CwDecimal;
use cw_constructor::models::metadata::MergeWithTraitExtension;

// Display types
#[cw_serde]
pub enum DisplayType {
    Number,
}

// Base token extension
#[cw_serde]
pub struct Attribute {
    pub trait_type: String,
    pub display_type: Option<DisplayType>,
    pub value: String, // u64 for DisplayType::Number
}

#[cw_serde]
pub struct Extension {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

// Merged extension
#[cw_serde]
pub enum MergedDisplayType {
    Number,
}

#[cw_serde]
pub struct MergedAttribute {
    pub trait_type: String,
    pub display_type: Option<MergedDisplayType>,
    pub value: String, // CwDecimal for MergedDisplayType::Number
}

impl From<&Attribute> for MergedAttribute {
    fn from(value: &Attribute) -> Self {
        match value.display_type {
            Some(DisplayType::Number) => MergedAttribute {
                trait_type: value.trait_type.clone(),
                display_type: Some(MergedDisplayType::Number),
                value: CwDecimal::from_str(&value.value)
                    .unwrap_or_default()
                    .to_string(),
            },
            _ => MergedAttribute {
                trait_type: value.trait_type.clone(),
                display_type: None,
                value: value.value.clone(),
            },
        }
    }
}

#[cw_serde]
pub struct MergedExtension {
    pub name: String,
    pub description: String,
    pub images: Vec<String>,
    pub attributes: Vec<MergedAttribute>,
}

impl From<Extension> for MergedExtension {
    fn from(value: Extension) -> Self {
        let attributes: Vec<MergedAttribute> =
            value.attributes.iter().map(MergedAttribute::from).collect();

        MergedExtension {
            name: value.name,
            description: value.description,
            images: vec![value.image],
            attributes,
        }
    }
}

// Trait token extension
#[cw_serde]
pub enum TraitDisplayType {
    BoostNumber,
    BoostPercentage,
}

#[cw_serde]
pub struct TraitAttribute {
    pub trait_type: String,
    pub display_type: Option<TraitDisplayType>,
    pub value: String, // i64 for TraitDisplayType::BoostNumber / ::BoostPercentage
}

#[cw_serde]
pub struct TraitExtension {
    pub name: String,
    pub image: Option<String>,
    pub attributes: Vec<TraitAttribute>,
}

impl MergeWithTraitExtension<TraitExtension> for MergedExtension {
    fn merge(&mut self, extensions: Vec<&TraitExtension>) {
        // Aggregate all the update values first without updating the self *in-place*
        // to be able to apply 0 floor as the *last* step AND re-use the base values in calculations
        self.attributes.iter_mut().for_each(|base_attr| {
            // Update the attribute only if it's not a string attr
            if let Some(MergedDisplayType::Number) = base_attr.display_type {
                let initial_value = CwDecimal::from_str(&base_attr.value).unwrap_or_default();

                // Collect all the similar attributes accross traits' extensions
                let t_attrs = extensions.iter().flat_map(|t| {
                    t.attributes.iter().filter(|a| {
                        a.trait_type == base_attr.trait_type && a.display_type.is_some()
                    })
                });

                // Aggregate number and percentage values *separately* to apply them
                // to the base value at *last* step
                let mut number_diff = 0i64;
                let mut percentage_diff = 0i64;
                t_attrs.for_each(|a| match a.display_type {
                    Some(TraitDisplayType::BoostNumber) => {
                        number_diff += i64::from_str(&a.value).unwrap_or_default();
                    }
                    Some(TraitDisplayType::BoostPercentage) => {
                        percentage_diff += i64::from_str(&a.value).unwrap_or_default();
                    }
                    _ => (),
                });

                // - number
                let mut negative = number_diff < 0;
                let mut diff =
                    CwDecimal::from_str(&number_diff.abs().to_string()).unwrap_or_default();
                // - percentage
                let coeff = CwDecimal::percent(percentage_diff.unsigned_abs());

                if let Ok(p_diff) = coeff.checked_mul(initial_value) {
                    if percentage_diff < 0 && number_diff < 0
                        || percentage_diff > 0 && number_diff > 0
                    {
                        // Both components negative or both positive, increase diff
                        diff = diff.checked_add(p_diff).unwrap_or(CwDecimal::MAX);
                    } else if number_diff == 0 {
                        diff = p_diff;
                        negative = percentage_diff < 0;
                    } else {
                        // Number and percentage components are of different signs
                        if p_diff > diff {
                            // Passed 0, invert negative
                            diff = p_diff - diff;
                            negative = !negative;
                        } else {
                            // Decrease diff
                            diff -= p_diff;
                        }
                    }
                }

                // Apply changes to the base attribute's value
                base_attr.value = match negative {
                    true => initial_value
                        .checked_sub(diff)
                        .unwrap_or(CwDecimal::MIN)
                        .to_string(),
                    false => initial_value
                        .checked_add(diff)
                        .unwrap_or(CwDecimal::MAX)
                        .to_string(),
                };
            }
        });

        // Append traits' images to the result image list
        extensions.iter().for_each(|t| {
            if let Some(image) = &t.image {
                self.images.push(image.clone());
            }
        });
    }
}
