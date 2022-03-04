use meta_functional_regex::{settings_mod::*, MetaFuncRegex};

#[test]
fn complicated_regex() {
    let start = MetaFuncRegex::new(r"^".to_string());

    let section_one = start
        .group(r"http|https|ftp", &NO_GROUP_SETTINGS)
        .unwrap()
        .literal_exp(":", &NO_SETTINGS)
        .unwrap()
        .list(
            r"\/",
            &Settings {
                exactly: Some(2),
                ..Default::default()
            },
        )
        .unwrap();

    let section_two = MetaFuncRegex::new_section()
        .list(r"a-zA-Z0-9\-\.", &ONE_OR_MORE)
        .unwrap()
        .literal_exp(r"\.", &NO_SETTINGS)
        .unwrap()
        .list(
            "a-zA-Z",
            &Settings {
                range: Some((Some(2), Some(4))),
                ..Default::default()
            },
        )
        .unwrap()
        .into_group(&NO_SETTINGS)
        .unwrap()
        .group(":[0-9]+", &OPTIONAL_GROUP)
        .unwrap()
        .literal_exp(r"\/", &OPTIONAL)
        .unwrap();

    let section_three = MetaFuncRegex::new_section()
        .literal_exp("a-zA-Z0-9", &NO_SETTINGS)
        .unwrap()
        .literal_exp(r"\-\._\?\,\'\/\\\+&amp;%\$#\=~", &NO_SETTINGS)
        .unwrap()
        .into_list(&NIL_OR_MORE)
        .unwrap()
        .into_group(&NO_SETTINGS)
        .unwrap();

    let result = format!(
        "{}{}{}",
        section_one.get_regex(),
        section_two.get_regex(),
        section_three.get_regex()
    );
    let the_regex = r"^(http|https|ftp):[\/]{2}([a-zA-Z0-9\-\.]+\.[a-zA-Z]{2,4})(:[0-9]+)?\/?([a-zA-Z0-9\-\._\?\,\'\/\\\+&amp;%\$#\=~]*)";

    assert_eq!(result, the_regex);
}
