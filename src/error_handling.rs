lazy_static! {
pub static ref ERR_RANGE_AND_STAR_OR_PLUS_EXP: &'static str = "cannot have range and * or + next to each other.";
pub static ref ERR_STAR_AND_PLUS_EXP: &'static str = "cannot have * and + next to each other.";
pub static ref ERR_RANGE_AND_EXACT_EXP: &'static str = "cannot have two quantifiers next to each other.";
pub static ref ERR_NON_CAPTURE_POSITIVE_LOOKAHEAD_GROUP: &'static str =
    "group cannot be non-capture and positive lookahead at the same time.";
pub static ref ERR_NONE_CAPTURE_NEGATIVE_LOOKAHEAD_GROUP: &'static str =
    "group cannot be non-capture and negative lookahead at the same time.";
pub static ref ERR_POSITIVE_NEGATIVE_LOOKAHEAD_GROUP: &'static str =
    "group cannot be positive and negative lookahead at the same time.";
pub static ref ERR_OPTIONAL_POSITIVE_LOOKAHEAD_GROUP: &'static str =
    "group cannot be positive lookahead and optional at the same time.";
pub static ref ERR_OPTIONAL_NEGATIVE_LOOKAHEAD_GROUP: &'static str =
    "group cannot be negative lookahead and optional at the same time.";
pub static ref ERR_POSITIVE_OR_NEGATIVE_LOOKAHEAD_WITH_RANGE_OR_EXACT_REPETITION_GROUP: &'static str =
    "group cannot be positive/negative lookahead having range or exact repitition at the same time.";
}

pub fn error_builder(expression: &str, err_desc: &str) -> String {
    format!("invalid regex in {}: {}.", expression, err_desc)
}
