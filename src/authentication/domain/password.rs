use secrecy::{ExposeSecret, Secret};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Password(Secret<String>);

impl Password {
    pub fn new(password: String) -> anyhow::Result<Self> {
        if Self::is_safe(&password) {
            return Ok(Self(Secret::new(password)));
        }

        anyhow::bail!("It must be 15+ characters with a digit, uppercase, lowercase, and symbol")
    }

    fn is_safe(password: &str) -> bool {
        password.len() >= 15
            && Self::has_digit(password)
            && Self::has_uppercase(password)
            && Self::has_lowercase(password)
            && Self::has_symbol(password)
    }

    fn has_digit(password: &str) -> bool {
        password.chars().any(|c| c.is_digit(10))
    }

    fn has_uppercase(password: &str) -> bool {
        password.chars().any(|c| c.is_uppercase())
    }

    fn has_lowercase(password: &str) -> bool {
        password.chars().any(|c| c.is_lowercase())
    }

    fn has_symbol(password: &str) -> bool {
        password.chars().any(|c| c.is_ascii_punctuation())
    }

    pub fn expose_password(&self) -> &str {
        self.0.expose_secret()
    }
}

impl TryFrom<&str> for Password {
    type Error = anyhow::Error;

    fn try_from(password: &str) -> Result<Self, Self::Error> {
        Self::new(password.to_string())
    }
}

impl Deref for Password {
    type Target = Secret<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    #[should_panic]
    fn weak_passwords_should_fail() {
        let test_cases = vec![
            ("12345", "Password is too short"),
            ("Abc#123", "Password is too short"),
            ("Abcdefghijklmn#", "Password is missing a digit"),
            ("ABC#DEFGHIJK1MN", "Password is missing a lowercase letter"),
            ("abc#defghijk1mnO", "Password is missing a symbol"),
        ];

        for (password, error_message) in test_cases {
            let password = Password::try_from(password);
            assert_err!(password, "{}", error_message);
        }
    }

    #[test]
    fn strong_password_contains_15_characters_symbols_numbers_letters_uppercase_and_lowercase() {
        let password = Password::try_from("oz#9k&Auk7DM8KjG3sIz");
        assert_ok!(password.as_ref());
        assert_eq!(
            password.unwrap().expose_password().to_string(),
            "oz#9k&Auk7DM8KjG3sIz"
        );
    }
}
