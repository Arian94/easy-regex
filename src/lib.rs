use regex::{Regex, Error};

pub mod literal_exp_mod;
pub mod group_mod;
pub mod list_mod;
pub mod settings_mod;
pub mod collection;
pub mod head_or_tail_mod;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone)]
pub struct EasyRegex(String);

impl EasyRegex {
    pub fn new(raw: &str) -> Self {
        EasyRegex(raw.to_string())
    }

    pub fn new_section() -> Self {
        EasyRegex(String::new())
    }

    pub fn get_regex(self) -> Result<Regex, Error> {
        let regex = regex::RegexBuilder::new(&self.0);
        regex.build()
    }
}