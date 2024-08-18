use crate::authentication::domain::form::Form;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct FormInputViewModel {
    id: String,
    name: String,
    value: String,
    title: String,
    #[serde(rename = "type")]
    input_type: String,
}

#[derive(Serialize, Deserialize)]
struct FormButtonViewModel {
    id: String,
    title: String,
    #[serde(rename = "type")]
    button_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormViewModel {
    title: String,
    fields: Vec<FormInputViewModel>,
    submit: FormButtonViewModel,
}

impl From<Form> for FormViewModel {
    fn from(value: Form) -> Self {
        let fields = value
            .fields
            .into_iter()
            .map(|field| FormInputViewModel {
                id: field.id,
                name: field.name,
                value: field.value,
                title: field.title,
                input_type: field.input_type,
            })
            .collect();
        let submit = FormButtonViewModel {
            id: value.submit.id,
            title: value.submit.title,
            button_type: value.submit.button_type,
        };
        Self {
            title: value.title,
            fields,
            submit,
        }
    }
}
