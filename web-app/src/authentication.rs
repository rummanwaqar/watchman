use crate::utils::{e500, see_other};
use crate::AppState;
use actix_session::Session;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::{web, FromRequest, HttpMessage};
use actix_web_lab::middleware::Next;
use secrecy::{ExposeSecret, Secret};
use std::fmt::Error;

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        Session::from_request(http_request, payload).await
    }?;

    match session.get::<String>("username").map_err(e500)? {
        Some(username) => {
            req.extensions_mut().insert(username);
            next.call(req).await
        }
        None => {
            let response = see_other("/login");
            let e = anyhow::anyhow!("The user has not logged in");
            Err(InternalError::from_response(e, response).into())
        }
    }
}

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
