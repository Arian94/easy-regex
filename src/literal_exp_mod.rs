use crate::{settings_mod::Settings, EasyRegex};

impl EasyRegex {
    pub fn literal(self, expression: &str, settings: &Settings) -> EasyRegex {
        let mut final_result = expression.to_string();

        if settings.flags.is_some() {
            let flag = settings.flags.unwrap().as_str();
            final_result = format!("({}){}", flag, final_result);
        } 
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
        EasyRegex(final_result)
    }
}

#[cfg(test)]
mod tests {
    use crate::settings_mod::{DEFAULT, OPTIONAL};

    use self::EasyRegex;
    use super::*;

    #[test]
    fn literal_exp_works() {
        let initial_exp = EasyRegex::new("initial_");
        let result = initial_exp.literal("abcd", &DEFAULT);
        assert_eq!(result.0, "initial_abcd");
    }

    #[test]
    fn literal_exp_optional_works() {
        let initial_exp = EasyRegex::new("initial_");
        let result = initial_exp.literal("abcd", &OPTIONAL);
        assert_eq!(result.0, "initial_abcd?");
    }

    #[test]
    fn literal_exp_range_works() {
        let initial_exp = EasyRegex::new("initial_");
        let result = initial_exp.literal(
            "abcd",
            &Settings {
                is_optional: true,
                range: Some((None, Some(2))),
                ..Default::default()
            },
        );
        assert_eq!(result.0, "initial_abcd{,2}?");
    }

    #[test]
    fn literal_exp_boundary_works() {
        let initial_exp = EasyRegex::new("initial_");
        let result = initial_exp.literal(
            "abcd",
            &Settings {
                with_left_boundary: true,
                with_right_boundary: true,
                ..Default::default()
            },
        );
        assert_eq!(result.0, "initial_\\babcd\\b");
    }

    #[test]
    fn literal_exp_mixed_works() {
        let initial_exp = EasyRegex::new("initial_");
        let result = initial_exp.literal(
            "ab\\scd",
            &Settings {
                is_optional: true,
                with_left_boundary: true,
                with_right_boundary: true,
                range: Some((None, Some(5))),
                ..Default::default()
            },
        );
        assert_eq!(result.0, "initial_\\bab\\scd{,5}?\\b");
    }

    #[test]
    fn literal_exp_works_with_range_and_start_and_plus() {
        let initial_exp = EasyRegex::new("initial_");
        let result = initial_exp.literal(
            "ab\\scd",
            &Settings {
                is_nil_or_more: true,
                range: Some((Some(2), None)),
                ..Default::default()
            },
        );

        assert_eq!("initial_ab\\scd{2,}*", result.get_regex().unwrap().as_str());
    }

    #[test]
    fn literal_exp_works_with_star_and_plus() {
        let initial_exp = EasyRegex::new("initial_");
        let result = initial_exp.literal(
            "ab\\scd",
            &Settings {
                is_nil_or_more: true,
                is_one_or_more: true,
                ..Default::default()
            },
        );

        assert_eq!("initial_ab\\scd*+", result.get_regex().unwrap().as_str());
    }

    #[test]
    fn literal_exp_works_with_range_and_exact() {
        let initial_exp = EasyRegex::new("initial_");
        let result = initial_exp.literal(
            "ab\\scd",
            &Settings {
                range: Some((Some(2), Some(10))),
                exactly: Some(8),
                ..Default::default()
            },
        );

        assert_eq!(
            "initial_ab\\scd{2,10}{8}",
            result.get_regex().unwrap().as_str()
        );
    }
}
