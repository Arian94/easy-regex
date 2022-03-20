//! With this crate, there will be no more panic when trying to write/read a long regular expression.
//! 
//! To start writing out a regex with this crate, there are two main options (except the methods explained in the ```head_or_tail``` module),
//! they are the [`new`](struct.EasyRegex.html#method.new) and [`new_section`](struct.EasyRegex.html#method.new_section).
//! The [`new`](struct.EasyRegex.html#method.new) method takes an &str as input and makes a raw expression (no settings involved) and
//! the [`new_section`](struct.EasyRegex.html#method.new_section)
//! method needs no input and basically makes it possible to write the intented regex by method chaining.
//! To take the prepared regex out of the chain, the last method will be [`get_regex`](struct.EasyRegex.html#method.get_regex) 
//! which outputs a Result including a regex of type Regex or an Error.
//! The [`get_regex`](struct.EasyRegex.html#method.get_regex) will in fact use 
//! the [`RegexBuilder::new`](https://docs.rs/regex/latest/regex/struct.RegexSetBuilder.html#method.new)
//! and [`RegexBuilder::build`](https://docs.rs/regex/latest/regex/struct.RegexSetBuilder.html#method.build) methods of
//! the [regex](https://crates.io/crates/regex) crate.

use regex::{Regex, Error};

pub mod literal;
pub mod group;
pub mod list;
pub mod settings;
pub mod collection;
pub mod head_or_tail;
pub mod metacharacters;
pub mod helpers;

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
