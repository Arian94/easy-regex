//! Methods related to metacharacters.

use crate::{settings::Settings, EasyRegex};

impl EasyRegex {
    /// Creates an EasyRegex instance starting with the ```\A``` pattern.
    pub fn only_the_beginning() -> Self {
        EasyRegex("\\A".to_string())
    }

    /// Adds the ```\b``` metacharacter, asserts position at a word boundary.
    pub fn word_boundary(self) -> Self {
        let result = format!("{}\\b", self.0);
        EasyRegex(result)
    }

    /// Adds the ```\w``` metacharacter, matches any word character [a-zA-Z0-9_].
    pub fn word(self, settings: Settings) -> Self {
        let result = self.literal("\\w", &settings);
        result
    }

    /// Adds the ```\w``` metacharacter, matches any non-word character [^a-zA-Z0-9_].
    pub fn non_word(self, settings: Settings) -> Self {
        let result = self.literal("\\W", &settings);
        result
    }

    /// Adds the ```\d``` metacharacter, matches digit character [0-9].
    pub fn digit(self, settings: Settings) -> Self {
        let result = self.literal("\\d", &settings);
        result
    }

    /// Adds the ```\D``` metacharacter, matches any non-digit character [^0-9].
    pub fn non_digit(self, settings: Settings) -> Self {
        let result = self.literal("\\D", &settings);
        result
    }

    /// Adds the ```\s``` metacharacter, matches whitespace character [\r\n\t\f\v ].
    pub fn whitespace(self, settings: Settings) -> Self {
        let result = self.literal("\\s", &settings);
        result
    }

    /// Adds the ```\S``` metacharacter, matches any non-whitespace character [^\r\n\t\f\v ].
    pub fn non_whitespace(self, settings: Settings) -> Self {
        let result = self.literal("\\S", &settings);
        result
    }

    /// Adds the ```\B``` metacharacter, asserts position anywhere but NOT at a word boundary.
    pub fn non_word_boundary(self) -> Self {
        let result = format!("{}\\B", self.0);
        EasyRegex(result)
    }

    /// Adds the ending metacharacter ```\z```, asserts position at the end of the text.
    pub fn only_the_end(self) -> Self {
        let result = format!("{}\\z", self.0);
        EasyRegex(result)
    }
}
