use crate::{error_handling::*, settings_mod::Settings, MetaFuncRegex};

impl MetaFuncRegex {
    pub fn new(raw: String) -> Self {
        MetaFuncRegex(raw)
    }

    pub fn make_literal_exp(
        mut self,
        expression: &str,
        settings: &Settings,
    ) -> Result<MetaFuncRegex, String> {
        if settings.range.is_some() && (settings.is_nil_or_more || settings.is_one_or_more) {
            let err = error_builder(expression, ERR_RANGE_AND_STAR_OR_PLUS_EXP);
            Err(err)
        } else if settings.is_nil_or_more && settings.is_one_or_more {
            let err = error_builder(expression, ERR_STAR_AND_PLUS_EXP);
            Err(err)
        } else if settings.exact_amount.is_some() && settings.range.is_some() {
            let err = error_builder(expression, ERR_RANGE_AND_EXACT_EXP);
            Err(err)
        } else {
            let mut final_result = expression.to_string();

            if settings.with_left_boundary {
                final_result = format!("{}{}", "\\b", final_result);
            }
            if settings.range.is_some()
                && (settings.range.unwrap().0.is_some() || settings.range.unwrap().1.is_some())
            {
                let numbers: (Option<u8>, Option<u8>) = settings.range.unwrap();

                final_result = format!("{}{}", final_result, "{");

                if let Some(start_range) = numbers.0 {
                    final_result = format!("{}{}", final_result, start_range);
                }

                final_result = format!("{}{}", final_result, ",");

                if let Some(end_range) = numbers.1 {
                    final_result = format!("{}{}", final_result, end_range);
                }

                final_result = format!("{}{}", final_result, "}");
            }
            if settings.is_nil_or_more {
                final_result = format!("{}{}", final_result, "*");
            }
            if settings.is_one_or_more {
                final_result = format!("{}{}", final_result, "+");
            }
            if settings.is_optional {
                final_result = format!("{}{}", final_result, "?");
            }
            if settings.with_right_boundary {
                final_result = format!("{}{}", final_result, "\\b");
            }

            self.0 = format!("{}{}", &self.0, final_result);
            Ok(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use self::MetaFuncRegex;
    use super::*;

    #[test]
    fn make_exp_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp
            .make_literal_exp("abcd", &Settings::default())
            .unwrap();
        assert_eq!(result.0, "initial_abcd");
    }

    #[test]
    fn make_exp_optional_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp
            .make_literal_exp(
                "abcd",
                &Settings {
                    is_optional: true,
                    is_nil_or_more: false,
                    is_one_or_more: false,
                    with_left_boundary: false,
                    with_right_boundary: false,
                    range: None,
                    exact_amount: None,
                },
            )
            .unwrap();
        assert_eq!(result.0, "initial_abcd?");
    }

    #[test]
    fn make_exp_range_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp
            .make_literal_exp(
                "abcd",
                &Settings {
                    is_optional: true,
                    is_nil_or_more: false,
                    is_one_or_more: false,
                    with_left_boundary: false,
                    with_right_boundary: false,
                    range: Some((None, Some(2))),
                    exact_amount: None,
                },
            )
            .unwrap();
        assert_eq!(result.0, "initial_abcd{,2}?");
    }

    #[test]
    fn make_exp_boundary_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp
            .make_literal_exp(
                "abcd",
                &Settings {
                    is_optional: false,
                    is_nil_or_more: false,
                    is_one_or_more: false,
                    with_left_boundary: true,
                    with_right_boundary: true,
                    range: None,
                    exact_amount: None,
                },
            )
            .unwrap();
        assert_eq!(result.0, "initial_\\babcd\\b");
    }

    #[test]
    fn make_exp_mixed_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp
            .make_literal_exp(
                "ab\\scd",
                &Settings {
                    is_optional: true,
                    is_nil_or_more: false,
                    is_one_or_more: false,
                    with_left_boundary: true,
                    with_right_boundary: true,
                    range: Some((None, Some(5))),
                    exact_amount: None,
                },
            )
            .unwrap();
        assert_eq!(result.0, "initial_\\bab\\scd{,5}?\\b");
    }

    ////////////////////////////////////////////////// ERRORS /////////////////////////////////////////////////////
    #[test]
    fn make_exp_not_works_with_range_and_start_and_plus() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.make_literal_exp(
            "ab\\scd",
            &Settings {
                is_optional: false,
                is_nil_or_more: true,
                is_one_or_more: false,
                with_left_boundary: false,
                with_right_boundary: false,
                range: Some((Some(2), None)),
                exact_amount: None,
            },
        );
        let err = error_builder("ab\\scd", ERR_RANGE_AND_STAR_OR_PLUS_EXP);

        assert_eq!(err, result.unwrap_err());
    }

    #[test]
    fn make_exp_not_works_with_star_and_plus() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.make_literal_exp(
            "ab\\scd",
            &Settings {
                is_optional: false,
                is_nil_or_more: true,
                is_one_or_more: true,
                with_left_boundary: false,
                with_right_boundary: false,
                range: None,
                exact_amount: None,
            },
        );
        let err = error_builder("ab\\scd", ERR_STAR_AND_PLUS_EXP);

        assert_eq!(err, result.unwrap_err());
    }

    #[test]
    fn make_exp_not_works_with_range_and_exact() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.make_literal_exp(
            "ab\\scd",
            &Settings {
                is_optional: false,
                is_nil_or_more: false,
                is_one_or_more: false,
                with_left_boundary: false,
                with_right_boundary: false,
                range: Some((Some(2), Some(10))),
                exact_amount: Some(8),
            },
        );
        let err = error_builder("ab\\scd", ERR_RANGE_AND_EXACT_EXP);

        assert_eq!(err, result.unwrap_err());
    }
}
