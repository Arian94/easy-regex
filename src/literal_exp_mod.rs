use crate::{error_handling::*, settings_mod::Settings, MetaFuncRegex};

impl MetaFuncRegex {
    pub fn literal_exp(
        self,
        expression: &str,
        settings: &Settings,
    ) -> Result<MetaFuncRegex, String> {
        if settings.range.is_some() && (settings.is_nil_or_more || settings.is_one_or_more) {
            let err = error_builder(expression, &ERR_RANGE_AND_STAR_OR_PLUS_EXP);
            Err(err)
        } else if settings.is_nil_or_more && settings.is_one_or_more {
            let err = error_builder(expression, &ERR_STAR_AND_PLUS_EXP);
            Err(err)
        } else if settings.exactly.is_some() && settings.range.is_some() {
            let err = error_builder(expression, &ERR_RANGE_AND_EXACT_EXP);
            Err(err)
        } else {
            let mut final_result = expression.to_string();

            if settings.with_left_boundary {
                final_result = format!("{}{}", "\\b", final_result);
            }
            if settings.with_left_non_boundary {
                final_result = format!("{}{}", "\\B", final_result);
            }
            if settings.range.is_some()
                && (settings.range.unwrap().0.is_some() || settings.range.unwrap().1.is_some())
            {
                let numbers = settings.range.unwrap();

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
            if let Some(number) = settings.exactly {
                final_result = format!("{}{}{}{}", final_result, "{", number, "}");
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
            if settings.with_right_non_boundary {
                final_result = format!("{}{}", final_result, "\\B");
            }

            final_result = format!("{}{}", &self.0, final_result);
            Ok(MetaFuncRegex(final_result))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::settings_mod::{NO_SETTINGS, OPTIONAL};

    use self::MetaFuncRegex;
    use super::*;

    #[test]
    fn make_exp_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.literal_exp("abcd", &NO_SETTINGS).unwrap();
        assert_eq!(result.0, "initial_abcd");
    }

    #[test]
    fn make_exp_optional_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.literal_exp("abcd", &OPTIONAL).unwrap();
        assert_eq!(result.0, "initial_abcd?");
    }

    #[test]
    fn make_exp_range_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp
            .literal_exp(
                "abcd",
                &Settings {
                    is_optional: true,
                    range: Some((None, Some(2))),
                    ..Default::default()
                },
            )
            .unwrap();
        assert_eq!(result.0, "initial_abcd{,2}?");
    }

    #[test]
    fn make_exp_boundary_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp
            .literal_exp(
                "abcd",
                &Settings {
                    with_left_boundary: true,
                    with_right_boundary: true,
                    ..Default::default()
                },
            )
            .unwrap();
        assert_eq!(result.0, "initial_\\babcd\\b");
    }

    #[test]
    fn make_exp_mixed_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp
            .literal_exp(
                "ab\\scd",
                &Settings {
                    is_optional: true,
                    with_left_boundary: true,
                    with_right_boundary: true,
                    range: Some((None, Some(5))),
                    ..Default::default()
                },
            )
            .unwrap();
        assert_eq!(result.0, "initial_\\bab\\scd{,5}?\\b");
    }

    ////////////////////////////////////////////////// ERRORS /////////////////////////////////////////////////////
    #[test]
    fn make_exp_not_works_with_range_and_start_and_plus() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.literal_exp(
            "ab\\scd",
            &Settings {
                is_nil_or_more: true,
                range: Some((Some(2), None)),
                ..Default::default()
            },
        );
        let err = error_builder("ab\\scd", &ERR_RANGE_AND_STAR_OR_PLUS_EXP);

        assert_eq!(err, result.unwrap_err());
    }

    #[test]
    fn make_exp_not_works_with_star_and_plus() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.literal_exp(
            "ab\\scd",
            &Settings {
                is_nil_or_more: true,
                is_one_or_more: true,
                ..Default::default()
            },
        );
        let err = error_builder("ab\\scd", &ERR_STAR_AND_PLUS_EXP);

        assert_eq!(err, result.unwrap_err());
    }

    #[test]
    fn make_exp_not_works_with_range_and_exact() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.literal_exp(
            "ab\\scd",
            &Settings {
                range: Some((Some(2), Some(10))),
                exactly: Some(8),
                ..Default::default()
            },
        );
        let err = error_builder("ab\\scd", &ERR_RANGE_AND_EXACT_EXP);

        assert_eq!(err, result.unwrap_err());
    }
}
