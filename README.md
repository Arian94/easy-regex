<img alt="github" src="https://img.shields.io/github/license/arian94/easy-regex?style=plastic" height="20px">
<img alt="build" src="https://github.com/arian94/easy-regex/actions/workflows/rust.yml/badge.svg?logo=rust">
<img alt="docs" src="https://img.shields.io/docsrs/easy-regex?style=plastic" height="20px">
<img alt="docs" src="https://img.shields.io/github/commit-activity/m/arian94/easy-regex?style=plastic" height="20px">

## Usage
To force writing long regular expressions as readable as possible by making it verbose and clear to write/read, this lightweight crate has been created.

## Introduction
It consists of few main methods and plenty of handy functions. You can write a long regex using them along with already-adjusted settings and prepared patterns for **English, French, German, Persian, Chinese and Arabic** languages. 

### Example 1 - Simple Regex
To create a regex like
```rust
r"(?i)(?-i:Don't capture)\s(me)";
```
one would use this crate as follows:
``` rust
use easy_regex::{EasyRegex, settings::{base:: DEFAULT, group::{DEFAULT_GROUP, SENSITIVE_NON_CAPTURE}}};

let text = "Don't capture ME";  // a text to be matched by our regex.
let result = EasyRegex::insensitive()
    .group("Don't capture", &SENSITIVE_NON_CAPTURE) // SENSITIVE_NON_CAPTURE refers to  
                                                    // (?-i) and (?: ...) options which  
                                                    // together makes the (?-i: ...) pattern.
    .literal(r"\s", &DEFAULT)
    .group(r"me", &DEFAULT_GROUP);

let mut captured_text = result.clone().get_regex().unwrap()
    .captures(text).unwrap().get(1).unwrap().as_str();

    assert_eq!(r"(?i)(?-i:Don't capture)\s(me)", result.get_regex().unwrap().as_str());
    assert_eq!("ME", captured_text); // insensitive ME
```

### Example 2 - French Regex
There are a collection of useful regular expressions for other languages including French.
```rust
use easy_regex::{EasyRegex, collection::FRENCH_ALPHABET, settings::base::ONE_OR_MORE};

let text = "Adélaïde Aurélie";
let result = EasyRegex::new_section().list(&FRENCH_ALPHABET, &ONE_OR_MORE);

let count = result.get_regex().unwrap().captures_iter(text).count();
assert_eq!(2, count);
```

### Example 3 - Long Regex
And for a long one:
```rust
r"^(http|https|ftp):[/]{2}([a-zA-Z0-9-.]+\.[a-zA-Z]{2,4})(:[0-9]+)?/?([a-zA-Z0-9-._?,'/\\+&amp;%$#=~]*)";
```
It would be:
```rust
use easy_regex::{
    EasyRegex, 
    settings::{
        Settings,
        base:: {DEFAULT, OPTIONAL, NIL_OR_MORE, ONE_OR_MORE}, 
        group::{DEFAULT_GROUP, OPTIONAL_GROUP, SENSITIVE_NON_CAPTURE}},
    collection::{ALPHA_NUMERIC, UPPER_LOWER_CASE}
    };

let section_one = EasyRegex::start_of_line()
    .group(r"http|https|ftp", &DEFAULT_GROUP)
    .literal(":", &DEFAULT)
    .list(
        r"/",
        &Settings::exactly(2)
    );

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