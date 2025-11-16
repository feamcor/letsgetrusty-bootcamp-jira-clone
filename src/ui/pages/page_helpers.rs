use ellipse::Ellipse;
use itertools::Itertools;

pub fn get_column_string(text: &str, width: usize) -> String {
    let text = text.trim();
    if width == 0 {
        String::from("")
    } else if width <= 3 {
        ".".repeat(width)
    } else if text.is_empty() {
        " ".repeat(width)
    } else if text.len() < width && width - text.len() < 3 {
        let mut text = String::from(text);
        let trailing_spaces = " ".repeat(width - text.len());
        text.push_str(&trailing_spaces);
        text
    } else if text.len() == width {
        String::from(text)
    } else {
        String::from(text.truncate_ellipse(width - 3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    } 
}
