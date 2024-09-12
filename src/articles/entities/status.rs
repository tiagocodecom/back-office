use sqlx::Type;

#[derive(Debug, PartialEq, Clone, Type, Default)]
pub enum Status {
    #[default]
    Draft,
    Published,
    Archived,
}

impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "draft" => Status::Draft,
            "published" => Status::Published,
            "archived" => Status::Archived,
            _ => panic!("{} is not a valid status", value),
        }
    }
}

impl AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        match self {
            Status::Draft => "draft",
            Status::Published => "published",
            Status::Archived => "archived",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_valid_strings_to_enum_variants() {
        let draft = Status::from("draft");
        let published = Status::from("published");
        let archived = Status::from("archived");

        assert_eq!(draft, Status::Draft);
        assert_eq!(published, Status::Published);
        assert_eq!(archived, Status::Archived);
    }

    #[test]
    fn case_insensitive_conversion_to_enum_variant() {
        let draft = Status::from("DRAFT");
        let published = Status::from("PUBLISHED");
        let archived = Status::from("ARCHIVED");

        assert_eq!(draft, Status::Draft);
        assert_eq!(published, Status::Published);
        assert_eq!(archived, Status::Archived);
    }

    #[test]
    #[should_panic(with = "non_existent_status is not a valid status")]
    fn returns_error_for_invalid_status_string() {
        let _ = Status::from("non_existent_status");
    }

    #[test]
    fn converts_enums_to_strings_refs() {
        let draft = Status::Draft;
        let published = Status::Published;
        let archived = Status::Archived;

        assert_eq!(draft.as_ref(), "draft");
        assert_eq!(archived.as_ref(), "archived");
        assert_eq!(published.as_ref(), "published");
    }
}
