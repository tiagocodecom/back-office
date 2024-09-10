/// Represents an input element in a form.
///
/// This struct encapsulates the properties of a form input field, such as
/// its identifier, name, value, and other attributes that define its behavior
/// and appearance in a form.
pub struct FormInput {
    id: String,
    name: String,
    value: String,
    title: String,
    placeholder: String,
    input_type: String,
    required: bool,
}

impl FormInput {
    /// Creates a new `FormInputBuilder` instance.
    ///
    /// This method initializes a new builder for constructing `FormInput` objects.
    ///
    /// # Example
    /// ```
    /// use back_office::common::entities::FormInput;
    ///
    /// let form_input = FormInput::builder()
    ///     .id("username".into())
    ///     .name("username".into())
    ///     .title("Username".into())
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> FormInputBuilder {
        FormInputBuilder::default()
    }

    /// Returns the identifier of the form input.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the form input.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the value of the form input.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the title of the form input.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the placeholder of the form input.
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// Returns the input type of the form input.
    pub fn input_type(&self) -> &str {
        &self.input_type
    }

    /// Returns whether the form input is required.
    pub fn required(&self) -> bool {
        self.required
    }
}

/// A builder for constructing `FormInput` instances.
///
/// This struct provides a flexible way to create `FormInput` objects by
/// setting various fields step by step. The builder pattern is particularly
/// useful when you have optional fields or want to ensure that certain fields
/// are set before constructing the object.
#[derive(Default)]
pub struct FormInputBuilder {
    id: Option<String>,
    name: Option<String>,
    value: Option<String>,
    title: Option<String>,
    placeholder: Option<String>,
    input_type: Option<String>,
    required: Option<bool>,
}

impl FormInputBuilder {
    /// Sets the `id` field of the `FormInput`.
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the `name` field of the `FormInput`.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the `value` field of the `FormInput`.
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    /// Sets the `title` field of the `FormInput`.
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the `placeholder` field of the `FormInput`.
    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Sets the `input_type` field of the `FormInput`.
    pub fn input_type(mut self, input_type: String) -> Self {
        self.input_type = Some(input_type);
        self
    }

    /// Sets the `required` field of the `FormInput`.
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// Constructs a `FormInput` instance from the provided fields.
    ///
    /// This method will return an error if either the `id` or `title` fields are not set.
    pub fn build(self) -> Result<FormInput, String> {
        let id = self.id.ok_or("id is required")?;

        Ok(FormInput {
            id: id.clone(),
            name: self.name.unwrap_or(id),
            value: self.value.unwrap_or_default(),
            title: self.title.ok_or("title is required")?,
            placeholder: self.placeholder.unwrap_or_default(),
            input_type: self.input_type.unwrap_or("text".into()),
            required: self.required.unwrap_or(false),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_form_input_with_all_fields() {
        let form_input = FormInput::builder()
            .id("email".into())
            .name("email".into())
            .value("".into())
            .title("Email".into())
            .placeholder("jonhdoe@gmail.com".into())
            .input_type("email".into())
            .required(true)
            .build()
            .unwrap();

        assert_eq!(form_input.id, "email");
        assert_eq!(form_input.name, "email");
        assert_eq!(form_input.value, "");
        assert_eq!(form_input.title, "Email");
        assert_eq!(form_input.placeholder, "jonhdoe@gmail.com");
        assert_eq!(form_input.input_type, "email");
        assert_eq!(form_input.required, true);
    }

    #[test]
    fn sets_input_type_to_text_by_default_when_unspecified() {
        let form_input = FormInput::builder()
            .id("email".into())
            .name("email".into())
            .title("Email".into())
            .value("".into())
            .build()
            .unwrap();

        assert_eq!(form_input.input_type, "text");
    }

    #[test]
    fn sets_required_to_false_by_default_when_unspecified() {
        let form_input = FormInput::builder()
            .id("email".into())
            .name("email".into())
            .title("Email".into())
            .value("".into())
            .build()
            .unwrap();

        assert_eq!(form_input.required, false);
    }

    #[test]
    #[should_panic]
    fn build_fails_if_id_or_title_is_missing() {
        let form_input = FormInput::builder()
            .name("email".into())
            .value("".into())
            .placeholder("".into())
            .build()
            .unwrap();
    }

    #[test]
    fn builds_form_input_with_defaults_when_only_id_and_title_are_provided() {
        let form_input = FormInput::builder()
            .id("email".into())
            .title("Email".into())
            .build()
            .unwrap();

        assert_eq!(form_input.id, "email");
        assert_eq!(form_input.name, "email");
        assert_eq!(form_input.input_type, "text");
        assert_eq!(form_input.title, "Email");
    }
}
