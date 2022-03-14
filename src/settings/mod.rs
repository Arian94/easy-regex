pub mod base;
pub mod group;

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
    /// Converts an enum flag into its equivalent meaning.
    /// 
    /// ### Example
    /// ```
    /// use easy_regex::settings::Flags;
    /// 
    /// println!("{}", Flags::Insensitive.as_str()); // outputs ?i
    /// ```
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
    pub is_optional_ungreedy: bool,
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
            is_optional_ungreedy: false,
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

impl Settings {
    pub fn exactly(number: u8) -> Self {
        Settings {
            exactly: Some(number),
            ..Default::default()
        }
    }

    pub fn range(from: Option<u8>, to: Option<u8>) -> Self {
        Settings {
            range: Some((from, to)),
            ..Default::default()
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

impl GroupSettings {
    pub fn grp_exactly(number: u8) -> Self {
        GroupSettings {
            other: Settings {
                exactly: Some(number),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn grp_range(from: Option<u8>, to: Option<u8>) -> Self {
        GroupSettings {
            other: Settings {
                range: Some((from, to)),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
