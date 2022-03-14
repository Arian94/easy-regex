use crate::EasyRegex;

impl EasyRegex {
    /// Creates an EasyRegex instance starting with the ```^``` pattern, asserts position at start of the string.
    pub fn start_of_line() -> Self {
        EasyRegex("^".to_string())
    }

    /// Creates an EasyRegex instance starting with the ```\A``` pattern.
    pub fn only_the_beginning() -> Self {
        EasyRegex("\\A".to_string())
    }

    /// Adds the ```\b``` pattern, asserts position at a word boundary.
    pub fn word_boundary(self) -> Self {
        let result = format!("{}\\b", self.0);
        EasyRegex(result)
    }

    /// Adds the ```\B``` pattern, asserts position anywhere but NOT at a word boundary.
    pub fn non_word_boundary(self) -> Self {
        let result = format!("{}\\B", self.0);
        EasyRegex(result)
    }

    /// Adds the alternation symbol ```|``` to the expression.
    pub fn or(self) -> Self {
        let result = format!("{}|", self.0);
        EasyRegex(result)
    }

    /// Creates a list having ```^``` at the beginning.
    ///
    /// ### Example
    ///
    /// ```
    /// use easy_regex::EasyRegex;
    ///
    /// let result = EasyRegex::new_section().not("abc");
    /// assert_eq!("[^abc]", result.get_regex().unwrap().as_str());
    /// ```
    pub fn not(self, expression: &str) -> Self {
        let result = format!("{}[^{}]", self.0, expression);
        EasyRegex(result)
    }

    /// Adds the ending pattern ```$```, asserts position at the end of the string.
    pub fn end_of_line(self) -> Self {
        let result = format!("{}$", self.0);
        EasyRegex(result)
    }

    /// Adds the ending pattern ```\z```, asserts position at the end of the text.
    pub fn only_the_end(self) -> Self {
        let result = format!("{}\\z", self.0);
        EasyRegex(result)
    }

    /// Creates an EasyRegex instance starting with the ```(?i)``` flag.
    pub fn insensitive() -> Self {
        EasyRegex("(?i)".to_string())
    }

    /// Creates an EasyRegex instance starting with the ```(?m)``` flag.
    pub fn multiline() -> Self {
        EasyRegex("(?m)".to_string())
    }

    /// Creates an EasyRegex instance starting with the ```(?s)``` flag.
    pub fn dot_match_newline() -> Self {
        EasyRegex("(?s)".to_string())
    }

    /// Creates an EasyRegex instance starting with the ```(?x)``` flag.
    pub fn ignore_whitespace() -> Self {
        EasyRegex("(?x)".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{EasyRegex, settings::base::DEFAULT};

    #[test]
    fn end_of_line_works() {
        let result = EasyRegex::new("abc").end_of_line();
        assert_eq!("abc$", result.0);
    }

    #[test]
    fn or_works() {
        let result = EasyRegex::new_section()
            .literal("abc", &DEFAULT)
            .or()
            .literal("efg", &DEFAULT)
            .into_list(&DEFAULT);
        assert_eq!("[abc|efg]", result.0);
    }
}