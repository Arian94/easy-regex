use crate::{settings::*, EasyRegex};

impl EasyRegex {
    /// To create a literal regular expression, this is the method can be used which
    /// takes an expression (a segment of the total pattern) followed
    /// by a set of settings (```Settings``` struct) that will be concatenated/inserted to the expression itself,
    /// outputing the previous pattern as well as this one.
    ///
    /// ### Example:
    ///
    /// ```
    /// use easy_regex::{EasyRegex, settings::base::DEFAULT};
    /// 
    /// let result = EasyRegex::new_section().literal("\\w", &DEFAULT);
    /// assert_eq!("\\w", result.get_regex().unwrap().as_str());
    /// ```
    pub fn literal(self, expression: &str, settings: &Settings) -> EasyRegex {
        let mut final_result = expression.to_string();

        if let Some(flag) = settings.flags {
            let mut stringified_flag = flag.as_str().to_string();
            stringified_flag = format!("({})", stringified_flag);
            stringified_flag.push_str(expression);
            final_result = stringified_flag;
        }
        if settings.with_left_boundary {
            let mut boundary = String::from("\\b");
            boundary.push_str(&final_result);
            final_result = boundary;
        }
        if settings.with_left_non_boundary {
            let mut boundary = String::from("\\B");
            boundary.push_str(&final_result);
            final_result = boundary;
        }
        if settings.range.is_some()
            && (settings.range.unwrap().0.is_some() || settings.range.unwrap().1.is_some())
        {
            let numbers = settings.range.unwrap();

            final_result.push_str("{");

            if let Some(start_range) = numbers.0 {
                final_result.push_str(&start_range.to_string());
            }

            final_result.push_str(",");

            if let Some(end_range) = numbers.1 {
                final_result.push_str(&end_range.to_string());
            }

            final_result.push_str("}");
        }
        if let Some(number) = settings.exactly {
            final_result.push_str("{");
            final_result.push_str(&number.to_string());
            final_result.push_str("}");
        }
        if settings.is_nil_or_more {
            final_result.push_str("*");
        }
        if settings.is_one_or_more {
            final_result.push_str("+");
        }
        if settings.is_optional {
            final_result.push_str("?");
        }
        if settings.is_optional_ungreedy {
            final_result.push_str("??");
        }
        if settings.with_right_boundary {
            final_result.push_str("\\b");
        }
        if settings.with_right_non_boundary {
            final_result.push_str("\\B");
        }

        final_result = format!("{}{}", &self.0, final_result);
        EasyRegex(final_result)
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::base::*;

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
