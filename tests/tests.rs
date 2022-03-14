use easy_regex::{
    collection::{ALPHA_NUMERIC, UPPER_LOWER_CASE},
    settings::{base::*, group::*, Settings},
    EasyRegex,
};

#[test]
fn complicated_regex() {
    let section_one = EasyRegex::start_of_line()
        .group(r"http|https|ftp", &DEFAULT_GROUP)
        .literal(":", &DEFAULT)
        .list(r"/", &Settings::exactly(2));

    let section_two = EasyRegex::new_section()
        .list(r"a-zA-Z0-9-.", &ONE_OR_MORE)
        .literal(r"\.", &DEFAULT)
        .list(
            &UPPER_LOWER_CASE,
            &Settings::range(Some(2), Some(4)),
        )
        .into_group(&DEFAULT)
        .group(":[0-9]+", &OPTIONAL_GROUP)
        .literal(r"/", &OPTIONAL);

    let section_three = EasyRegex::new_section()
        .literal(&ALPHA_NUMERIC, &DEFAULT)
        .literal(r"-._?,'/\\+&amp;%$#=~", &DEFAULT)
        .into_list(&NIL_OR_MORE)
        .into_group(&DEFAULT);

    let result = format!(
        "{}{}{}",
        section_one.get_regex().unwrap(),
        section_two.get_regex().unwrap(),
        section_three.get_regex().unwrap()
    );
    const URL_REGEX: &str = r"^(http|https|ftp):[/]{2}([a-zA-Z0-9-.]+\.[a-zA-Z]{2,4})(:[0-9]+)?/?([a-zA-Z0-9-._?,'/\\+&amp;%$#=~]*)";
    assert_eq!(result, URL_REGEX);
}

// \b(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.
// (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.
// (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.
// (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b
#[test]
fn capturing_ip_address() {
    let result = EasyRegex::new_section()
        .word_boundary()
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
        .group("leave it", &SENSITIVE_NON_CAPTURE)
        .literal(" behind", &DEFAULT);

    assert_eq!(
        "(?i)(?-i:leave it) behind",
        result.get_regex().unwrap().as_str()
    );
}

#[test]
fn customized_regex_with_named_groups() {
    let result = EasyRegex::start_of_line().named_group("middle_name", "[a-z]", &OPTIONAL_GROUP);
    assert_eq!(
        "^(?P<middle_name>[a-z])?",
        result.get_regex().unwrap().as_str()
    );
}
