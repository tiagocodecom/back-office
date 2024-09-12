/// A struct representing a URL-friendly slug, which is a lowercase
/// string composed of alphanumeric characters and dashes.
///
/// This struct ensures that slugs are valid according to the
/// specified constraints and provides methods for creating and
/// manipulating slugs.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Slug(String);

impl Slug {
    /// Creates a new `Slug` from a given string, validating that the
    /// string contains only lowercase letters, numbers, and dashes.
    ///
    /// # Panics
    ///
    /// This function will panic if the input string contains any
    /// characters other than lowercase letters, numbers, or dashes.
    ///
    /// # Examples
    ///
    /// ```
    /// use back_office::articles::entities::Slug;
    /// let slug = Slug::new("valid-slug".to_string());
    /// assert_eq!(slug.as_ref(), "valid-slug");
    ///
    /// // This will panic
    /// // let _ = Slug::new("Invalid Slug!".to_string());
    /// ```
    pub fn new(s: String) -> Self {
        let is_valid = s
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_numeric() || c.to_string() == "-");

        if !is_valid {
            panic!("{} is invalid (only lower case, numbers, dash spaced)", s);
        }

        Self(s)
    }
}

impl AsRef<str> for Slug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for Slug {
    fn from(s: String) -> Self {
        let slug = s
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("-")
            .to_lowercase()
            .chars()
            .filter(|c| {
                c.is_ascii_alphabetic()
                    || c.is_numeric()
                    || c.is_whitespace()
                    || c.to_string() == "-"
            })
            .collect::<String>()
            .replace(" ", "-");

        Self::new(slug)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panics_when_invalid_characters_are_present() {
        let _ = Slug::new("example-slug!".to_string());
        let _ = Slug::new("example slug!".to_string());
        let _ = Slug::new("example ``slug!".to_string());
        let _ = Slug::new("example  =slug!".to_string());
        let _ = Slug::new("example ``slug!".to_string());
    }

    #[test]
    fn creates_slug_with_valid_characters() {
        let slug = Slug::new("example-slug".to_string());
        assert_eq!(slug.as_ref(), "example-slug");
    }

    #[test]
    fn special_characters_and_spaces_are_converted_to_slug() {
        let slug = Slug::from("example slug!@#$%^&*()`~".to_string());
        assert_eq!(slug.as_ref(), "example-slug");
    }

    #[test]
    fn uppercase_characters_are_converted_to_lowercase_in_slug() {
        let slug = Slug::from("Example slug".to_string());
        assert_eq!(slug.as_ref(), "example-slug");
    }

    #[test]
    fn valid_slugs_are_not_modified() {
        let slug = Slug::from("example-slug".to_string());
        assert_eq!(slug.as_ref(), "example-slug");
    }

    #[test]
    fn trims_leading_and_trailing_spaces() {
        let slug = Slug::from("  example slug  ".to_string());
        assert_eq!(slug.as_ref(), "example-slug");
    }

    #[test]
    fn reduces_multiple_consecutive_spaces_to_single_dash() {
        let slug = Slug::from("example   slug".to_string());
        assert_eq!(slug.as_ref(), "example-slug");
    }
}
