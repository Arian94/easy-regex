use meta_functional_regex::{
    settings_mod::{GroupSettings, Settings},
    MetaFuncRegex,
};

#[test]
fn complicated_regex() {
    let start = MetaFuncRegex::new(r#"^"#.to_string());
    let default_settings = &Settings::default();
    let default_group_settings = &GroupSettings::default();

    let section_one = start
        .make_group(r#"http|https|ftp"#, default_group_settings)
        .unwrap()
        .make_literal_exp(":", default_settings)
        .unwrap()
        .make_list(
            r#"\/"#,
            &Settings {
                exact_amount: Some(2),
                ..Default::default()
            },
        )
        .unwrap();

    let section_two = MetaFuncRegex::new_section()
        .make_list(
            r#"a-zA-Z0-9\-\."#,
            &Settings {
                is_one_or_more: true,
                ..Default::default()
            },
        )
        .unwrap()
        .make_literal_exp(r#"\."#, default_settings)
        .unwrap()
        .make_list(
            "a-zA-Z",
            &Settings {
                range: Some((Some(2), Some(4))),
                ..Default::default()
            },
        )
        .unwrap()
        .into_group(default_settings)
        .unwrap()
        .make_group(
            ":[0-9]+",
            &GroupSettings {
                other: Settings {
                    is_optional: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .unwrap()
        .make_literal_exp(
            r#"\/"#,
            &Settings {
                is_optional: true,
                ..Default::default()
            },
        )
        .unwrap();

    let section_three = MetaFuncRegex::new_section()
        .make_literal_exp("a-zA-Z0-9", default_settings)
        .unwrap()
        .make_literal_exp(r#"\-\._\?\,\'\/\\\+&amp;%\$#\=~"#, default_settings)
        .unwrap()
        .into_list(&Settings {is_nil_or_more: true, ..Default::default()}).unwrap()
        .into_group(default_settings)
        .unwrap();

    let result = format!(
        "{}{}{}",
        section_one.get_regex(),
        section_two.get_regex(),
        section_three.get_regex()
    );
    let the_regex = r#"^(http|https|ftp):[\/]{2}([a-zA-Z0-9\-\.]+\.[a-zA-Z]{2,4})(:[0-9]+)?\/?([a-zA-Z0-9\-\._\?\,\'\/\\\+&amp;%\$#\=~]*)"#;

    assert_eq!(result, the_regex);
}
