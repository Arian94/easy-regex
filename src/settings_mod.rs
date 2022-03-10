#[derive(Clone, Copy)]
pub enum Flags {
    Insensitive,
    Multiline,
    DotMatchNewLine,
    IgnoreWhitespace,
    Sensitive,
    SingleLine,
    DotDisMatchNewLine,
    IncludeWhitespace,
}

impl Flags {
    pub fn as_str(&self) -> &'static str {
        match self {
            Flags::Insensitive => "?i",
            Flags::Multiline => "?m",
            Flags::DotMatchNewLine => "?s",
            Flags::IgnoreWhitespace => "?x",
            Flags::Sensitive => "?-i",
            Flags::SingleLine => "?-m",
            Flags::DotDisMatchNewLine => "?-s",
            Flags::IncludeWhitespace => "?-x",
        }
    }
}

pub struct Settings {
    pub is_optional: bool,
    pub is_one_or_more: bool,
    pub is_nil_or_more: bool,
    pub with_left_boundary: bool,
    pub with_left_non_boundary: bool,
    pub with_right_boundary: bool,
    pub with_right_non_boundary: bool,
    pub range: Option<(Option<u8>, Option<u8>)>,
    pub exactly: Option<u8>,
    pub flags: Option<Flags>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            is_optional: false,
            is_one_or_more: false,
            is_nil_or_more: false,
            with_left_boundary: false,
            with_left_non_boundary: false,
            with_right_boundary: false,
            with_right_non_boundary: false,
            range: None,
            exactly: None,
            flags: None,
        }
    }
}

pub struct GroupSettings {
    pub other: Settings,
    pub is_non_capture: bool,
}

impl Default for GroupSettings {
    fn default() -> Self {
        GroupSettings {
            other: Settings::default(),
            is_non_capture: false,
        }
    }
}

lazy_static! {
    pub static ref DEFAULT: Settings = Settings {
        is_optional: false,
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
    pub static ref OPTIONAL: Settings = Settings {
        is_optional: true,
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
        with_left_boundary: false,
        with_left_non_boundary: false,
        with_right_boundary: false,
        with_right_non_boundary: false,
        range: None,
        exactly: None,
        flags: Some(Flags::IncludeWhitespace),
    };
    //******************************************* GROUP ********************************************* */
    pub static ref DEFAULT_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
            is_optional: false,
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
    pub static ref OPTIONAL_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
            is_optional: true,
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
            with_left_boundary: false,
            with_left_non_boundary: false,
            with_right_boundary: false,
            with_right_non_boundary: false,
            range: None,
            exactly: None,
            flags: None
        },
    };
    //******************************* FLAGS ****************************** */
    pub static ref INSENSITIVE_GROUP: GroupSettings = GroupSettings {
        is_non_capture: false,
        other: Settings {
            is_optional: false,
            is_one_or_more: false,
            is_nil_or_more: false,
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
