//! A handful of universal regular expressions.
//!
//! This is a collection of the most used regular expressions to reduce making rudimentary mistakes and make the code even more readable.
//! Except English, there are patterns for five other languages as Persian, French, German, Arabic and Chinese.

/// Should be used inside the **list** method for its full capability.
pub const ALPHA_NUMERIC: &str = "a-zA-Z0-9";
/// Should be used inside the **list** method for its full capability.
pub const UPPER_LOWER_CASE: &str = "a-zA-Z";
/// Should be used inside the **list** method for its full capability.
pub const LOWER_CASE: &str = "a-z";
/// Should be used inside the **list** method for its full capability.
pub const UPPER_CASE: &str = "A-Z";
/// Should be used inside the **list** method for its full capability.
pub const DIGITS: &str = "0-9";
pub const ANY: &str = ".";
pub const NULL_CHAR: &str = "\0";
pub const NEW_LINE: &str = "\n";
pub const FORM_FEED: &str = "\\f";
pub const TAB: &str = "\t";
pub const VERTICAL_TAB: &str = "\\v";
pub const BACKSPACE: &str = "[\\b]";
/// Retrieves two capturing groups, one for **username**, the other for **mail server and its domain**.
/// # Examples
///
/// ```
/// use easy_regex::{collection::*, EasyRegex};
/// let text = r#"something@email.co.uk"#;
///
/// let result = EasyRegex::new(EMAIL);
/// let captures = result.get_regex().unwrap();
/// captures.captures_iter(text).for_each(|caps| {
///     println!("{}, {}", &caps.get(1).unwrap().as_str(), &caps.get(2).unwrap().as_str());
/// })
/// // will print: something, email.co.uk
/// ```
pub const EMAIL: &str = r"([a-z0-9_+.]*)@([a-z0-9]+(?:[\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
/// Retrieves **protocol, subdomain, domain name, top level name, directory** and **query params** of a URL on multiple lines.
/// # Examples
///
/// ```
/// use easy_regex::{collection::*, EasyRegex};
/// let text = r#"http://www.swimming-pool.co.uk/products/shorts?searchMe=queryMe&name=smith
/// something@gmail.com
/// www.seasoning.com
/// university.gov helloworld.com
/// https://javaScript.com
/// "#;
///
/// let result = EasyRegex::new(WEBSITE_URL);
/// let captures = result.get_regex().unwrap();
/// captures.captures_iter(text).for_each(|caps| {
///     println!(
///         "protocol: {}, subdomain: {}, domain name: {}, top level name: {}, directory: {}, query params: {}\n",
///         // "protocol",
///         &caps.get(1).map_or("not found", |m| m.as_str()),
///         // "subdomain",
///         &caps.get(2).map_or("not found", |m| m.as_str()),
///         // "domain_name",
///         &caps.get(3).map_or("not found", |m| m.as_str()),
///         // "top_level_name",
///         &caps.get(4).map_or("not found", |m| m.as_str()),
///         // "directory",
///         &caps.get(5).map_or("not found", |m| m.as_str()),
///         // "query_params"
///         &caps.get(6).map_or("not found", |m| m.as_str()),
///     );  
///         // will print:
///         // protocol: http, subdomain: www, domain name: swimming-pool,
///         // top level name: .co.uk, directory: /products/shorts,
///         // query params: searchMe=queryMe&name=smith
///
///         // protocol: not found, subdomain: www, domain name: seasoning,
///         // top level name: .com, directory: not found, query params: not found
///
///         // protocol: not found, subdomain: not found, domain name: university,
///         // top level name: .gov, directory: not found, query params: not found
///
///         // protocol: https, subdomain: not found, domain name: javaScript,
///         // top level name: .com, directory: not found, query params: not found
/// })
/// ```
pub const WEBSITE_URL: &str = r"(?m)(?:(?:(?P<protocol>ftp|https?)://)?(?:(?P<subdomain>www)\.)?)?(?P<domain_name>[-a-zA-Z0-9]{2,253})(?P<top_level_name>(?:\.[a-z]{2,6})+)(?P<directory>(?:/[a-z0-9]+)+)?(?:\?(?P<query_params>[-a-zA-Z0-9@:%_\+~#()&//=]*))?";
/// Should be used inside the **list** method for its full capability.
pub const PERSIAN_ALPHABET: &str = r"\u0621-\u0628\u062A-\u063A\u0641-\u0642\u0644-\u0648\u064E-\u0651\u0655\u067E\u0686\u0698\u06A9\u06AF\u06BE\u06CC|\p{arabic}";
/// Should be used inside the **list** method for its full capability.
pub const PERSIAN_ARABIC_NUM: &str = r"\u06F0-\u06F9\u0660-\u0669";
/// Should be used inside the **list** method for its full capability.
pub const PERSIAN_ALPHA_NUMERIC: &str = r"\u0621-\u0628\u062A-\u063A\u0641-\u0642\u0644-\u0648\u064E-\u0651\u0655\u067E\u0686\u0698\u06A9\u06AF\u06BE\u06CC\u06F0-\u06F9\u0660-\u0669";
pub const PERSIAN_PUNCTUATION: &str = r"\u060C\u061B\u061F\u0640\u066A\u066B\u066C";
/// Should be used inside the **list** method for its full capability.
pub const PERSIAN_SPACES: &str = r"\u0020\u2000-\u200F\u2028-\u202F";
/// Should be used inside the **list** method for its full capability.
pub const FRENCH_ALPHABET: &str = r"a-zA-Z\u00C0-\u017F";
/// Should be used inside the **list** method for its full capability.
pub const GERMAN_ALPHABET: &str = r"a-zA-Z\u00E4\u00F6\u00FC\u00C4\u00D6\u00DC\u00df";
/// Should be used inside the **list** method for its full capability.
pub const CHINESE_ALPHABET: &str = r"\u4e00-\u9fa5";
/// Captures hour, minute and optional case-insensitive am/pm in 12-hour clock.
///
/// # Examples
/// ```
/// let text = r#"
/// 2:50 6:52 06:30 3:8
/// 7:43 18:59 4:50Pm 5:20 am
/// "#;
/// // By using this regex, the output will be:
/// // Some(Captures({
/// //    0: Some("2:50"),
/// //    1: Some("2"),
/// //    2: Some("50"),
/// //    3: None
/// // })),
/// // Some(Captures({
/// //    0: Some("6:52"),
/// //    1: Some("6"),
/// //    2: Some("52"),
/// //    3: None
/// // })),
/// // Some(Captures({
/// //    0: Some("06:30"),
/// //    1: Some("06"),
/// //    2: Some("30"),
/// //    3: None
/// // })),
/// // Some(Captures({
/// //    0: Some("3:8"),
/// //    1: Some("3"),
/// //    2: Some("8"),
/// //    3: None
/// // })),
/// // Some(Captures({
/// //    0: Some("7:43"),
/// //    1: Some("7"),
/// //    2: Some("43"),
/// //    3: None
/// // })),
//// // Some(Captures({
/// //    0: Some("4:50"),
/// //    1: Some("4"),
/// //    2: Some("50"),
/// //    3: Some("Pm"),
/// // })),
//// // Some(Captures({
/// //    0: Some("5:20"),
/// //    1: Some("5"),
/// //    2: Some("20"),
/// //    3: Some("am"),
/// // })),
/// ```
pub const TIME_HH_MM_12_AMPM: &str = r"\b(1[0-2]|0?[1-9]):([0-5]?\d)(?: ?((?i)[ap]m))?\b";
/// Same as ```TIME_HH_MM_12_AMPM``` capturing hour, minute, seconds and optional case-insensitive am/pm.
pub const TIME_HH_MM_SS_12_AMPM: &str =
    r"\b(1[0-2]|0?[1-9]):([0-5]?\d):([0-5]?\d)(?: ?((?i)[ap]m))?\b";
/// Captures hour and minute in 24-hour clock.
pub const TIME_HH_MM_24: &str = r"\b([01]?\d|2[0-3]):([0-5]?\d)\b";
/// Same as ```TIME_HH_MM_24``` captures hour, minute as well as seconds.
pub const TIME_HH_MM_SS_24: &str = r"\b([01]?\d|2[0-3]):([0-5]?\d):([0-5]?\d)\b";
/// Complete and abbreviated forms of Gregorian months (case sensitive).
pub const MONTH_NAMES: &str = r"January|Jan\.?|February|Feb\.?|March|Mar\.?|April|Apr\.?|May|June|Jun\.?July|Jul\.?|August|Aug\.?|September|Sep\.?|Sept|October|Oct\.?|November|Nov\.?|December|Dec\.?";
const _DAY: &str = r"([ 0-2]?[1-9]|[1-2]0|3[01])";
const _MONTH: &str = r"([ 0]?[1-9]|10|11|12)";
const _YEAR: &str = r"([1-9]\d{3})";
fn _date_regex_generator() -> String {
    format!(
        r"\b{}/{}/{}\b|\b{}/{}/{}\b|\b{}/{}/{}\b|\b{}{}{}\b|\b{}{}{}\b|\b{}{}{}\b|\b{}-{}-{}\b|\b{}-{}-{}\b|\b{}-{}-{}\b|\b({})(?:,| |, )?{}(?:,| |, )?{}\b|\b{}(?:,| |, )?({})(?:,| |, )?{}\b|\b{}(?:,| |, )?({})(?:,| |, )?{}\b",
        _MONTH,
        _DAY,
        _YEAR,
        _DAY,
        _MONTH,
        _YEAR,
        _YEAR,
        _MONTH,
        _DAY,
        _MONTH,
        _DAY,
        _YEAR,
        _DAY,
        _MONTH,
        _YEAR,
        _YEAR,
        _MONTH,
        _DAY,
        _MONTH,
        _DAY,
        _YEAR,
        _DAY,
        _MONTH,
        _YEAR,
        _YEAR,
        _MONTH,
        _DAY,
        MONTH_NAMES,
        _DAY,
        _YEAR,
        _DAY,
        MONTH_NAMES,
        _YEAR,
        _YEAR,
        MONTH_NAMES,
        _DAY,
    )
}
/// Captures day, month and year in all valid formats.
/// ```text
/// "10/25/2025", "25-10-2025", "Feb, 15 2020", etc.
/// ```
pub const DATE: &str = r"\b([ 0]?[1-9]|10|11|12)/([ 0-2]?[1-9]|[1-2]0|3[01])/([1-9]\d{3})\b|\b([ 0-2]?[1-9]|[1-2]0|3[01])/([ 0]?[1-9]|10|11|12)/([1-9]\d{3})\b|\b([1-9]\d{3})/([ 0]?[1-9]|10|11|12)/([ 0-2]?[1-9]|[1-2]0|3[01])\b|\b([ 0]?[1-9]|10|11|12)([ 0-2]?[1-9]|[1-2]0|3[01])([1-9]\d{3})\b|\b([ 0-2]?[1-9]|[1-2]0|3[01])([ 0]?[1-9]|10|11|12)([1-9]\d{3})\b|\b([1-9]\d{3})([ 0]?[1-9]|10|11|12)([ 0-2]?[1-9]|[1-2]0|3[01])\b|\b([ 0]?[1-9]|10|11|12)-([ 0-2]?[1-9]|[1-2]0|3[01])-([1-9]\d{3})\b|\b([ 0-2]?[1-9]|[1-2]0|3[01])-([ 0]?[1-9]|10|11|12)-([1-9]\d{3})\b|\b([1-9]\d{3})-([ 0]?[1-9]|10|11|12)-([ 0-2]?[1-9]|[1-2]0|3[01])\b|\b(January|Jan\.?|February|Feb\.?|March|Mar\.?|April|Apr\.?|May|June|Jun\.?July|Jul\.?|August|Aug\.?|September|Sep\.?|Sept|October|Oct\.?|November|Nov\.?|December|Dec\.?)(?:,| |, )?([ 0-2]?[1-9]|[1-2]0|3[01])(?:,| |, )?([1-9]\d{3})\b|\b([ 0-2]?[1-9]|[1-2]0|3[01])(?:,| |, )?(January|Jan\.?|February|Feb\.?|March|Mar\.?|April|Apr\.?|May|June|Jun\.?July|Jul\.?|August|Aug\.?|September|Sep\.?|Sept|October|Oct\.?|November|Nov\.?|December|Dec\.?)(?:,| |, )?([1-9]\d{3})\b|\b([1-9]\d{3})(?:,| |, )?(January|Jan\.?|February|Feb\.?|March|Mar\.?|April|Apr\.?|May|June|Jun\.?July|Jul\.?|August|Aug\.?|September|Sep\.?|Sept|October|Oct\.?|November|Nov\.?|December|Dec\.?)(?:,| |, )?([ 0-2]?[1-9]|[1-2]0|3[01])\b";
pub const IPV4: &str = r"\b(?:(?:[0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}(?:[0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\b";
lazy_static! {
    pub static ref IPV6: &'static str = r"\b(?:(?:[0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|(?:[0-9a-fA-F]{1,4}:){1,7}:|(?:[0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|(?:[0-9a-fA-F]{1,4}:){1,5}(?::[0-9a-fA-F]{1,4}){1,2}|(?:[0-9a-fA-F]{1,4}:){1,4}(?::[0-9a-fA-F]{1,4}){1,3}|(?:[0-9a-fA-F]{1,4}:){1,3}(?::[0-9a-fA-F]{1,4}){1,4}|(?:[0-9a-fA-F]{1,4}:){1,2}(?::[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:(?:(?::[0-9a-fA-F]{1,4}){1,6})|:(?:(?::[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(?::[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(?:ffff(?::0{1,4}){0,1}:){0,1}(?:(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])|(?:[0-9a-fA-F]{1,4}:){1,4}:(?:(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9]))\b";
    pub static ref IPV4_6: &'static str = r"\b(?:(?:(?:(?:(?:[0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}(?:[0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])))|(?:(?:(?:(?:[0-9A-Fa-f]{1,4}:){7}(?:[0-9A-Fa-f]{1,4}|:))|(?:(?:[0-9A-Fa-f]{1,4}:){6}(?::[0-9A-Fa-f]{1,4}|(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(?:(?:[0-9A-Fa-f]{1,4}:){5}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,2})|:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(?:(?:[0-9A-Fa-f]{1,4}:){4}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,3})|(?:(?::[0-9A-Fa-f]{1,4})?:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?:(?:[0-9A-Fa-f]{1,4}:){3}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,4})|(?:(?::[0-9A-Fa-f]{1,4}){0,2}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?:(?:[0-9A-Fa-f]{1,4}:){2}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,5})|(?:(?::[0-9A-Fa-f]{1,4}){0,3}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?:(?:[0-9A-Fa-f]{1,4}:){1}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,6})|(?:(?::[0-9A-Fa-f]{1,4}){0,4}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?::(?:(?:(?::[0-9A-Fa-f]{1,4}){1,7})|(?:(?::[0-9A-Fa-f]{1,4}){0,5}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:)))(?:%.+)?))\b";
}

#[cfg(test)]
mod tests {
    use crate::{
        collection::*,
        settings::{base::*, group::*},
        EasyRegex,
    };

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

    #[test]
    fn website_url_works() {
        let text = r#"http://www.swimming-pool.co.uk/products/shorts?searchMe=queryMe&name=smith
            something@gmail.com
            www.seasoning.com
            university.gov helloworld.com
            https://javaScript.com
        "#;

        let result = EasyRegex::new(WEBSITE_URL);
        let captures = result.get_regex().unwrap();
        captures.captures_iter(text).for_each(|caps| {
            println!(
                "protocol: {}, subdomain: {}, domain name: {}, top level name: {}, directory: {}, query params: {}\n",
                &caps.get(1).map_or("not found", |m| m.as_str()), // "protocol",
                &caps.get(2).map_or("not found", |m| m.as_str()), // "subdomain",
                &caps.get(3).map_or("not found", |m| m.as_str()), // "domain_name",
                &caps.get(4).map_or("not found", |m| m.as_str()), // "top_level_name",
                &caps.get(5).map_or("not found", |m| m.as_str()), // "directory",
                &caps.get(6).map_or("not found", |m| m.as_str()), // "query_params"
            );
        })
    }

    #[test]
    fn time_works() {
        let text = "7:4 5:20 6:30am 02:2 01:30";
        let result = EasyRegex::new(TIME_HH_MM_12_AMPM);
        result
            .clone()
            .get_regex()
            .unwrap()
            .captures_iter(text)
            .for_each(|f| {
                println!("{:?}", f);
            });
        let count = result.get_regex().unwrap().captures_iter(text).count();
        assert_eq!(5, count);
    }

    #[test]
    fn date_and_time_works() {
        let text = r#"
            Feb 17 2009 5:3am 03/26/1994 8:41 23/7/2030 9:20Pm
            12 Sept 2015 6:14 03-26-1994 2:18 2030/4/27 3:50
        "#;
        let result = EasyRegex::new_section()
            .group(DATE, &DEFAULT_GROUP) // will capture any valid format of a date.
            .literal_space()
            .group(TIME_HH_MM_24, &DEFAULT_GROUP); // will capture hours and minutes in 24-hour clock.
        result
            .clone()
            .get_regex()
            .unwrap()
            .captures_iter(text)
            .for_each(|captures| println!("{}", captures.get(0).unwrap().as_str()));
        //

        let matched_patterns_count = result.get_regex().unwrap().captures_iter(text).count();
        assert_eq!(4, matched_patterns_count);
        // let text = r#"
        // Feb 17 2009 5:3 am 23/7/2030 9:20Pm
        // 12 Sept 2015 6:14
        // "#;
        // let result = EasyRegex::new_section()
        //     .group(DATE, &DEFAULT_GROUP)
        //     .space()
        //     .group(TIME_HH_MM_12_AMPM, &DEFAULT_GROUP);
        // result
        //     .clone()
        //     .get_regex()
        //     .unwrap()
        //     .captures_iter(text)
        //     .for_each(|captures| println!("{}", captures.get(0).unwrap().as_str()));

        //     let count = result.get_regex().unwrap().captures_iter(text).count();
        // assert_eq!(3, count);
    }

    #[test]
    fn ip_works() {
        let text =
            "2001:0db8:85a3:0000:0000:8a2e:0370:7334 5002:0db8:85a3:0000:0000:8a2e:0560:7334";
        let result = EasyRegex::new(&IPV6);
        result
            .clone()
            .get_regex()
            .unwrap()
            .captures_iter(text)
            .for_each(|f| {
                println!("{:?}", f);
            });
        let count = result.get_regex().unwrap().captures_iter(text).count();
        assert_eq!(2, count);
    }

    #[test]
    fn date_gen_output() {
        println!("{}", _date_regex_generator());
    }
}
