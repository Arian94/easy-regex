To make regex-es verbose and clear to write/read, this crate is created.
You can write a long regex with just a few operators and set already adjusted settings for each segment of the pattern you create.
For instance, to create a regex like
```
    (?i)(?-i:leave me) behind
```
one would use this crate as follows:
```
    let result = EasyRegex::insensitive()
            .group("leave me", &SENSITIVE_NON_CAPTURE)
            .literal(" behind", &DEFAULT);
```
For a long one:
```
    ^(http|https|ftp):[/]{2}([a-zA-Z0-9\-\.]+\.[a-zA-Z]{2,4})(:[0-9]+)?/?([a-zA-Z0-9\-\._\?,'/\\\+&amp;%\$#=~]*)
```
It would be:
```
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
        .into_group(&DEFAULT)
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