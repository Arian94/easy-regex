use crate::{
    error_handling::*,
    settings_mod::{GroupSettings, Settings},
    MetaFuncRegex,
};

impl MetaFuncRegex {
    pub fn make_group(
        self,
        expression: &str,
        settings: &GroupSettings,
    ) -> Result<MetaFuncRegex, String> {
        let mut final_result = expression.to_string();

        if settings.is_non_capture && settings.is_positive_lookahead {
            let err = error_builder(expression, ERR_NON_CAPTURE_POSITIVE_LOOKAHEAD_GROUP);
            Err(err)
        } else if settings.is_non_capture && settings.is_negative_lookahead {
            let err = error_builder(expression, ERR_NONE_CAPTURE_NEGATIVE_LOOKAHEAD_GROUP);
            Err(err)
        } else if settings.is_positive_lookahead && settings.is_negative_lookahead {
            let err = error_builder(expression, ERR_POSITIVE_NEGATIVE_LOOKAHEAD_GROUP);
            Err(err)
        } else if settings.is_positive_lookahead && settings.other.is_optional {
            let err = error_builder(expression, ERR_OPTIONAL_POSITIVE_LOOKAHEAD_GROUP);
            Err(err)
        } else if settings.is_negative_lookahead && settings.other.is_optional {
            let err = error_builder(expression, ERR_OPTIONAL_NEGATIVE_LOOKAHEAD_GROUP);
            Err(err)
        } else {
            if settings.is_non_capture {
                final_result = format!("{}{}{}", "(?:", final_result, ")");
            } else if settings.is_positive_lookahead {
                final_result = format!("{}{}{}", "(?=", final_result, ")");
            } else if settings.is_negative_lookahead {
                final_result = format!("{}{}{}", "(?!", final_result, ")");
            } else {
                final_result = format!("{}{}{}", "(", final_result, ")");
            }

            let final_result = self.make_literal_exp(&final_result, &settings.other);
            if let Ok(lit_exp) = final_result {
                Ok(lit_exp)
            } else {
                Err(final_result.unwrap_err())
            }
        }
    }

    pub fn into_group(self, settings: &Settings) -> Result<MetaFuncRegex, String> {
        let raw_result = format!("({})", self.0);
        let final_result = MetaFuncRegex("".to_string()).make_literal_exp(&raw_result, &settings);
        if let Ok(lit_exp) = final_result {
            Ok(lit_exp)
        } else {
            Err(final_result.unwrap_err())
        }
    }

    pub fn into_non_capture_group(mut self) -> MetaFuncRegex {
        self.0 = format!("(?:{})", self.0);
        self
    }

    pub fn into_positive_group(mut self) -> MetaFuncRegex {
        self.0 = format!("(?={})", self.0);
        self
    }

    pub fn into_negative_group(mut self) -> MetaFuncRegex {
        self.0 = format!("(?!{})", self.0);
        self
    }
}

#[cfg(test)]
mod tests {
    use self::MetaFuncRegex;
    use super::*;
    use crate::settings_mod::Settings;

    #[test]
    fn make_group_works() {
        let initial_exp = MetaFuncRegex::new("initial_".to_string());
        let result = initial_exp.make_group("group", &GroupSettings::default());
        assert_eq!("initial_(group)", result.unwrap().0);
    }

    #[test]
    fn make_optional_non_capture_group_works() {
        let initial_exp = MetaFuncRegex::new("^".to_string());
        let group_settings = GroupSettings {
            other: Settings {
                is_optional: true,
                ..Default::default()
            },
            is_non_capture: true,
            is_positive_lookahead: false,
            is_negative_lookahead: false,
        };

        let result = initial_exp.make_group("group", &group_settings);
        assert_eq!("^(?:group)?", result.unwrap().0);
    }

    #[test]
    fn into_group_works() {
        let initial_exp = MetaFuncRegex::new("group".to_string());
        let result = initial_exp.into_group(&Settings::default());

        assert_eq!("(group)", result.unwrap().0);
    }

    #[test]
    fn into_negative_group_works_with_bug() {
        let initial_exp = MetaFuncRegex::new("group".to_string());
        let result = initial_exp.into_negative_group().make_literal_exp(
            "",
            &Settings {
                is_optional: true,
                ..Default::default()
            },
        );

        assert_eq!("(?!group)?", result.unwrap().0); // this is invalid regex, a false positive.
    }

    ////////////////////////////////////////////////// ERRORS /////////////////////////////////////////////////////
    #[test]
    fn make_optional_negative_group_not_works() {
        let initial_exp = MetaFuncRegex::new("^".to_string());
        let group_settings = GroupSettings {
            other: Settings {
                is_optional: true,
                ..Default::default()
            },
            is_non_capture: false,
            is_positive_lookahead: false,
            is_negative_lookahead: true,
        };

        let result = initial_exp.make_group("group", &group_settings);
        let err = error_builder("group", ERR_OPTIONAL_NEGATIVE_LOOKAHEAD_GROUP);
        assert_eq!(err, result.unwrap_err());
    }
}
