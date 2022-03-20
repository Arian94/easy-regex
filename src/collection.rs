//! A handful of universal regular expressions.
//! 
//! This is a collection of the most used regular expressions to reduce making rudimentary mistakes and make the code even more readable.
//! Except English, there are patterns for five other languages as Persian, French, German, Arabic and Chinese.

lazy_static! {
    pub static ref ALPHA_NUMERIC: &'static str = "a-zA-Z0-9";
    pub static ref NUMERIC: &'static str = "0-9";
    pub static ref UPPER_LOWER_CASE: &'static str = "a-zA-Z";
    pub static ref LOWER_CASE: &'static str = "a-z";
    pub static ref UPPER_CASE: &'static str = "A-Z";
    pub static ref DIGITS: &'static str = "0-9";
    pub static ref ANY: &'static str = ".";
    pub static ref NULL_CHAR: &'static str = "\0";
    pub static ref NEW_LINE: &'static str = "\n";
    pub static ref FORM_FEED: &'static str = "\\f";
    pub static ref TAB: &'static str = "\t";
    pub static ref VERTICAL_TAB: &'static str = "\\v";
    pub static ref BACKSPACE: &'static str = "[\\b]";
    pub static ref EMAIL: &'static str =
        r"([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
    pub static ref WEBSITE_URL: &'static str = r"((ftp|http|https):\/\/)?(www.)?(?!.*(ftp|http|https|www.))[a-zA-Z0-9_-]+(\.[a-zA-Z]+)+((\/)[\w#]+)*(\/\w+\?[a-zA-Z0-9_]+=\w+(&[a-zA-Z0-9_]+=\w+)*)?\/?";
    pub static ref PERSIAN_ALPHABET: &'static str = r"\u0621-\u0628\u062A-\u063A\u0641-\u0642\u0644-\u0648\u064E-\u0651\u0655\u067E\u0686\u0698\u06A9\u06AF\u06BE\u06CC";
    pub static ref PERSIAN_ARABIC_NUM: &'static str = r"\u06F0-\u06F9\u0660-\u0669";
    pub static ref PERSIAN_ALPHA_NUMERIC: &'static str = r"\u0621-\u0628\u062A-\u063A\u0641-\u0642\u0644-\u0648\u064E-\u0651\u0655\u067E\u0686\u0698\u06A9\u06AF\u06BE\u06CC\u06F0-\u06F9\u0660-\u0669";
    pub static ref PERSIAN_ALPHA_PUNCTUATION: &'static str =
        r"\u060C\u061B\u061F\u0640\u066A\u066B\u066C";
    pub static ref PERSIAN_SPACES: &'static str = r"\u0020\u2000-\u200F\u2028-\u202F";
    pub static ref FRENCH_ALPHABET: &'static str = r"a-zA-Z\u00C0-\u017F";
    pub static ref GERMAN_ALPHABET: &'static str = r"a-zA-Z\u00E4\u00F6\u00FC\u00C4\u00D6\u00DC\u00df";
    pub static ref CHINESE_ALPHABET: &'static str =r"\u4e00-\u9fa5";
}

#[cfg(test)]
mod tests {
    use crate::{collection::*, settings::base::*, EasyRegex};

    #[test]
    fn persian_words_regex_works() {
        let result = EasyRegex::new_section().list(&PERSIAN_ALPHA_NUMERIC, &ONE_OR_MORE);

        let text = "سلام شماره من ۱۲۳۶ است";
        let is_match = result.clone().get_regex().unwrap().find_iter(text).count();
        result
            .get_regex()
            .unwrap()
            .find_iter(text)
            .into_iter()
            .for_each(|found| {
                println!("{}", found.as_str());
            });

        assert_eq!(5, is_match);
    }

    #[test]
    fn french_words_regex_works() {
        let text = "Adélaïde Aurélie Gaëlle";
        let result = EasyRegex::new_section().list(&FRENCH_ALPHABET, &ONE_OR_MORE);

        let count = result.get_regex().unwrap().captures_iter(text).count();
        assert_eq!(3, count);
    }

    #[test]
    fn german_words_regex_works() {
        let text = "Müller Sönke Käthe";
        let result = EasyRegex::new_section().list(&GERMAN_ALPHABET, &ONE_OR_MORE);

        let count = result.get_regex().unwrap().captures_iter(text).count();
        assert_eq!(3, count);
    }

    #[test]
    fn chinese_words_regex_works() {
        let text = "正则表达式";
        let result = EasyRegex::new_section().list(&CHINESE_ALPHABET, &ONE_OR_MORE);

        let is_match = result.get_regex().unwrap().is_match(text);
        assert_eq!(true, is_match);
    }
}


