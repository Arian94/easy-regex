use crate::{
    settings_mod::{GroupSettings, Settings, DEFAULT},
    EasyRegex,
};

impl EasyRegex {
    pub fn group(self, expression: &str, group_sttings: &GroupSettings) -> Self {
        let mut final_result = EasyRegex::new_section();

        // to make the regex itself clearer, this extra if condition is added.
        if group_sttings.other.flags.is_some() && group_sttings.is_non_capture {
            final_result.0 = format!(
                "({}:{})",
                group_sttings.other.flags.unwrap().as_str(),
                expression
            );
        } else {
            final_result = final_result
                .literal(
                    expression,
                    &Settings {
                        flags: group_sttings.other.flags,
                        ..Default::default()
                    },
                )
                .into_group(&Settings {
                    flags: None,
                    ..group_sttings.other
                });
            if group_sttings.is_non_capture {
                final_result.0.insert_str(1, "?:");
            }
        }

        self.literal(&final_result.0, &DEFAULT)
    }

    pub fn named_group(self, name: &str, expression: &str, group_settings: &GroupSettings) -> Self{
        let final_result = format!("?P<{}>{}", name, expression);
        self.group(&final_result, &group_settings)
    }

    pub fn into_group(self, settings: &Settings) -> Self {
        let raw_result = format!("({})", self.0);
        let final_result = EasyRegex(String::new()).literal(&raw_result, &settings);
        final_result
    }

    pub fn into_named_group(self, name: &str, settings: &Settings) -> Self {
        let raw_result = format!("(?P<{}>{})", name, self.0);
        let final_result = EasyRegex(String::new()).literal(&raw_result, &settings);
        final_result
    }

    pub fn into_non_capturing(self) -> Self {
        let result = format!("(?:{})", self.0);
        EasyRegex(result)
    }
////////////////////////////////////////////////////////////////
    pub fn into_insensitive_group(self) -> Self {
        let result = format!("((?i){})", self.0);
        EasyRegex(result)
    }

    pub fn into_multline_group(self) -> Self {
        let result = format!("((?m){})", self.0);
        EasyRegex(result)
    }

    pub fn into_dot_match_newline_group(self) -> Self {
        let result = format!("((?s){})", self.0);
        EasyRegex(result)
    }

    pub fn into_ignore_whitespace_group(self) -> Self {
        let result = format!("((?x){})", self.0);
        EasyRegex(result)
    }
////////////////////////////////////////////////////////////////
    pub fn into_insensitive_non_capturing(self) -> Self {
        let result = format!("(?i:{})", self.0);
        EasyRegex(result)
    }

    pub fn into_multiline_non_capturing(self) -> Self {
        let result = format!("(?m:{})", self.0);
        EasyRegex(result)
    }

    pub fn into_dot_match_newline_non_capturing(self) -> Self {
        let result = format!("(?s:{})", self.0);
        EasyRegex(result)
    }

    pub fn into_ignore_whitespace_non_capturing(self) -> Self {
        let result = format!("(?x:{})", self.0);
        EasyRegex(result)
    }
////////////////////////////////////////////////////////////////
    pub fn into_sensitive_group(self) -> Self {
        let result = format!("(?-i){}", self.0);
        EasyRegex(result)
    }
    pub fn into_single_line_group(self) -> Self {
        let result = format!("(?-m){}", self.0);
        EasyRegex(result)
    }

    pub fn into_dot_dismatch_newline_group(self) -> Self {
        let result = format!("(?-s){}", self.0);
        EasyRegex(result)
    }

    pub fn into_include_whitespace_group(self) -> Self {
        let result = format!("(?-x){}", self.0);
        EasyRegex(result)
    }
////////////////////////////////////////////////////////////////
    pub fn into_sensitive_non_capturing(self) -> Self {
        let result = format!("(?-i:{})", self.0);
        EasyRegex(result)
    }

    pub fn into_single_line_non_capturing(self) -> Self {
        let result = format!("(?-m:{})", self.0);
        EasyRegex(result)
    }

    pub fn into_dot_dismatch_newline_non_capturing(self) -> Self {
        let result = format!("(?-s:{})", self.0);
        EasyRegex(result)
    }

    pub fn into_include_whitespace_non_capturing_group(self) -> Self {
        let result = format!("(?-x:{})", self.0);
        EasyRegex(result)
    }
}

#[cfg(test)]
mod tests {
    use self::EasyRegex;
    use super::*;
    use crate::settings_mod::*;

    #[test]
    fn group_works() {
        let initial_exp = EasyRegex::new("initial_");
        let result = initial_exp.group("group", &DEFAULT_GROUP);
        assert_eq!("initial_(group)", result.0);
    }

    #[test]
    fn optional_non_capture_group_works() {
        let initial_exp = EasyRegex::start_of_line();
        let group_settings = GroupSettings {
            other: Settings {
                is_optional: true,
                ..Default::default()
            },
            is_non_capture: true,
        };

        let result = initial_exp.group("group", &group_settings);
        assert_eq!("^(?:group)?", result.0);
    }

    #[test]
    fn insensitive_group_works() {
        let result = EasyRegex::start_of_line()
            .group("group", &INSENSITIVE_GROUP)
            .get_regex()
            .unwrap();
        assert_eq!("^((?i)group)", result.as_str());
    }

    #[test]
    fn insensitive_non_capturing_group_works() {
        let result = EasyRegex::start_of_line()
            .group("group", &INSENSITIVE_NON_CAPTURE)
            .get_regex()
            .unwrap();
        assert_eq!("^(?i:group)", result.as_str());
    }

    #[test]
    fn into_group_works() {
        let initial_exp = EasyRegex::new("group");
        let result = initial_exp.into_group(&DEFAULT);

        assert_eq!("(group)", result.0);
    }

    ////////////////////////////////////////////////// ERRORS /////////////////////////////////////////////////////
    // #[test]
    //     fn into_negative_group_added_optional_exp_not_works() {
    //         let initial_exp = MetaFuncRegex::new("group");
    //         let result = initial_exp
    //             // .into_negative_group()
    //             .literal_exp(&String::new(), &OPTIONAL);
    //         let err = result.get_regex().unwrap_err();
    //         let re = regex::Regex::new("/(?!group)/").unwrap();
    //         // regex::Regex::is_matchbuild(&re).unwrap();
    //         // println!("{}", &after);
    //         assert_eq!(
    //             regex::Error::Syntax(
    //                 "regex parse error:
    //     (?!group)?
    //     ^^^
    // error: look-around, including look-ahead and look-behind, is not supported"
    //                     .to_string()
    //             ),
    //             err
    //         );
    //     }

    // #[test]
    //     fn optional_negative_group_not_works() {
    //         let initial_exp = MetaFuncRegex::new("^");
    //         let group_settings = GroupSettings {
    //             other: Settings {
    //                 is_optional: true,
    //                 ..Default::default()
    //             },
    //             is_non_capture: false,
    //             flags: None,
    //         };

    //         let result = initial_exp.group("group", &group_settings);
    //         let err = result.get_regex().unwrap_err();
    //         assert_eq!(
    //             regex::Error::Syntax(
    //                 "regex parse error:
    //     ^(?!group)?
    //      ^^^
    // error: look-around, including look-ahead and look-behind, is not supported"
    //                     .to_string()
    //             ),
    //             err
    //         );
    //     }
}
