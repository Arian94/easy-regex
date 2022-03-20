//! Includes methods for starting/ending chain of expressions.
//! 
//! This module includes methods which typically are useful to start or end a regular expression.
//! Methods as [`start_of_line`](../struct.EasyRegex.html#method.start_of_line), [`only_the_beginning`](../struct.EasyRegex.html#method.only_the_beginning)
//! and flag-related methods can only be used as the starting method and
//! the others could be used in the middle or at the end of a regex, although methods like [`end_of_line`](../struct.EasyRegex.html#method.end_of_line) are meaningful 
//! only when called as the final function.

use crate::EasyRegex;

impl EasyRegex {
    /// Creates an EasyRegex instance starting with the ```^``` pattern, asserts position at start of the string.
    pub fn start_of_line() -> Self {
        EasyRegex("^".to_string())
    }

    /// Adds the alternation symbol ```|``` to the expression.
    pub fn or(self) -> Self {
        let result = format!("{}|", self.0);
        EasyRegex(result)
    }

    /// Creates a list having ```^``` at the beginning.
    ///
    /// # Examples
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