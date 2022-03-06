use meta_functional_regex::{settings_mod::*, MetaFuncRegex, collection::{UPPER_LOWER_CASE, ALPHA_NUMERIC}};

#[test]
fn complicated_regex() {
    let section_one = MetaFuncRegex::start_of_line()
        .group(r"http|https|ftp", &NO_GROUP_SETTINGS).unwrap()
        .literal_exp(":", &NO_SETTINGS).unwrap()
        .list(r"\/", &Settings {exactly: Some(2), ..Default::default()}).unwrap();

    let section_two = MetaFuncRegex::new_section()
        .list(r"a-zA-Z0-9\-\.", &ONE_OR_MORE).unwrap()
        .literal_exp(r"\.", &NO_SETTINGS).unwrap()
        .list(&UPPER_LOWER_CASE, &Settings {range: Some((Some(2), Some(4))),..Default::default()}).unwrap()
        .into_group(&NO_SETTINGS).unwrap()
        .group(":[0-9]+", &OPTIONAL_GROUP).unwrap()
        .literal_exp(r"\/", &OPTIONAL).unwrap();

    let section_three = MetaFuncRegex::new_section()
        .literal_exp(&ALPHA_NUMERIC, &NO_SETTINGS).unwrap()
        .literal_exp(r"\-\._\?\,\'\/\\\+&amp;%\$#\=~", &NO_SETTINGS).unwrap()
        .into_list(&NIL_OR_MORE).unwrap()
        .into_group(&NO_SETTINGS).unwrap();

    let result = format!(
        "{}{}{}",
        section_one.get_regex(),
        section_two.get_regex(),
        section_three.get_regex()
    );
    const URL_REGEX: &str = r"^(http|https|ftp):[\/]{2}([a-zA-Z0-9\-\.]+\.[a-zA-Z]{2,4})(:[0-9]+)?\/?([a-zA-Z0-9\-\._\?\,\'\/\\\+&amp;%\$#\=~]*)";

    assert_eq!(result, URL_REGEX);
}

// \b(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.
// (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.
// (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.
// (25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b
#[test]
fn capturing_ip_address() {
    let result = MetaFuncRegex::new("\\b".to_string())
        .group("25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?", &NO_GROUP_SETTINGS).unwrap()
        .literal_exp("\\.", &NO_SETTINGS).unwrap()
        .group("25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?", &NO_GROUP_SETTINGS).unwrap()
        .literal_exp("\\.", &NO_SETTINGS).unwrap()
        .group("25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?", &NO_GROUP_SETTINGS).unwrap()
        .literal_exp("\\.", &NO_SETTINGS).unwrap()
        .group("25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?", &NO_GROUP_SETTINGS).unwrap()
        .literal_exp("\\b", &NO_SETTINGS).unwrap();

    const IP_REGEX: &str = r"\b(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b";
    assert_eq!(result.get_regex(), IP_REGEX);
}
