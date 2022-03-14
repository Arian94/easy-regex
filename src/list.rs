//! Creates a list of expressions.
//! 
//! Two methods are used to create a list. The main one is the [`list`](../struct.EasyRegex.html#method.list) method and the other is
//! the [`into_list`](../struct.EasyRegex.html#method.into_list).
//! They both use the [`literal`](../struct.EasyRegex.html#method.literal) method internally and in some ways are similar 
//! to the [`group`](../struct.EasyRegex.html#method.group) and [`into_group`](../struct.EasyRegex.html#method.into_group) methods.

use crate::{settings::Settings, EasyRegex};

impl EasyRegex {
    /// To create a list, this method can be used which takes an expression (a segment of the total pattern) followed
    /// by a set of settings (```Settings``` struct) that will be concatenated/inserted to the expression itself, outputing the previous pattern as well as this list.
    ///
    /// ### Example:
    ///
    /// ```
    /// use easy_regex::{EasyRegex, settings::base::DEFAULT};

    /// let result = EasyRegex::new_section().list("some_list", &DEFAULT);
    /// assert_eq!("[some_list]", result.get_regex().unwrap().as_str());
    /// ```
    pub fn list(self, expression: &str, settings: &Settings) -> EasyRegex {
        let mut final_result = expression.to_string();
        final_result = format!("[{}]", final_result);

        let final_result = self.literal(&final_result, &settings);
        final_result
    }

    /// Turn the previous expression into a list. It uses ```Settings``` struct for settings parameter.
    ///
    /// ### Example:
    ///
    /// ```
    /// use easy_regex::{EasyRegex, settings::base::OPTIONAL_UNGREEDY};
    ///
    /// let result = EasyRegex::new(r"a-z").into_list(&OPTIONAL_UNGREEDY);
    /// assert_eq!(r"[a-z]??", result.get_regex().unwrap().as_str());
    /// ```
    pub fn into_list(self, settings: &Settings) -> EasyRegex {
        let raw_result = format!("[{}]", self.0);
        let final_result = EasyRegex::new_section().literal(&raw_result, &settings);
        final_result
    }
}

#[cfg(test)]
mod tests {
    use self::EasyRegex;
    use super::*;
    use crate::settings::{base::*, Flags};

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
