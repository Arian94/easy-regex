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

pub const NO_SETTINGS: Settings = Settings {
    is_optional: false,
    is_one_or_more: false,
    is_nil_or_more: false,
    with_left_boundary: false,
    with_left_non_boundary: false,
    with_right_boundary: false,
    with_right_non_boundary: false,
    range: None,
    exactly: None,
};
pub const OPTIONAL: Settings = Settings {
    is_optional: true,
    is_one_or_more: false,
    is_nil_or_more: false,
    with_left_boundary: false,
    with_left_non_boundary: false,
    with_right_boundary: false,
    with_right_non_boundary: false,
    range: None,
    exactly: None,
};
pub const ONE_OR_MORE: Settings = Settings {
    is_optional: false,
    is_one_or_more: true,
    is_nil_or_more: false,
    with_left_boundary: false,
    with_left_non_boundary: false,
    with_right_boundary: false,
    with_right_non_boundary: false,
    range: None,
    exactly: None,
};
pub const NIL_OR_MORE: Settings = Settings {
    is_optional: false,
    is_one_or_more: false,
    is_nil_or_more: true,
    with_left_boundary: false,
    with_left_non_boundary: false,
    with_right_boundary: false,
    with_right_non_boundary: false,
    range: None,
    exactly: None,
};
pub const LEFT_BOUNDARY: Settings = Settings {
    is_optional: false,
    is_one_or_more: false,
    is_nil_or_more: false,
    with_left_boundary: true,
    with_left_non_boundary: false,
    with_right_boundary: false,
    with_right_non_boundary: false,
    range: None,
    exactly: None,
};
pub const RIGHT_BOUNDARY: Settings = Settings {
    is_optional: false,
    is_one_or_more: false,
    is_nil_or_more: false,
    with_left_boundary: false,
    with_left_non_boundary: false,
    with_right_boundary: true,
    with_right_non_boundary: false,
    range: None,
    exactly: None,
};
pub const BOTH_BOUNDARY: Settings = Settings {
    is_optional: false,
    is_one_or_more: false,
    is_nil_or_more: false,
    with_left_boundary: true,
    with_left_non_boundary: false,
    with_right_boundary: true,
    with_right_non_boundary: false,
    range: None,
    exactly: None,
};
pub const LEFT_NON_BOUNDARY: Settings = Settings {
    is_optional: false,
    is_one_or_more: false,
    is_nil_or_more: false,
    with_left_boundary: false,
    with_left_non_boundary: true,
    with_right_boundary: false,
    with_right_non_boundary: false,
    range: None,
    exactly: None,
};
pub const RIGHT_NON_BOUNDARY: Settings = Settings {
    is_optional: false,
    is_one_or_more: false,
    is_nil_or_more: false,
    with_left_boundary: false,
    with_left_non_boundary: false,
    with_right_boundary: false,
    with_right_non_boundary: true,
    range: None,
    exactly: None,
};
pub const BOTH_NON_BOUNDARY: Settings = Settings {
    is_optional: false,
    is_one_or_more: false,
    is_nil_or_more: false,
    with_left_boundary: false,
    with_left_non_boundary: true,
    with_right_boundary: false,
    with_right_non_boundary: true,
    range: None,
    exactly: None,
};

///////////////////////////////////// GROUP ////////////////////////////////////////////
pub const NO_GROUP_SETTINGS: GroupSettings = GroupSettings {
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
    },
    is_non_capture: false,
    is_negative_lookahead: false,
    is_positive_lookahead: false,
};
pub const OPTIONAL_GROUP: GroupSettings = GroupSettings {
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
    },
    is_non_capture: false,
    is_negative_lookahead: false,
    is_positive_lookahead: false,
};
pub const NON_CAPTURE: GroupSettings = GroupSettings {
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
    },
    is_non_capture: true,
    is_negative_lookahead: false,
    is_positive_lookahead: false,
};
pub const NEGATIVE_LOOKAHEAD: GroupSettings = GroupSettings {
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
    },
    is_non_capture: false,
    is_negative_lookahead: true,
    is_positive_lookahead: false,
};
pub const POSITIVE_LOOKAHEAD: GroupSettings = GroupSettings {
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
    },
    is_non_capture: false,
    is_negative_lookahead: false,
    is_positive_lookahead: true,
};