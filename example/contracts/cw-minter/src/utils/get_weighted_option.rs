use crate::models::weights::WeightedOption;

/// Returns option based on value and options' weights
pub fn get_weighted_option<T>(mut value: u16, options: &Vec<WeightedOption<T>>) -> &T {
    // Value within total weight, [1..total_weight]
    let total_weight = options.iter().fold(0, |acc, o| acc + o.weight);
    value = (value % total_weight) + 1;

    for o in options {
        if o.weight >= value {
            return &o.value;
        }
        value -= o.weight;
    }

    &options[0].value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_index() {
        assert_eq!(
            get_weighted_option(
                0,
                &vec![
                    WeightedOption {
                        weight: 2,
                        value: 1
                    },
                    WeightedOption {
                        weight: 2,
                        value: 2
                    }
                ]
            ),
            &1
        );
    }

    #[test]
    fn base() {
        assert_eq!(
            get_weighted_option(
                1,
                &vec![
                    WeightedOption {
                        weight: 2,
                        value: 1
                    },
                    WeightedOption {
                        weight: 3,
                        value: 2
                    }
                ]
            ),
            &1
        );

        assert_eq!(
            get_weighted_option(
                2,
                &vec![
                    WeightedOption {
                        weight: 2,
                        value: 1
                    },
                    WeightedOption {
                        weight: 3,
                        value: 2
                    }
                ]
            ),
            &2
        );

        assert_eq!(
            get_weighted_option(
                4,
                &vec![
                    WeightedOption {
                        weight: 2,
                        value: 1
                    },
                    WeightedOption {
                        weight: 3,
                        value: 2
                    }
                ]
            ),
            &2
        );
    }

    #[test]
    fn overflow() {
        assert_eq!(
            get_weighted_option(
                5,
                &vec![
                    WeightedOption {
                        weight: 2,
                        value: 1
                    },
                    WeightedOption {
                        weight: 3,
                        value: 2
                    }
                ]
            ),
            &1
        );
    }
}
