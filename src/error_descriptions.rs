pub const ERR_RANGE_AND_STAR_OR_PLUS_EXP: &str =
    "invalid regex: cannot have range and * or + next to each other.";
pub const ERR_STAR_AND_PLUS_EXP: &str = "invalid regex: cannot have * and + next to each other.";
pub const ERR_RANGE_AND_EXACT_EXP: &str =
    "invalid regex: cannot have two quantifiers next to each other.";
pub const ERR_NON_CAPTURE_POSITIVE_LOOKAHEAD_GROUP: &str =
    "invalid regex: group cannot be non-capture and positive lookahead at the same time.";
pub const ERR_NONE_CAPTURE_NEGATIVE_LOOKAHEAD_GROUP: &str =
    "invalid regex: group cannot be non-capture and negative lookahead at the same time.";
pub const ERR_POSITIVE_NEGATIVE_LOOKAHEAD_GROUP: &str =
    "invalid regex: group cannot be positive and negative lookahead at the same time.";
pub const ERR_OPTIONAL_POSITIVE_LOOKAHEAD_GROUP: &str =
    "invalid regex: group cannot be positive lookahead and optional at the same time.";
pub const ERR_OPTIONAL_NEGATIVE_LOOKAHEAD_GROUP: &str =
    "invalid regex: group cannot be negative lookahead and optional at the same time.";
