use crate::common::entities::form_input::FormInput;
use crate::common::entities::FormButton;

pub enum FormMethod {
    Get,
    Post,
    Put,
}

impl Default for FormMethod {
    fn default() -> Self {
        FormMethod::Get
    }
}

pub struct Form {
    id: String,
    action: String,
    title: String,
    method: FormMethod,
    csrf_token: String,
    fields: Vec<FormInput>,
    submit: FormButton,
}

impl Form {
    pub fn builder() -> FormBuilder {
        FormBuilder::default()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn method(&self) -> &FormMethod {
        &self.method
    }

    pub fn csrf_token(&self) -> &str {
        &self.csrf_token
    }

    pub fn fields(&self) -> &Vec<FormInput> {
        &self.fields
    }

    pub fn submit(&self) -> &FormButton {
        &self.submit
    }
}

#[derive(Default)]
pub struct FormBuilder {
    id: Option<String>,
    action: Option<String>,
    title: Option<String>,
    method: Option<FormMethod>,
    csrf_token: Option<String>,
    fields: Option<Vec<FormInput>>,
    submit: Option<FormButton>,
}

impl FormBuilder {
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn action(mut self, action: String) -> Self {
        self.action = Some(action);
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn method(mut self, method: FormMethod) -> Self {
        self.method = Some(method);
        self
    }

    pub fn csrf_token(mut self, csrf_token: String) -> Self {
        self.csrf_token = Some(csrf_token);
        self
    }

    pub fn fields(mut self, fields: Vec<FormInput>) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn add_field(mut self, field: FormInput) -> Self {
        let mut fields = self.fields.unwrap_or_default();
        fields.push(field);
        self.fields = Some(fields);
        self
    }

    pub fn submit(mut self, submit: FormButton) -> Self {
        self.submit = Some(submit);
        self
    }

    pub fn build(self) -> Result<Form, String> {
        Ok(Form {
            id: self.id.unwrap_or("".into()),
            action: self.action.unwrap_or("/".into()),
            title: self.title.unwrap_or("".into()),
            method: self.method.unwrap_or_default(),
            csrf_token: self.csrf_token.unwrap_or_default(),
            fields: self.fields.unwrap_or_default(),
            submit: self.submit.unwrap_or_default(),
        })
    }
}
