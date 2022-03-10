use easy_regex::{
    collection::{ALPHA_NUMERIC, UPPER_LOWER_CASE},
    settings_mod::*,
    EasyRegex,
};

#[test]
fn complicated_regex() {
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
    const URL_REGEX: &str = r"^(http|https|ftp):[/]{2}([a-zA-Z0-9\-\.]+\.[a-zA-Z]{2,4})(:[0-9]+)?/?([a-zA-Z0-9\-\._\?,'/\\\+&amp;%\$#=~]*)";

    assert_eq!(result, URL_REGEX);
}

// \b(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.
// (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.
// (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.
// (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b
#[test]
fn capturing_ip_address() {
    let result = EasyRegex::new_section().word_boundary()
        .group("25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?", &DEFAULT_GROUP)
        .literal("\\.", &DEFAULT)
        .group("25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?", &DEFAULT_GROUP)
        .literal("\\.", &DEFAULT)
        .group("25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?", &DEFAULT_GROUP)
        .literal("\\.", &DEFAULT)
        .group("25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?", &DEFAULT_GROUP)
        .word_boundary();

    const IP_REGEX: &str = r"\b(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b";
    assert_eq!(result.get_regex().unwrap().as_str(), IP_REGEX);
}

#[test]
fn customized_regex() {
    let result = EasyRegex::insensitive()
        .group("leave me", &SENSITIVE_NON_CAPTURE)
        .literal(" behind", &DEFAULT);

    assert_eq!(
        // "(?i)(?:(?-i)leave it) behind",
        "(?i)(?-i:leave me) behind",
        result.get_regex().unwrap().as_str()
    );
}
