use crate::AppState;
use actix_web::web;
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
    pub username: String,
    pub password: Secret<String>,
}

pub fn validate_credentials(
    credentials: Credentials,
    data: web::Data<AppState>,
) -> Result<(), AuthError> {
    if credentials.username == data.username
        && credentials.password.expose_secret() == data.password.expose_secret()
    {
        Ok(())
    } else {
        Err(AuthError::InvalidCredentials(anyhow::Error::new(
            Error::default(),
        )))
    }
}
