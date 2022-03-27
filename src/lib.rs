//! This crate implements another way of writing regualar expressions as if they are pseudocodes,
//! they will be easier to understand and debug, especially the long ones.
//! 
//! To start writing a regex using this crate, there are several ways (see [`head_or_tail`](head_or_tail)) of which 
//! [`new`](struct.EasyRegex.html#method.new) and [`new_section`](struct.EasyRegex.html#method.new_section) are the common methods.
//! The [`new`](struct.EasyRegex.html#method.new) method takes an ```&str``` as input and makes a raw expression while
//! the [`new_section`](struct.EasyRegex.html#method.new_section)
//! method needs no input and basically creates an empty string to write the intented expressions by method chaining.
//! To take the prepared regex out of the chain, the last method will be [`get_regex`](struct.EasyRegex.html#method.get_regex) 
//! which outputs a ```Result``` including a regex of type ```Regex``` or an ```Error```.
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
/// Main struct includes methods to be chained together in order to create a regular expression.
pub struct EasyRegex(String);

impl EasyRegex {
    /// Creates an ```EasyRegex``` instance, having initial raw pattern.
    pub fn new(raw: &str) -> Self {
        EasyRegex(raw.to_string())
    }

    /// Creates an empty ```EasyRegex``` instance, useful for start of a pattern.
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
