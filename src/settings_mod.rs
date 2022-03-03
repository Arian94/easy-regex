pub struct Settings {
    pub is_optional: bool,
    pub is_one_or_more: bool,
    pub is_nil_or_more: bool,
    pub with_left_boundary: bool,
    pub with_right_boundary: bool,
    pub range: Option<(Option<u8>, Option<u8>)>,
    pub exact_amount: Option<u8>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            is_optional: false,
            is_one_or_more: false,
            is_nil_or_more: false,
            with_left_boundary: false,
            with_right_boundary: false,
            range: None,
            exact_amount: None,
        }
    }
}

pub struct GroupSettings {
    pub other: Settings,
    pub is_non_capture: bool,
    pub is_positive_lookahead: bool,
    pub is_negative_lookahead: bool,
}

impl Default for GroupSettings {
    fn default() -> Self {
        GroupSettings {
            other: Settings::default(),
            is_non_capture: false,
            is_positive_lookahead: false,
            is_negative_lookahead: false,
        }
    }
}
