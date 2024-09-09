use serde::Deserialize;

/// Represents a button element in a form.
///
/// This struct encapsulates the properties of a form button, such as
/// its identifier, title, and type.
pub struct FormButton {
    id: String,
    title: String,
    button_type: String,
}

/// A builder for constructing `FormButton` instances.
///
/// This struct allows for flexible construction of `FormButton` objects by
/// setting various fields step by step. It's particularly useful when
/// you want to ensure certain fields are set before creating the object.
#[derive(Default)]
pub struct FormButtonBuilder {
    id: Option<String>,
    title: Option<String>,
    button_type: Option<String>,
}

impl FormButton {
    /// Creates a new `FormButtonBuilder` instance.
    ///
    /// This method initializes a new builder for constructing `FormButton` objects.
    pub fn builder() -> FormButtonBuilder {
        FormButtonBuilder::default()
    }

    /// Returns the identifier of the form button.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the title of the form button.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the type of the form button (e.g., "submit", "button").
    pub fn button_type(&self) -> &str {
        &self.button_type
    }
}

impl Default for FormButton {
    /// Provides a default `FormButton` instance.
    ///
    /// This implementation creates a `FormButton` with a default `id` of "submit",
    /// a default `title` of "Submit", and a default `button_type` of "button".
    fn default() -> Self {
        FormButton::builder()
            .id("submit".into())
            .title("Submit".to_string())
            .button_type("button".to_string())
            .build()
            .unwrap()
    }
}

impl FormButtonBuilder {
    /// Sets the `id` field of the `FormButton`.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the `title` field of the `FormButton`.
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the `button_type` field of the `FormButton`.
    pub fn button_type(mut self, button_type: String) -> Self {
        self.button_type = Some(button_type);
        self
    }

    /// Constructs a `FormButton` instance from the provided fields.
    ///
    /// This method will return an error if the `title` field is not set.
    pub fn build(self) -> Result<FormButton, String> {
        Ok(FormButton {
            id: self.id.unwrap_or_default(),
            title: self.title.ok_or("Title is required".to_string())?,
            button_type: self.button_type.unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_form_button_with_all_fields() {
        let form_button = FormButton::builder()
            .id("submit_form".into())
            .title("Send".into())
            .button_type("submit".into())
            .build()
            .unwrap();

        assert_eq!(form_button.id(), "submit_form");
        assert_eq!(form_button.title(), "Send");
        assert_eq!(form_button.button_type(), "submit");
    }

    #[test]
    fn build_fails_when_title_is_not_set() {
        let form_button = FormButton::builder()
            .id("submit_form".into())
            .button_type("submit".into())
            .build();

        assert!(form_button.is_err());
    }
}
