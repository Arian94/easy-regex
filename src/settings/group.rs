//! Collection of static Group Settings.
//! 
//! These are adjusted **group settings** to be used in methods like [`group`](../../struct.EasyRegex.html#method.group) to save time and make the code more readable.

use crate::settings::{Flags, GroupSettings, Settings};

lazy_static! {
    pub static ref DEFAULT_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
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
        },
    };
    pub static ref OPTIONAL_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
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
        },
    };
    pub static ref OPTIONAL_GROUP_UNGREEDY: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
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
        },
    };
    pub static ref NON_CAPTURE: GroupSettings = GroupSettings {
        is_non_capture: true,
        other: Settings {
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
        },
    };
    pub static ref INSENSITIVE_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
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
        },
    };
    pub static ref MULTILINE_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
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
        },
    };
    pub static ref DOT_MATCH_NEWLINE_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
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
        },
    };
    pub static ref IGNORE_WHITESPACE_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
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
        },
    };
    pub static ref INSENSITIVE_NON_CAPTURE: GroupSettings = GroupSettings {
        is_non_capture: true,
        other: Settings {
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
        },
    };
    pub static ref MULTILINE_NON_CAPTURE: GroupSettings = GroupSettings {
        is_non_capture: true,
        other: Settings {
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
        },
    };
    pub static ref DOT_MATCH_NEWLINE_NON_CAPTURE: GroupSettings = GroupSettings {
        is_non_capture: true,
        other: Settings {
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
        },
    };
    pub static ref IGNORE_WHITESPACE_NON_CAPTURE: GroupSettings = GroupSettings {
        is_non_capture: true,
        other: Settings {
            is_optional: false,
            is_optional_ungreedy: false,
            is_one_or_more: false,
            is_nil_or_more: false,
            with_left_boundary: false,
            with_left_non_boundary: false,
            with_right_boundary: false,
            with_right_non_boundary: false,
            range: None,
            exactly: None,
            flags: Some(Flags::IgnoreWhitespace),
        },
    };
    pub static ref SENSITIVE_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
            is_optional: false,
            is_optional_ungreedy: false,
            is_one_or_more: false,
            is_nil_or_more: false,
            with_left_boundary: false,
            with_left_non_boundary: false,
            with_right_boundary: false,
            with_right_non_boundary: false,
            range: None,
            exactly: None,
            flags: Some(Flags::Sensitive),
        },
    };
    pub static ref SINGLE_LINE_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
            is_optional: false,
            is_optional_ungreedy: false,
            is_one_or_more: false,
            is_nil_or_more: false,
            with_left_boundary: false,
            with_left_non_boundary: false,
            with_right_boundary: false,
            with_right_non_boundary: false,
            range: None,
            exactly: None,
            flags: Some(Flags::SingleLine),
        },
    };
    pub static ref DOT_DISMATCH_NEWLINE_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
            is_optional: false,
            is_optional_ungreedy: false,
            is_one_or_more: false,
            is_nil_or_more: false,
            with_left_boundary: false,
            with_left_non_boundary: false,
            with_right_boundary: false,
            with_right_non_boundary: false,
            range: None,
            exactly: None,
            flags: Some(Flags::DotDisMatchNewLine),
        },
    };
    pub static ref INCLUDE_WHITESPACE_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
            is_optional: false,
            is_optional_ungreedy: false,
            is_one_or_more: false,
            is_nil_or_more: false,
            with_left_boundary: false,
            with_left_non_boundary: false,
            with_right_boundary: false,
            with_right_non_boundary: false,
            range: None,
            exactly: None,
            flags: Some(Flags::IncludeWhitespace),
        },
    };
    pub static ref SENSITIVE_NON_CAPTURE: GroupSettings = GroupSettings {
        is_non_capture: true,
        other: Settings {
            is_optional: false,
            is_optional_ungreedy: false,
            is_one_or_more: false,
            is_nil_or_more: false,
            with_left_boundary: false,
            with_left_non_boundary: false,
            with_right_boundary: false,
            with_right_non_boundary: false,
            range: None,
            exactly: None,
            flags: Some(Flags::Sensitive),
        },
    };
    pub static ref SINGLE_LINE_NON_CAPTURE: GroupSettings = GroupSettings {
        is_non_capture: true,
        other: Settings {
            is_optional: false,
            is_optional_ungreedy: false,
            is_one_or_more: false,
            is_nil_or_more: false,
            with_left_boundary: false,
            with_left_non_boundary: false,
            with_right_boundary: false,
            with_right_non_boundary: false,
            range: None,
            exactly: None,
            flags: Some(Flags::SingleLine),
        },
    };
    pub static ref DOT_DISMATCH_NEWLINE_NON_CAPTURE: GroupSettings = GroupSettings {
        is_non_capture: true,
        other: Settings {
            is_optional: false,
            is_optional_ungreedy: false,
            is_one_or_more: false,
            is_nil_or_more: false,
            with_left_boundary: false,
            with_left_non_boundary: false,
            with_right_boundary: false,
            with_right_non_boundary: false,
            range: None,
            exactly: None,
            flags: Some(Flags::DotDisMatchNewLine),
        },
    };
    pub static ref INCLUDE_WHITESPACE_NON_CAPTURE: GroupSettings = GroupSettings {
        is_non_capture: true,
        other: Settings {
            is_optional: false,
            is_optional_ungreedy: false,
            is_one_or_more: false,
            is_nil_or_more: false,
            with_left_boundary: false,
            with_left_non_boundary: false,
            with_right_boundary: false,
            with_right_non_boundary: false,
            range: None,
            exactly: None,
            flags: Some(Flags::IncludeWhitespace),
        },
    };
}
