use crate::{EasyRegex, settings_mod::DEFAULT};

impl EasyRegex {
    pub fn start_of_line() -> Self {
        EasyRegex("^".to_string())
    }

    pub fn only_the_beginning() -> Self {
        EasyRegex("\\A".to_string())
    }

    pub fn word_boundary(self) -> Self {
        self.literal("\\b", &DEFAULT)
    }

    pub fn non_word_boundary(self) -> Self {
        self.literal("\\B", &DEFAULT)
    }

    pub fn or(self) -> Self {
        let result = format!("{}|", self.0);
        EasyRegex(result)
    }

    pub fn not(self, expression: &str) -> Self {
        let result = format!("{}[^{}]", self.0, expression);
        EasyRegex(result)
    }

    pub fn end_of_line(self) -> Self {
        EasyRegex("$".to_string())
    }

    pub fn only_the_end(self) -> Self {
        EasyRegex("\\z".to_string())
    }

    pub fn insensitive() -> Self {
        EasyRegex("(?i)".to_string())
    }

    pub fn multiline() -> Self {
        EasyRegex("(?m)".to_string())
    }

    pub fn dot_match_newline() -> Self {
        EasyRegex("(?s)".to_string())
    }

    pub fn ignore_whitespace() -> Self {
        EasyRegex("(?x)".to_string())
    }
}