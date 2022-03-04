use crate::{settings_mod::Settings, MetaFuncRegex};

impl MetaFuncRegex {
    pub fn list(
        self,
        expression: &str,
        settings: &Settings,
    ) -> Result<MetaFuncRegex, String> {
        let mut final_result = expression.to_string();
        final_result = format!("[{}]", final_result);

        let final_result = self.literal_exp(&final_result, &settings);
        if let Ok(lit_exp) = final_result {
            Ok(lit_exp)
        } else {
            Err(final_result.unwrap_err())
        }
    }

    pub fn into_list(self, settings: &Settings) -> Result<MetaFuncRegex, String> {
        let raw_result = format!("[{}]", self.0);
        let final_result = MetaFuncRegex("".to_string()).literal_exp(&raw_result, &settings);
        if let Ok(lit_exp) = final_result {
            Ok(lit_exp)
        } else {
            Err(final_result.unwrap_err())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::MetaFuncRegex;
    use crate::{error_handling::*, settings_mod::NO_SETTINGS};

    #[test]
    fn make_list_works() {
        let initial_exp = MetaFuncRegex::new("initial".to_string());
        let result = initial_exp.list("abcd", &NO_SETTINGS).unwrap();
        assert_eq!(result.0, "initial[abcd]");
    }

    #[test]
    fn make_list_not_works() {
        let initial_exp = MetaFuncRegex::new("initial".to_string());
        let result = initial_exp.list("abcd", &Settings {
            is_nil_or_more: true,
            is_one_or_more: true,
            ..Default::default()
        });
        let err = error_builder("[abcd]", ERR_STAR_AND_PLUS_EXP);

        assert_eq!(err, result.unwrap_err());
    }
}
