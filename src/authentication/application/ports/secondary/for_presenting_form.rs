use crate::authentication::domain::form::Form;

pub trait ForPresentingForm {
    fn render(&self, template: &str, context: Form) -> String;
}
