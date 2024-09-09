use crate::common::entities::Form;

pub enum LoginProviderOutput {
    // The default login provider is a form with email and password.
    Default(Form),
    // Ideally we would add more providers here, like Google, Facebook, etc.
    // ...
}
