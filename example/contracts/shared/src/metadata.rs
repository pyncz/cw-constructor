use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal as CwDecimal;
use cw_constructor::models::metadata::MergeWithTraitExtension;

// Display types
#[cw_serde]
pub enum NumberDisplayType {
    Number,
}

#[cw_serde]
pub enum DecimalDisplayType {
    Decimal,
}

// Base token extension
#[cw_serde]
pub struct NumberAttribute {
    pub trait_type: String,
    pub display_type: NumberDisplayType,
    pub value: u64,
}

#[cw_serde]
pub struct StringAttribute {
    pub trait_type: String,
    pub value: String,
}

#[cw_serde]
pub enum Attribute {
    Number(NumberAttribute),
    String(StringAttribute),
}

#[cw_serde]
pub struct Extension {
    pub name: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

// Merged extension
#[cw_serde]
pub struct DecimalAttribute {
    pub trait_type: String,
    pub display_type: DecimalDisplayType,
    pub value: CwDecimal,
}

#[cw_serde]
pub enum MergedAttribute {
    String(StringAttribute),
    Decimal(DecimalAttribute),
}

impl From<&Attribute> for MergedAttribute {
    fn from(value: &Attribute) -> Self {
        match value {
            Attribute::Number(a) => MergedAttribute::Decimal(DecimalAttribute {
                trait_type: a.trait_type.clone(),
                display_type: DecimalDisplayType::Decimal,
                value: CwDecimal::from_str(a.value.to_string().as_str()).unwrap_or_default(),
            }),
            Attribute::String(a) => MergedAttribute::String(a.clone()),
        }
    }
}

#[cw_serde]
pub struct MergedExtension {
    pub name: String,
    pub images: Vec<String>,
    pub attributes: Vec<MergedAttribute>,
}

impl From<&Extension> for MergedExtension {
    fn from(value: &Extension) -> Self {
        let attributes: Vec<MergedAttribute> = value
            .attributes
            .iter()
            .map(|a| MergedAttribute::from(a))
            .collect();

        MergedExtension {
            name: value.name.clone(),
            images: vec![value.image.clone()],
            attributes,
        }
    }
}

// Trait token extension
#[cw_serde]
pub enum TraitNumberAttributeDisplayType {
    BoostNumber,
    BoostPercentage,
}

#[cw_serde]
pub struct TraitNumberAttribute {
    pub trait_type: String,
    pub display_type: TraitNumberAttributeDisplayType,
    pub value: i64,
}

#[cw_serde]
pub enum TraitAttribute {
    Number(TraitNumberAttribute),
    String(StringAttribute),
}

impl TraitAttribute {
    fn trait_type(&self) -> &str {
        match self {
            TraitAttribute::Number(a) => &a.trait_type,
            TraitAttribute::String(a) => &a.trait_type,
        }
    }
}

#[cw_serde]
pub struct TraitExtension {
    pub image: Option<String>,
    pub attributes: Vec<TraitAttribute>,
}

impl MergeWithTraitExtension<TraitExtension> for MergedExtension {
    fn merge(&mut self, extensions: Vec<&TraitExtension>) {
        // Aggregate all the update values first without updating the self *in-place*
        // to be able to apply 0 floor as the *last* step AND re-use the base values in calculations
        self.attributes.iter_mut().for_each(|base_attr| {
            // Update the attribute only if it's not a string attr
            if let MergedAttribute::Decimal(base_attr) = base_attr {
                let initial_value = base_attr.value;

                // Collect all the similar attributes accross traits' extensions
                let t_attrs = extensions.iter().flat_map(|t| {
                    t.attributes.iter().filter_map(|a| {
                        if a.trait_type() == base_attr.trait_type {
                            if let TraitAttribute::Number(number_a) = a {
                                return Some(number_a);
                            }
                        }
                        None
                    })
                });

                // Aggregate number and percentage values *separately* to apply them
                // to the base value at *last* step
                let mut number_diff = 0i64;
                let mut percentage_diff = 0i64;
                t_attrs.for_each(|a| match a.display_type {
                    TraitNumberAttributeDisplayType::BoostNumber => {
                        number_diff += a.value;
                    }
                    TraitNumberAttributeDisplayType::BoostPercentage => {
                        percentage_diff += a.value;
                    }
                });

                // - number
                let mut negative = number_diff < 0;
                let mut diff =
                    CwDecimal::from_str(number_diff.abs().to_string().as_str()).unwrap_or_default();
                // - percentage
                let coeff = CwDecimal::percent(percentage_diff.abs() as u64);

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
                            diff = diff - p_diff;
                        }
                    }
                }

                // Apply changes to the base attribute's value
                base_attr.value = match negative {
                    true => base_attr.value.checked_sub(diff).unwrap_or(CwDecimal::MIN),
                    false => base_attr.value.checked_add(diff).unwrap_or(CwDecimal::MAX),
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
