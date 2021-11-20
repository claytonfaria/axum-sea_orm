use super::{dto::LoginInput, error::AuthError};

pub struct AuthService;

impl AuthService {
    pub async fn sign_in(input: LoginInput) -> Result<String, AuthError> {
        let user_email = input.email;

        if user_email == "claytonfaria" {
            Ok(user_email)
        } else {
            Err(AuthError::WrongCredentials)
        }
    }
}
