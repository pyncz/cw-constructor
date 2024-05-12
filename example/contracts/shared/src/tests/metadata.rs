#![cfg(test)]
use std::str::FromStr;

use crate::metadata::{
    Attribute, DecimalAttribute, DecimalDisplayType, Extension, MergedAttribute, MergedExtension,
    NumberAttribute, NumberDisplayType, StringAttribute, TraitAttribute, TraitExtension,
    TraitNumberAttribute, TraitNumberAttributeDisplayType,
};
use cosmwasm_std::Decimal as CwDecimal;
use cw_constructor::models::metadata::MergeWithTraitExtension;
use serde_json::{json, to_string};

/// Test if keeps not-numeric attributes
#[test]
fn keep_string_attributes() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::String(StringAttribute {
            trait_type: "Kind".to_string(),
            value: "Blue".to_string(),
        })],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::String(StringAttribute {
                trait_type: "Kind".to_string(),
                value: "Blue".to_string(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Kind",
                    "value": "Blue"
                }
            ]
        })
        .to_string()
    );
}

/// Test if ignores attributes not featured in the original extension
#[test]
fn ignore_unknown_attributes() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::String(StringAttribute {
            trait_type: "Kind".to_string(),
            value: "Blue".to_string(),
        })],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![TraitAttribute::Number(TraitNumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: TraitNumberAttributeDisplayType::BoostNumber,
            value: 2,
        })],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::String(StringAttribute {
                trait_type: "Kind".to_string(),
                value: "Blue".to_string(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Kind",
                    "value": "Blue"
                }
            ]
        })
        .to_string()
    );
}

// BoostNumber attributes
/// Test basic adding a numeric attribute
#[test]
fn boost_number_positive() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 3,
        })],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![TraitAttribute::Number(TraitNumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: TraitNumberAttributeDisplayType::BoostNumber,
            value: 2,
        })],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::Decimal(DecimalAttribute {
                trait_type: "Rage".to_string(),
                display_type: DecimalDisplayType::Decimal,
                value: CwDecimal::from_str("5").unwrap(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "5"
                }
            ]
        })
        .to_string()
    );
}

/// Test adding multiple numeric attributes
#[test]
fn boost_number_positive_multiple() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 3,
        })],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostNumber,
                value: 2,
            }),
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostNumber,
                value: 1,
            }),
        ],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::Decimal(DecimalAttribute {
                trait_type: "Rage".to_string(),
                display_type: DecimalDisplayType::Decimal,
                value: CwDecimal::from_str("6").unwrap(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "6"
                }
            ]
        })
        .to_string()
    );
}

/// Test adding a negative numeric attribute
#[test]
fn boost_number_negative() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 3,
        })],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![TraitAttribute::Number(TraitNumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: TraitNumberAttributeDisplayType::BoostNumber,
            value: -2,
        })],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::Decimal(DecimalAttribute {
                trait_type: "Rage".to_string(),
                display_type: DecimalDisplayType::Decimal,
                value: CwDecimal::from_str("1").unwrap(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "1"
                }
            ]
        })
        .to_string()
    );
}

/// Test adding a negative numeric attribute resulting in at least 0
#[test]
fn boost_number_negative_floor() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 3,
        })],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![TraitAttribute::Number(TraitNumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: TraitNumberAttributeDisplayType::BoostNumber,
            value: -4,
        })],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::Decimal(DecimalAttribute {
                trait_type: "Rage".to_string(),
                display_type: DecimalDisplayType::Decimal,
                value: CwDecimal::from_str("0").unwrap(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "0"
                }
            ]
        })
        .to_string()
    );
}

/// Test getting the same result for different numeric trait attributes' order
#[test]
fn boost_number_order() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 3,
        })],
    };

    let mut ext1: MergedExtension = base.clone().into();
    ext1.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostNumber,
                value: -4,
            }),
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostNumber,
                value: 2,
            }),
        ],
    }]);

    let mut ext2: MergedExtension = base.clone().into();
    ext2.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostNumber,
                value: 2,
            }),
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostNumber,
                value: -4,
            }),
        ],
    }]);

    assert_eq!(ext1, ext2);

    let serialized_ext = to_string(&ext1).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "1"
                }
            ]
        })
        .to_string()
    );
}

// BoostPercentage attributes
/// Test basic adding a percentage attribute
#[test]
fn boost_percentage_positive() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 4,
        })],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![TraitAttribute::Number(TraitNumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: TraitNumberAttributeDisplayType::BoostPercentage,
            value: 50,
        })],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::Decimal(DecimalAttribute {
                trait_type: "Rage".to_string(),
                display_type: DecimalDisplayType::Decimal,
                value: CwDecimal::from_str("6").unwrap(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "6"
                }
            ]
        })
        .to_string()
    );
}

/// Test adding multiple percentage attributes
#[test]
fn boost_percentage_positive_multiple() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 4,
        })],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostPercentage,
                value: 50,
            }),
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostPercentage,
                value: 25,
            }),
        ],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::Decimal(DecimalAttribute {
                trait_type: "Rage".to_string(),
                display_type: DecimalDisplayType::Decimal,
                value: CwDecimal::from_str("7").unwrap(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "7"
                }
            ]
        })
        .to_string()
    );
}

/// Test adding a percentage attribute boosting more than 100%
#[test]
fn boost_percentage_positive_lt_100() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 4,
        })],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![TraitAttribute::Number(TraitNumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: TraitNumberAttributeDisplayType::BoostPercentage,
            value: 101,
        })],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::Decimal(DecimalAttribute {
                trait_type: "Rage".to_string(),
                display_type: DecimalDisplayType::Decimal,
                value: CwDecimal::from_str("8.04").unwrap(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "8.04"
                }
            ]
        })
        .to_string()
    );
}

/// Test adding a negative percentage attribute
#[test]
fn boost_percentage_negative() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 4,
        })],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![TraitAttribute::Number(TraitNumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: TraitNumberAttributeDisplayType::BoostPercentage,
            value: -50,
        })],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::Decimal(DecimalAttribute {
                trait_type: "Rage".to_string(),
                display_type: DecimalDisplayType::Decimal,
                value: CwDecimal::from_str("2").unwrap(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "2"
                }
            ]
        })
        .to_string()
    );
}

/// Test adding a percentage numeric attribute resulting in at least 0
#[test]
fn boost_percentage_negative_floor() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 4,
        })],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![TraitAttribute::Number(TraitNumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: TraitNumberAttributeDisplayType::BoostPercentage,
            value: -150,
        })],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::Decimal(DecimalAttribute {
                trait_type: "Rage".to_string(),
                display_type: DecimalDisplayType::Decimal,
                value: CwDecimal::from_str("0").unwrap(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "0"
                }
            ]
        })
        .to_string()
    );
}

/// Test getting the same result for different percentage trait attributes' order
#[test]
fn boost_percentage_order() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 4,
        })],
    };

    let mut ext1: MergedExtension = base.clone().into();
    ext1.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostPercentage,
                value: -200,
            }),
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostPercentage,
                value: 200,
            }),
        ],
    }]);

    let mut ext2: MergedExtension = base.clone().into();
    ext2.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostPercentage,
                value: 200,
            }),
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostPercentage,
                value: -200,
            }),
        ],
    }]);

    assert_eq!(ext1, ext2);

    let serialized_ext = to_string(&ext1).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "4"
                }
            ]
        })
        .to_string()
    );
}

// BoostPercentage attributes

/// Test getting the same result for different percentage trait attributes' order
#[test]
fn boost_combined() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![Attribute::Number(NumberAttribute {
            trait_type: "Rage".to_string(),
            display_type: NumberDisplayType::Number,
            value: 4,
        })],
    };

    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostNumber,
                value: 2,
            }),
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostPercentage,
                value: 100,
            }),
        ],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![MergedAttribute::Decimal(DecimalAttribute {
                trait_type: "Rage".to_string(),
                display_type: DecimalDisplayType::Decimal,
                value: CwDecimal::from_str("10").unwrap(),
            })],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "10"
                }
            ]
        })
        .to_string()
    );
}

/// Test if processes multiple different attributes
#[test]
fn accept_multiple_attributes() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![
            Attribute::Number(NumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: NumberDisplayType::Number,
                value: 4,
            }),
            Attribute::Number(NumberAttribute {
                trait_type: "Deception".to_string(),
                display_type: NumberDisplayType::Number,
                value: 2,
            }),
        ],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: None,
        attributes: vec![
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Rage".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostNumber,
                value: 2,
            }),
            TraitAttribute::Number(TraitNumberAttribute {
                trait_type: "Deception".to_string(),
                display_type: TraitNumberAttributeDisplayType::BoostNumber,
                value: 1,
            }),
        ],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string()],
            attributes: vec![
                MergedAttribute::Decimal(DecimalAttribute {
                    trait_type: "Rage".to_string(),
                    display_type: DecimalDisplayType::Decimal,
                    value: CwDecimal::from_str("6").unwrap(),
                }),
                MergedAttribute::Decimal(DecimalAttribute {
                    trait_type: "Deception".to_string(),
                    display_type: DecimalDisplayType::Decimal,
                    value: CwDecimal::from_str("3").unwrap(),
                })
            ],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image"
            ],
            "attributes": [
                {
                    "trait_type": "Rage",
                    "display_type": "decimal",
                    "value": "6"
                },
                {
                    "trait_type": "Deception",
                    "display_type": "decimal",
                    "value": "3"
                }
            ]
        })
        .to_string()
    );
}

/// Test basic aggregation of traits' images
#[test]
fn aggregate_images() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![&TraitExtension {
        image: Some("trait-image".to_string()),
        attributes: vec![],
    }]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec!["image".to_string(), "trait-image".to_string()],
            attributes: vec![],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image",
                "trait-image"
            ],
            "attributes": []
        })
        .to_string()
    );
}

/// Test aggregation of multiple optional traits' images
#[test]
fn aggregate_images_multiple() {
    let base = &Extension {
        name: "Gregg".to_string(),
        description: "Gregg loves oranges".to_string(),
        image: "image".to_string(),
        attributes: vec![],
    };
    let mut ext: MergedExtension = base.clone().into();

    ext.merge(vec![
        &TraitExtension {
            image: Some("trait-image-1".to_string()),
            attributes: vec![],
        },
        &TraitExtension {
            image: None,
            attributes: vec![],
        },
        &TraitExtension {
            image: Some("trait-image-2".to_string()),
            attributes: vec![],
        },
    ]);

    assert_eq!(
        ext,
        MergedExtension {
            name: "Gregg".to_string(),
            description: "Gregg loves oranges".to_string(),
            images: vec![
                "image".to_string(),
                "trait-image-1".to_string(),
                "trait-image-2".to_string()
            ],
            attributes: vec![],
        }
    );

    let serialized_ext = to_string(&ext).unwrap();
    assert_eq!(
        serialized_ext,
        json!({
            "name": "Gregg",
            "description": "Gregg loves oranges",
            "images": [
                "image",
                "trait-image-1",
                "trait-image-2"
            ],
            "attributes": []
        })
        .to_string()
    );
}
