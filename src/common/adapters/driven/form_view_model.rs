use crate::common::entities::Form;
use crate::common::entities::FormMethod;
use serde::Serialize;

#[derive(Serialize)]
pub struct FormViewModel {
    id: String,
    action: String,
    title: String,
    method: String,
    csrf_token: String,
    fields: Vec<FormInputViewModel>,
    submit: FormButtonViewModel,
}

#[derive(Serialize)]
struct FormInputViewModel {
    id: String,
    name: String,
    value: String,
    title: String,
    placeholder: String,
    #[serde(rename = "type")]
    input_type: String,
    required: bool,
}

#[derive(Serialize)]
struct FormButtonViewModel {
    id: String,
    title: String,
    #[serde(rename = "type")]
    button_type: String,
}

impl From<Form> for FormViewModel {
    fn from(value: Form) -> Self {
        let fields = value
            .fields()
            .into_iter()
            .map(|field| FormInputViewModel {
                id: field.id().into(),
                name: field.name().into(),
                value: field.value().into(),
                title: field.title().into(),
                placeholder: field.placeholder().into(),
                input_type: field.input_type().into(),
                required: field.required(),
            })
            .collect();

        let submit = FormButtonViewModel {
            id: value.submit().id().into(),
            title: value.submit().title().into(),
            button_type: value.submit().button_type().into(),
        };

        Self {
            id: value.id().into(),
            action: value.action().into(),
            title: value.title().into(),
            method: match value.method() {
                FormMethod::Post => "POST".into(),
                FormMethod::Put => "POST".into(),
                _ => "GET".into(),
            },
            csrf_token: value.csrf_token().into(),
            fields,
            submit,
        }
    }
}
