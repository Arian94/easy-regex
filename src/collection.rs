lazy_static! {
    pub static ref ALPHA_NUMERIC: &'static str = "a-zA-Z0-9";
    pub static ref UPPER_LOWER_CASE: &'static str = "a-zA-Z";
    pub static ref NUMBERS: &'static str = "0-9";
    pub static ref ANY: &'static str = ".";
    pub static ref ANY_ALPHA_NUMERIC: &'static str = "\\w";
    pub static ref NOT_ANY_ALPHA_NUMERIC: &'static str = "\\W";
    pub static ref DIGIT: &'static str = "\\d";
    pub static ref NO_DIGIT: &'static str = "\\D";
    pub static ref SPACE: &'static str = "\\s";
    pub static ref NO_SPACE: &'static str = "\\S";
    pub static ref NULL_CHAR: &'static str = "\0";
    pub static ref NEW_LINE: &'static str = "\n";
    pub static ref FORM_FEED: &'static str = "\\f";
    pub static ref TAB: &'static str = "\t";
    pub static ref VERTICAL_TAB: &'static str = "\\v";
    pub static ref BACKSPACE: &'static str = "[\\b]";
    pub static ref EMAIL: &'static str = r"([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
    pub static ref WEBSITE_URL: &'static str = r"((ftp|http|https):\/\/)?(www.)?(?!.*(ftp|http|https|www.))[a-zA-Z0-9_-]+(\.[a-zA-Z]+)+((\/)[\w#]+)*(\/\w+\?[a-zA-Z0-9_]+=\w+(&[a-zA-Z0-9_]+=\w+)*)?\/?";
}
