## Introduction
To make regex-es verbose and clear to write/read, this crate is created.
You can write a long regex with just a few operators and use already-adjusted settings for each segment of the pattern you create.

### Example 1
For instance, to create a regex like
```rust
"/(?i)(?-i:leave me) behind/"
```
one would use this crate as follows:
``` rust
let result = EasyRegex::insensitive()
    .group("leave me", &SENSITIVE_NON_CAPTURE)    // second argument refers to (?i) flag and (?: ...) option which together makes a (?-i: ...) pattern.
    .literal(" behind", &DEFAULT);
```

### Example 2
For a long one:
```rust
"^(http|https|ftp):[/]{2}([a-zA-Z0-9\-\.]+\.[a-zA-Z]{2,4})(:[0-9]+)?/?([a-zA-Z0-9\-\._\?,'/\\\+&amp;%\$#=~]*)"
```
It would be:
```rust
let section_one = EasyRegex::start_of_line()
    .group(r"http|https|ftp", &DEFAULT_GROUP)
    .literal(":", &DEFAULT)
    .list(
        r"/",
        &Settings {
            exactly: Some(2),
            ..Default::default()
        },
    );

let section_two = EasyRegex::new_section()
    .list(r"a-zA-Z0-9\-\.", &ONE_OR_MORE)
    .literal(r"\.", &DEFAULT)
    .list(
        &UPPER_LOWER_CASE,
        &Settings {
            range: Some((Some(2), Some(4))),
            ..Default::default()
        },
    )
    .into_group(&DEFAULT)    // put all previous patterns of "section_two" into a group as (previous patterns)
    .group(":[0-9]+", &OPTIONAL_GROUP)
    .literal(r"/", &OPTIONAL);

let section_three = EasyRegex::new_section()
    .literal(&ALPHA_NUMERIC, &DEFAULT)
    .literal(r"\-\._\?,'/\\\+&amp;%\$#=~", &DEFAULT)
    .into_list(&NIL_OR_MORE)
    .into_group(&DEFAULT);

let result = format!(
    "{}{}{}",
    section_one.get_regex().unwrap(),
    section_two.get_regex().unwrap(),
    section_three.get_regex().unwrap()
);
```