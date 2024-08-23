use crate::authentication::domain::email::Email;
use crate::authentication::domain::password::Password;
use crate::authentication::domain::username::Username;
use uuid::Uuid;

pub struct User {
    id: Uuid,
    email: Email,
    username: Username,
    password: Password,
    created_at: String,
}

impl User {
    pub fn new(
        id: Uuid,
        email: Email,
        username: Username,
        password: Password,
        created_at: String,
    ) -> Self {
        User {
            id,
            email,
            username,
            password,
            created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::ExposeSecret;

    #[test]
    #[should_panic]
    fn create_user_with_invalid_data_should_fail() {
        let _ = User::new(
            Uuid::new_v4(),
            Email::try_from("admin.com").unwrap(),
            Username::try_from("").unwrap(),
            Password::try_from("12345").unwrap(),
            "created_at".into(),
        );
    }

    #[test]
    fn create_user_with_valid_data_should_succeed() {
        let user = User::new(
            Uuid::new_v4(),
            Email::try_from("admin@admin.com").unwrap(),
            Username::try_from("admin").unwrap(),
            Password::try_from("oz#9k&Auk7DM8KjG3sIz").unwrap(),
            "created_at".into(),
        );

        assert_eq!(user.email.as_ref(), "admin@admin.com");
        assert_eq!(*user.username, "admin");
        assert_eq!(
            user.password.expose_secret().to_string(),
            "oz#9k&Auk7DM8KjG3sIz"
        );
    }
}
