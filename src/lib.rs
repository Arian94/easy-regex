use regex::{Regex, Error};

pub mod literal_exp_mod;
pub mod group_mod;
pub mod list_mod;
pub mod settings;
pub mod collection;
pub mod head_or_tail_mod;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone)]
pub struct EasyRegex(String);

impl EasyRegex {
    /// Creates an **EasyRegex** instance, having initial raw pattern.
    pub fn new(raw: &str) -> Self {
        EasyRegex(raw.to_string())
    }

    /// Creates an empty **EasyRegex** instance, useful for start of a pattern.
    pub fn new_section() -> Self {
        EasyRegex(String::new())
    }

    /// Retrieves the prepared regular expression as a ```Result```.
    pub fn get_regex(self) -> Result<Regex, Error> {
        let regex = regex::RegexBuilder::new(&self.0);
        regex.build()
    }
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
