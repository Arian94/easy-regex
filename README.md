<img alt="build" src="https://github.com/arian94/easy-regex/actions/workflows/rust.yml/badge.svg?logo=rust">
<img alt="docs" src="https://img.shields.io/docsrs/easy-regex?style=plastic" height="20px">
<img alt="tests" src="https://img.shields.io/docsrs/easy-regex?style=plastic&label=tests" height="20px">
<img alt="license" src="https://img.shields.io/github/license/arian94/easy-regex?style=plastic" height="20px">

## Usage
This lightweight crate helps you write long regular expressions as readable as possible
by making important parts verbose and gives you options to facilitate writing/reading your expressions.
It works in combination with the [regex](https://crates.io/crates/regex) crate.

## Introduction
It consists of three main methods and plenty of handy functions. Along with already-adjusted combinations of
flags, special characters and prepared patterns such as Website URL, Date/Time Formats, French, Persian, Chinese Alphabets etc.
you could write *long* regular expressions that are easy to follow and read.

### Main Features
- [Simplify Writing Regular Expressions](#simple-and-long)
- [Regex Collection](#collection)
- [Helper Methods](#helper-methods)

## <a id=simple-and-long>Simplify Writing Regular Expressions</a>
The main functions are *literal*, *list* and *group*. They work together by chaining them, take two arguments, one for an expression,
the other for special characters, flags etc.

### Simple Regex
To create a regex like
```rust
r"(?i)(?-i:Don't capture)\s(me)";
```
one would use the crate as follows:
``` rust
use easy_regex::{EasyRegex, settings::{base:: DEFAULT, group::{DEFAULT_GROUP, SENSITIVE_NON_CAPTURE}}};

let text = "Don't capture ME";  // a text to be matched by our regex.
let result = EasyRegex::insensitive()
    .group("Don't capture", &SENSITIVE_NON_CAPTURE) // SENSITIVE_NON_CAPTURE refers to  
                                                    // (?-i) and (?: ...) options which  
                                                    // together makes the (?-i: ...) pattern.
    .whitespace(&DEFAULT)
    .group(r"me", &DEFAULT_GROUP);

let captured_text = result.clone().get_regex().unwrap()
    .captures(text).unwrap().get(1).unwrap().as_str();

    assert_eq!(r"(?i)(?-i:Don't capture)\s(me)", result.get_regex().unwrap().as_str());
    assert_eq!("ME", captured_text); // insensitive ME
```

### Long Regex
For
```rust
r"^(http|https|ftp):/{2}([a-zA-Z0-9-.]+\.[a-zA-Z]{2,4})(:[0-9]+)?/?([a-zA-Z0-9-._?,'/\\+&amp;%$#=~]*)";
```
it would be:
```rust
use easy_regex::{
    EasyRegex, 
    settings::{Settings, base::*, group::*},
    collection::{ALPHA_NUMERIC, UPPER_LOWER_CASE}
    };

let section_one = EasyRegex::start_of_line()
    .group(r"http|https|ftp", &DEFAULT_GROUP)
    .literal(":", &DEFAULT)
    .literal(r"/", &Settings::exactly(2));

let section_two = EasyRegex::new_section()
    .list(r"a-zA-Z0-9-.", &ONE_OR_MORE)
    .literal(r"\.", &DEFAULT)
    .list(
        &UPPER_LOWER_CASE,
        &Settings::range(Some(2), Some(4))
    )
    .into_group(&DEFAULT)  // put all previous patterns of "section_two" into a group with default options 
                           // i.e. a capturing group like (previous patterns)
    .group(":[0-9]+", &OPTIONAL_GROUP)
    .literal(r"/", &OPTIONAL);

let section_three = EasyRegex::new_section()
    .literal(&ALPHA_NUMERIC, &DEFAULT)
    .literal(r"-._?,'/\\+&amp;%$#=~", &DEFAULT) // special characters need not be scaped
                                                // due to the next method, into_list.
    .into_list(&NIL_OR_MORE)
    .into_group(&DEFAULT);

let collected_sections = format!(
    "{}{}{}",
    section_one.get_regex().unwrap(),
    section_two.get_regex().unwrap(),
    section_three.get_regex().unwrap()
);

let is_result_ok = regex::RegexBuilder::new(&collected_sections).build().is_ok();
assert_eq!(true, is_result_ok);
```

## <a id=collection>Regex Collection</a>
There are some regular expressions for complicated patterns as Website URL, Date/Time formats, Non-English Alphabets and so on. 
Here are some examples.

### Date and Time Regex
``` rust
use easy_regex::{
    EasyRegex, 
    settings::{Settings, group::DEFAULT_GROUP},
    collection::{DATE, TIME_HH_MM_24}
    };

let text = r#"
    Feb 17 2009 5:3am 03/26/1994 8:41 23/7/2030 9:20Pm
    12 Sept 2015 6:14 03-26-1994 02:18 2030/4/27 3:50
"#;
let result = EasyRegex::new_section()
    .group(DATE, &DEFAULT_GROUP)   // will capture any valid format of a date.
    .literal_space()
    .group(TIME_HH_MM_24, &DEFAULT_GROUP);  // will capture hours and minutes in 24-hour clock.
result
    .clone()
    .get_regex()
    .unwrap()
    .captures_iter(text)
    .for_each(|captures| println!("{}", captures.get(0).unwrap().as_str()));
    // The captures will be:
    //   03/26/1994 8:41
    //   12 Sept 2015 6:14
    //   03-26-1994 02:18
    //   2030/4/27 3:50

let matched_patterns_count = result.get_regex().unwrap().captures_iter(text).count();
assert_eq!(4, matched_patterns_count);
```

### French Regex
There are a collection of useful regular expressions for other languages including French.
```rust
use easy_regex::{EasyRegex, collection::FRENCH_ALPHABET, settings::base::ONE_OR_MORE};

let text = "Adélaïde Aurélie";
let result = EasyRegex::new_section().list(&FRENCH_ALPHABET, &ONE_OR_MORE);

let count = result.get_regex().unwrap().captures_iter(text).count();
assert_eq!(2, count);
```

## <a id=helper-methods>Helper Methods</a>
To make life easier, there are methods for creating certain expressions such as HTML Elements 
that can have child elements as well. See [Helpers](https://docs.rs/easy-regex/latest/easy_regex/helpers/index.html).
