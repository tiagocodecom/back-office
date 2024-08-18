use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use validator::ValidateEmail;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> anyhow::Result<Self> {
        if ValidateEmail::validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(anyhow::anyhow!("Invalid email"))
        }
    }
}

impl TryFrom<&str> for Email {
    type Error = anyhow::Error;

    fn try_from(email: &str) -> Result<Self, Self::Error> {
        Self::new(email.to_string())
    }
}

impl Deref for Email {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn create_email_with_invalid_data_should_fail() {
        let _ = Email::try_from("email").unwrap();
    }

    #[test]
    fn create_email_with_valid_data_should_succeed() {
        let email = Email::try_from("admin@admin.com").unwrap();
        assert_eq!(*email, "admin@admin.com");
    }

    #[test]
    fn email_should_be_comparable() {
        let first_email = Email::try_from("first_admin@admin.com").unwrap();
        let second_email = Email::try_from("second_admin@admin.com").unwrap();
        assert_ne!(first_email, second_email);
    }
}
