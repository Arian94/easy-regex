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
    use crate::settings_mod::DEFAULT;

    #[test]
    fn list_works() {
        let initial_exp = EasyRegex::new("initial");
        let result = initial_exp.list("abcd", &DEFAULT);
        assert_eq!(result.0, "initial[abcd]");
    }

    #[test]
    fn list_not_works() {
        let initial_exp = EasyRegex::new("initial");
        let result = initial_exp.list(
            "abcd",
            &Settings {
                is_nil_or_more: true,
                is_one_or_more: true,
                ..Default::default()
            },
        );
//         let re = regex::Regex::new(
//             r"(?x)
//   (?!P<y>\d{4}) # the year
//   -
//   (?!P<m>\d{2}) # the month
//   -
//   (?!P<d>\d{2}) # the day
// ",
//         )
//         .unwrap();
//         let before = "2012-03-14, 2013-01-01 and 2014-07-05";
//         let after = re.replace_all(before, "$m/$d/$y");
//         println!("{}", &after);
        assert_eq!("initial[abcd]*+", result.get_regex().unwrap().as_str());
    }
}
