pub struct FormInput {
    pub id: String,
    pub name: String,
    pub value: String,
    pub title: String,
    pub input_type: String,
}

pub struct FormButton {
    pub id: String,
    pub title: String,
    pub button_type: String,
}

pub struct Form {
    pub title: String,
    pub fields: Vec<FormInput>,
    pub submit: FormButton,
}

impl FormInput {
    pub fn new(id: String, name: String, value: String, title: String, input_type: String) -> Self {
        Self {
            id,
            name,
            value,
            title,
            input_type,
        }
    }
}

impl FormButton {
    pub fn new(id: String, title: String, button_type: String) -> Self {
        Self {
            id,
            title,
            button_type,
        }
    }
}

impl Form {
    pub fn new(title: String, fields: Vec<FormInput>, submit: FormButton) -> Self {
        Self {
            title,
            fields,
            submit,
        }
    }
}
