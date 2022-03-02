use crate::{error_descriptions::*, MetaFuncRegex};

pub struct Settings {
    pub is_optional: bool,
    pub one_or_more: bool,
    pub nil_or_more: bool,
    pub with_left_boundary: bool,
    pub with_right_boundary: bool,
    pub range: Option<(Option<u8>, Option<u8>)>,
    pub exact_amount: Option<i8>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            is_optional: false,
            one_or_more: false,
            nil_or_more: false,
            with_left_boundary: false,
            with_right_boundary: false,
            range: None,
            exact_amount: None,
        }
    }
}

impl MetaFuncRegex {
    pub fn new(raw: String) -> Self {
        MetaFuncRegex(raw)
    }

    pub fn make_literal_exp(
        mut self,
        expression: &str,
        settings: &Settings,
    ) -> Result<MetaFuncRegex, &'static str> {
        let mut final_result = expression.to_string();

        if settings.range.is_some() && (settings.nil_or_more || settings.one_or_more) {
            Err(ERR_RANGE_AND_STAR_OR_PLUS_EXP)
        } else if settings.nil_or_more && settings.one_or_more {
            Err(ERR_STAR_AND_PLUS_EXP)
        } else if settings.exact_amount.is_some() && settings.range.is_some() {
            Err(ERR_RANGE_AND_EXACT_EXP)
        } else {
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
            if settings.nil_or_more {
                final_result = format!("{}{}", final_result, "*");
            }
            if settings.one_or_more {
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
    use super::*;
    use self::MetaFuncRegex;

    #[test]
    fn make_exp_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.make_literal_exp(
            "abcd",
            &Settings::default(),
        )
        .unwrap();
        assert_eq!(result.0, "initial_abcd");
    }

    #[test]
    fn make_exp_optional_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.make_literal_exp(
            "abcd",
            &Settings {
                is_optional: true,
                nil_or_more: false,
                one_or_more: false,
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
        let result = initial_exp.make_literal_exp(
            "abcd",
            &Settings {
                is_optional: true,
                nil_or_more: false,
                one_or_more: false,
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
        let result = initial_exp.make_literal_exp(
            "abcd",
            &Settings {
                is_optional: false,
                nil_or_more: false,
                one_or_more: false,
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
        let result = initial_exp.make_literal_exp(
            "ab\\scd",
            &Settings {
                is_optional: true,
                nil_or_more: false,
                one_or_more: false,
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
                nil_or_more: true,
                one_or_more: false,
                with_left_boundary: false,
                with_right_boundary: false,
                range: Some((Some(2), None)),
                exact_amount: None,
            },
        );
        assert_eq!(ERR_RANGE_AND_STAR_OR_PLUS_EXP, result.unwrap_err());
    }

    #[test]
    fn make_exp_not_works_with_star_and_plus() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.make_literal_exp(
            "ab\\scd",
            &Settings {
                is_optional: false,
                nil_or_more: true,
                one_or_more: true,
                with_left_boundary: false,
                with_right_boundary: false,
                range: None,
                exact_amount: None,
            },
        );
        assert_eq!(ERR_STAR_AND_PLUS_EXP, result.unwrap_err());
    }

    #[test]
    fn make_exp_not_works_with_range_and_exact() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.make_literal_exp(
            "ab\\scd",
            &Settings {
                is_optional: false,
                nil_or_more: false,
                one_or_more: false,
                with_left_boundary: false,
                with_right_boundary: false,
                range: Some((Some(2), Some(10))),
                exact_amount: Some(8),
            },
        );
        assert_eq!(ERR_RANGE_AND_EXACT_EXP, result.unwrap_err());
    }
}