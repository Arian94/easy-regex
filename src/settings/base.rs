use crate::settings::{Settings, Flags};

lazy_static! {
    pub static ref DEFAULT: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref OPTIONAL: Settings = Settings {
        is_optional: true,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref OPTIONAL_UNGREEDY: Settings = Settings {
        is_optional: false,
        is_optional_ungreedy: true,
        is_one_or_more: false,
        is_nil_or_more: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref ONE_OR_MORE: Settings = Settings {
        is_optional: false,
        is_one_or_more: true,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref ONE_OR_MORE_UNGREEDY: Settings = Settings {
        is_optional: true,
        is_one_or_more: true,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref NIL_OR_MORE: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: true,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref NIL_OR_MORE_UNGREEDY: Settings = Settings {
        is_optional: true,
        is_one_or_more: false,
        is_nil_or_more: true,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref LEFT_BOUNDARY: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: true,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref RIGHT_BOUNDARY: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: true,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref BOTH_BOUNDARY: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: true,
        with_left_non_boundary: false,
        with_right_boundary: true,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref LEFT_NON_BOUNDARY: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: true,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref RIGHT_NON_BOUNDARY: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: true,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref BOTH_NON_BOUNDARY: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: true,
        with_right_boundary: false,
        with_right_non_boundary: true,
        range: None,
        exactly: None,
        flags: None
    };
    pub static ref INSENSITIVE: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: Some(Flags::Insensitive),
    };
    pub static ref MULTILINE: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: Some(Flags::Multiline),
    };
    pub static ref DOT_MATCH_NEWLINE: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: Some(Flags::DotMatchNewLine),
    };
    pub static ref IGNORE_WHITESPACE: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: Some(Flags::IgnoreWhitespace),
    };
    pub static ref SENSITIVE: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: Some(Flags::Sensitive),
    };
    pub static ref SINGLE_LINE: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: Some(Flags::SingleLine),
    };
    pub static ref DOT_DISMATCH_NEWLINE: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: Some(Flags::DotDisMatchNewLine),
    };
    pub static ref INCLUDE_WHITESPACE: Settings = Settings {
        is_optional: false,
        is_one_or_more: false,
        is_nil_or_more: false,
        is_optional_ungreedy: false,
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: Some(Flags::IncludeWhitespace),
    };
}
