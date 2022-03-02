use crate::{error_descriptions::*, literal_exp_mod::Settings, MetaFuncRegex};

pub struct GroupSettings {
    pub other: Settings,
    pub is_non_capture: bool,
    pub is_positive_lookahead: bool,
    pub is_negative_lookahead: bool,
}

impl Default for GroupSettings {
    fn default() -> Self {
        GroupSettings {
            other: Settings::default(),
            is_non_capture: false,
            is_positive_lookahead: false,
            is_negative_lookahead: false,
        }
    }
}

impl MetaFuncRegex {
    pub fn make_group(
        mut self,
        expression: &str,
        settings: &GroupSettings,
    ) -> Result<MetaFuncRegex, &'static str> {
        let mut final_result = expression.to_string();

        if settings.is_non_capture && settings.is_positive_lookahead {
            Err(ERR_NON_CAPTURE_POSITIVE_LOOKAHEAD_GROUP)
        } else if settings.is_non_capture && settings.is_negative_lookahead {
            Err(ERR_NONE_CAPTURE_NEGATIVE_LOOKAHEAD_GROUP)
        } else if settings.is_positive_lookahead && settings.is_negative_lookahead {
            Err(ERR_POSITIVE_NEGATIVE_LOOKAHEAD_GROUP)
        } else if settings.is_positive_lookahead && settings.other.is_optional {
            Err(ERR_OPTIONAL_POSITIVE_LOOKAHEAD_GROUP)
        } else if settings.is_negative_lookahead && settings.other.is_optional {
            Err(ERR_OPTIONAL_NEGATIVE_LOOKAHEAD_GROUP)
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

            self = self
                .make_literal_exp(final_result.as_str(), &settings.other)
                .unwrap();
            Ok(self)
        }
    }

    pub fn into_group(mut self) -> MetaFuncRegex {
        self.0 = format!("({})", self.0);
        self
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
    use super::*;
    use self::MetaFuncRegex;

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
    fn into_group_works_with_bug() {
        let initial_exp = MetaFuncRegex::new("group".to_string());
        let result = initial_exp
        .into_negative_group().make_literal_exp("", &Settings {
            is_optional: true,
            ..Default::default()
        });

        assert_eq!("(?!group)?", result.unwrap().0);   // this is invalid regex, a false positive.
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
        assert_eq!(ERR_OPTIONAL_NEGATIVE_LOOKAHEAD_GROUP, result.unwrap_err());
    }
}
