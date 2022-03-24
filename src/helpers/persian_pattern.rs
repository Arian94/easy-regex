//! Unicode characters of Persian letters are not defined in the same order as their corresponding letters.
//! This could result in confusion when creating customized persian word patterns.
//! To solve that, a helper method is defined here.

#[allow(dead_code)]

struct LettersIndices<'a> {
    letter: &'a str,
    index: &'a str,
    hex: &'a str,
    name: &'a str,
}

lazy_static! {
    static ref LETTERS: [LettersIndices<'static>; 43] = [
        LettersIndices {
            index: "\u{0621}}",
            letter: "ء",
            hex: "d8 a1",
            name: "ARABIC LETTER HAMZA",
        },
        LettersIndices {
            index: "\u{0622}",
            letter: "آ",
            hex: "d8 a2",
            name: "ARABIC LETTER ALEF WITH MADDA ABOVE",
        },
        LettersIndices {
            index: "\u{0623}",
            letter: "أ",
            hex: "d8 a3",
            name: "ARABIC LETTER ALEF WITH HAMZA ABOVE",
        },
        LettersIndices {
            index: "\u{0624}",
            letter: "ؤ",
            hex: "d8 a4",
            name: "ARABIC LETTER WAW WITH HAMZA ABOVE",
        },
        LettersIndices {
            index: "\u{0625}",
            letter: "إ",
            hex: "d8 a5",
            name: "ARABIC LETTER ALEF WITH HAMZA BELOW",
        },
        LettersIndices {
            index: "\u{0626}",
            letter: "ئ",
            hex: "d8 a6",
            name: "ARABIC LETTER YEH WITH HAMZA ABOVE",
        },
        LettersIndices {
            index: "\u{0627}",
            letter: "ا",
            hex: "d8 a7",
            name: "ARABIC LETTER ALEF",
        },
        LettersIndices {
            index: "\u{0628}",
            letter: "ب",
            hex: "d8 a8",
            name: "ARABIC LETTER BEH",
        },
        LettersIndices {
            index: "\u{067E}",
            letter: "پ",
            hex: "d9 be",
            name: "PERSIAN LETTER PEH",
        },
        LettersIndices {
            index: "\u{062A}",
            letter: "ت",
            hex: "d8 aa",
            name: "ARABIC LETTER TEH",
        },
        LettersIndices {
            index: "\u{062B}",
            letter: "ث",
            hex: "d8 ab",
            name: "ARABIC LETTER THEH",
        },
        LettersIndices {
            index: "\u{062C}",
            letter: "ج",
            hex: "d8 ac",
            name: "ARABIC LETTER JEEM",
        },
        LettersIndices {
            index: "\u{0686}",
            letter: "چ",
            hex: "da 86",
            name: "ARABIC LETTER TCHEH",
        },
        LettersIndices {
            index: "\u{062D}",
            letter: "ح",
            hex: "d8 ad",
            name: "ARABIC LETTER HAH",
        },
        LettersIndices {
            index: "\u{062E}",
            letter: "خ",
            hex: "d8 ae",
            name: "ARABIC LETTER KHAH",
        },
        LettersIndices {
            index: "\u{062F}",
            letter: "د",
            hex: "d8 af",
            name: "ARABIC LETTER DAL",
        },
        LettersIndices {
            index: "\u{0630}",
            letter: "ذ",
            hex: "d8 b0",
            name: "ARABIC LETTER ZAL",
        },
        LettersIndices {
            index: "\u{0631}",
            letter: "ر",
            hex: "d8 b1",
            name: "ARABIC LETTER REH",
        },
        LettersIndices {
            index: "\u{0632}",
            letter: "ز",
            hex: "d8 b2",
            name: "ARABIC LETTER ZAIN",
        },
        LettersIndices {
            index: "\u{0698}",
            letter: "ژ",
            hex: "da 98",
            name: "ARABIC LETTER ZHEH",
        },
        LettersIndices {
            index: "\u{0633}",
            letter: "س",
            hex: "d8 b3",
            name: "ARABIC LETTER SEEN",
        },
        LettersIndices {
            index: "\u{0634}",
            letter: "ش",
            hex: "d8 b4",
            name: "ARABIC LETTER SHEEN",
        },
        LettersIndices {
            index: "\u{0635}",
            letter: "ص",
            hex: "d8 b5",
            name: "ARABIC LETTER SAD",
        },
        LettersIndices {
            index: "\u{0636}",
            letter: "ض",
            hex: "d8 b6",
            name: "ARABIC LETTER ZAD",
        },
        LettersIndices {
            index: "\u{0637}",
            letter: "ط",
            hex: "d8 b7",
            name: "ARABIC LETTER TAH",
        },
        LettersIndices {
            index: "\u{0638}",
            letter: "ظ",
            hex: "d8 b8",
            name: "ARABIC LETTER ZAH",
        },
        LettersIndices {
            index: "\u{0639}",
            letter: "ع",
            hex: "d8 b9",
            name: "ARABIC LETTER AIN",
        },
        LettersIndices {
            index: "\u{063A}",
            letter: "غ",
            hex: "d8 ba",
            name: "ARABIC LETTER GHAIN",
        },
        LettersIndices {
            index: "\u{0641}",
            letter: "ف",
            hex: "d9 81",
            name: "ARABIC LETTER FEH",
        },
        LettersIndices {
            index: "\u{0642}",
            letter: "ق",
            hex: "d9 82",
            name: "ARABIC LETTER QAF",
        },
        LettersIndices {
            index: "\u{06A9}",
            letter: "ک",
            hex: "da a9",
            name: "ARABIC LETTER KEHEH",
        },
        LettersIndices {
            index: "\u{06AF}",
            letter: "گ",
            hex: "da af",
            name: "ARABIC LETTER GAF",
        },
        LettersIndices {
            index: "\u{0644}",
            letter: "ل",
            hex: "d9 84",
            name: "ARABIC LETTER LAM",
        },
        LettersIndices {
            index: "\u{0645}",
            letter: "م",
            hex: "d9 85",
            name: "ARABIC LETTER MEEM",
        },
        LettersIndices {
            index: "\u{0646}",
            letter: "ن",
            hex: "d9 86",
            name: "ARABIC LETTER NOON",
        },
        LettersIndices {
            index: "\u{0648}",
            letter: "و",
            hex: "d9 88",
            name: "ARABIC LETTER WAW",
        },
        LettersIndices {
            index: "\u{0647}\u{06BE}",
            letter: "ه",
            hex: "ه: d9 87, ھ: da be",
            name: "ARABIC LETTER HEH AND HEH DOACHASHMEE",
        },
        LettersIndices {
            index: "\u{06CC}",
            letter: "ی",
            hex: "db 8c",
            name: "ARABIC LETTER FARSI YEH",
        },
        LettersIndices {
            index: "\u{064E}",
            letter: "َ",
            hex: "d9 8e",
            name: "ARABIC FATHA",
        },
        LettersIndices {
            index: "\u{064F}",
            letter: "ُ",
            hex: "d9 8f",
            name: "ARABIC DAMMA",
        },
        LettersIndices {
            index: "\u{0650}",
            letter: "ِ",
            hex: "d9 90",
            name: "ARABIC KASRA",
        },
        LettersIndices {
            index: "\u{0651}",
            letter: "ّ",
            hex: "d9 91",
            name: "ARABIC SHADDA",
        },
        LettersIndices {
            index: "\u{0655}",
            letter: "ٕ",
            hex: "d9 95",
            name: "ARABIC HAMZA BELOW",
        },
    ];
}

/// To create a range of persian letters.
///
/// # Examples
/// ```
/// use easy_regex::helpers::persian_pattern::create_persian_pattern;
/// 
/// let result = create_persian_pattern(("ا"), ("ر"));
/// assert_eq!("ابپتثجچحخدذر", result.unwrap());
/// ```
pub fn create_persian_pattern<'a>(from: &'a str, to: &'a str) -> Result<String, String> {
    if let Some(found_from_idx) = LETTERS.iter().position(|l| l.letter == from) {
        if let Some(found_to_idx) = LETTERS.iter().position(|l| l.letter == to) {
            if found_from_idx > found_to_idx {
                let err_msg = format!(
                    "letter '{}' is after letter '{}', consider swapping them",
                    &from, &to
                );
                Err(err_msg)
            } else {
                let found_slice = &LETTERS[found_from_idx..found_to_idx + 1];
                let mut result = String::new();
                found_slice.iter().for_each(|l| {
                    result.push_str(l.index);
                });

                Ok(result)
            }
        } else {
            let err_msg = format!("letter '{}' is not valid", &to);
            Err(err_msg)
        }
    } else {
        let err_msg = format!("letter '{}' is not valid", &from);
        Err(err_msg)
    }
}

#[cfg(test)]
mod tests {
    use super::create_persian_pattern;

    #[test]
    fn create_persian_pattern_works() {
        let result = create_persian_pattern("ا", "د");
        assert_eq!("ابپتثجچحخد", result.unwrap())
    }

    #[test]
    fn create_persian_pattern_wrong_from_input() {
        let result = create_persian_pattern("d", "م");
        assert_eq!("letter 'd' is not valid", result.unwrap_err())
    }

    #[test]
    fn create_persian_pattern_wrong_to_input() {
        let result = create_persian_pattern("ا", "d");
        assert_eq!("letter 'd' is not valid", result.unwrap_err())
    }

    #[test]
    fn create_persian_pattern_wrong_order() {
        let result = create_persian_pattern("خ", "ا");
        assert_eq!(
            "letter 'خ' is after letter 'ا', consider swapping them",
            result.unwrap_err()
        );
    }
}
