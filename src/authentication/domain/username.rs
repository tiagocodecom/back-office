use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> anyhow::Result<Self> {
        if username.len() < 5 {
            return Err(anyhow::anyhow!("Username must have at least 5 characters"));
        }

        if username.contains(' ') {
            return Err(anyhow::anyhow!("Username must not contain spaces"));
        }

        Ok(Self(username))
    }
}

impl TryFrom<&str> for Username {
    type Error = anyhow::Error;

    fn try_from(username: &str) -> Result<Self, Self::Error> {
        Self::new(username.into())
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Username {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::assert_err;

    #[test]
    fn creation_fails_with_empty_string() {
        let username = Username::try_from("");
        assert_err!(username);
    }

    #[test]
    #[should_panic]
    fn must_have_at_least_5_characters() {
        let _ = Username::try_from("ab").unwrap();
    }

    #[test]
    #[should_panic]
    fn must_not_contain_spaces() {
        _ = Username::try_from("user name").unwrap();
    }

    #[test]
    fn should_be_able_to_compare_usernames() {
        let first_username = Username::try_from("first_username").unwrap();
        let second_username = Username::try_from("second_username").unwrap();
        assert_ne!(first_username, second_username);
    }
}
