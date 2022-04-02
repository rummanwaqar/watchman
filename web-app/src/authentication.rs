use secrecy::{ExposeSecret, Secret};
use std::fmt::Error;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct Credentials {
    // These two fields were not marked as `pub` before!
    pub username: String,
    pub password: Secret<String>,
}

pub fn validate_credentials(credentials: Credentials) -> Result<(), AuthError> {
    if credentials.username == "admin" && credentials.password.expose_secret() == "password" {
        Ok(())
    } else {
        Err(AuthError::InvalidCredentials(anyhow::Error::new(
            Error::default(),
        )))
    }
}
