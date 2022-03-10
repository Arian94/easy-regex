use crate::{settings_mod::Settings, EasyRegex};

impl EasyRegex {
    pub fn list(self, expression: &str, settings: &Settings) -> EasyRegex {
        let mut final_result = expression.to_string();
        final_result = format!("[{}]", final_result);

        let final_result = self.literal(&final_result, &settings);
        final_result
    }

    pub fn into_list(self, settings: &Settings) -> EasyRegex {
        let raw_result = format!("[{}]", self.0);
        let final_result = EasyRegex("".to_string()).literal(&raw_result, &settings);
        final_result
    }
}

#[cfg(test)]
mod tests {
    use self::EasyRegex;
    use super::*;
    use crate::settings_mod::{DEFAULT, Flags};

    #[test]
    fn list_works() {
        let initial_exp = EasyRegex::new("initial");
        let result = initial_exp.list("abcd", &DEFAULT);
        assert_eq!(result.0, "initial[abcd]");
    }

    #[test]
    fn another_list_works() {
        let initial_exp = EasyRegex::new("initial");
        let result = initial_exp.list(
            "abcd",
            &Settings {
                is_nil_or_more: true,
                is_one_or_more: true,
                ..Default::default()
            },
        );
        assert_eq!("initial[abcd]*+", result.get_regex().unwrap().as_str());
    }

    #[test]
    fn list_with_flag_and_settings_works() {
        let result = EasyRegex::new_section().list(
            "list",
            &Settings {
                range: Some((Some(2), None)),
                flags: Some(Flags::Insensitive),
                ..Default::default()
            },
        );

        assert_eq!("(?i)[list]{2,}", result.0);
    }
}
