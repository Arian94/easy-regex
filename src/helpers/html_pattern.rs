//! Helper methods related to creating regex for HTML elements.

use std::borrow::BorrowMut;

/// A Recursive structure for defining HTML pattern.
pub struct HtmlPattern<'a> {
    pub tag_name: &'a str,
    pub essential_attribute: Option<&'a str>,
    pub child_html: Box<Option<HtmlPattern<'a>>>,
}

/// Creates regular expressions for HTML elements recursively.
/// 
/// This methods takes an HTML element which could have nested elements and outputs a regular expression
/// that captures the innerHTML. If the element has nested elements, it will caputure the deepest child element's innerHTML.
///
/// # Examples
///
/// ```
/// use easy_regex::helpers::html_pattern::{HtmlPattern, create_pattern};
///
/// let pattern_str = HtmlPattern {
///     tag_name: "div",
///     essential_attribute: Some("id=\"86\""),
///     child_html: Box::new(Some(HtmlPattern {
///         tag_name: "div",
///         essential_attribute: Some("class=\"some-class\""),
///         child_html: Box::new(None),
///     })
/// )};
/// 
/// let result = create_pattern(pattern_str);
/// assert_eq!(
/// "<div[^<>]*id=\"86\"[^<>]*>.*<div[^<>]*class=\"some-class\"[^<>]*>(.*)</div>.*</div>",
/// result
/// );
/// ```
pub fn create_pattern(pattern: HtmlPattern) -> String {
    let mut html_pattern = format!("<{}[^<>]*", pattern.tag_name);

    if let Some(ess_attr) = pattern.essential_attribute {
        let attr = format!("{}[^<>]*>.*", ess_attr);
        html_pattern.push_str(&attr);
    } else {
        html_pattern.push_str(">.*");
    }

    let some_ref = &mut String::new();
    some_ref.push_str(&html_pattern);
    recursive_pattern(some_ref.borrow_mut(), pattern.child_html.unwrap());
    let end = format!(".*</{}>", pattern.tag_name);
    some_ref.push_str(&end);

    some_ref.to_string()
}

fn recursive_pattern(some_ref: &mut String, pattern: HtmlPattern) -> String {
    let mut html_pattern = format!("<{}[^<>]*", pattern.tag_name);

    if let Some(ess_attr) = pattern.essential_attribute {
        let attr = format!("{}[^<>]*>", ess_attr);
        html_pattern.push_str(&attr);
    } else {
        html_pattern.push_str(">");
    }

    some_ref.push_str(&html_pattern);

    if pattern.child_html.is_some() {
        let child = pattern.child_html.unwrap();
        recursive_pattern(some_ref.borrow_mut(), child);
        let end = format!(".*</{}>", pattern.tag_name);
        some_ref.push_str(&end);
    } else {
        let tail_pattern = format!("(.*)</{}>", pattern.tag_name);
        some_ref.push_str(&tail_pattern);
    }
    some_ref.to_string()
}

#[cfg(test)]
mod tests {
    use super::{create_pattern, HtmlPattern};

    #[test]
    fn one_level_nested_elements() {
        let pattern_str = HtmlPattern {
            tag_name: "div",
            essential_attribute: Some("id=\"86\""),
            child_html: Box::new(Some(HtmlPattern {
                tag_name: "div",
                essential_attribute: Some("class=\"some-class\""),
                child_html: Box::new(None),
            })),
        };

        let result = create_pattern(pattern_str);
        assert_eq!(
            "<div[^<>]*id=\"86\"[^<>]*>.*<div[^<>]*class=\"some-class\"[^<>]*>(.*)</div>.*</div>",
            result
        );
    }

    #[test]
    fn two_level_nested_elements() {
        let pattern_str = HtmlPattern {
            tag_name: "div",
            essential_attribute: Some("id=\"86\""),
            child_html: Box::new(Some(HtmlPattern {
                tag_name: "div",
                essential_attribute: Some("class=\"some-class\""),
                child_html: Box::new(Some(HtmlPattern {
                    tag_name: "p",
                    essential_attribute: None,
                    child_html: Box::new(None),
                })),
            })),
        };

        let result = create_pattern(pattern_str);
        assert_eq!("<div[^<>]*id=\"86\"[^<>]*>.*<div[^<>]*class=\"some-class\"[^<>]*><p[^<>]*>(.*)</p>.*</div>.*</div>", result);
    }
}
