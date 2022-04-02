use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::{HttpResponse, ResponseError};
use actix_web_flash_messages::FlashMessage;
use secrecy::Secret;
use std::fmt::{Display, Error, Formatter};

use crate::authentication::{validate_credentials, AuthError, Credentials};

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

pub async fn login(form: web::Form<FormData>) -> Result<HttpResponse, LoginError> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };

    match validate_credentials(credentials) {
        Ok(_) => Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish()),
        Err(e) => {
            FlashMessage::error(e.to_string()).send();
            Ok(HttpResponse::SeeOther()
                // No cookies here now!
                .insert_header((LOCATION, "/login"))
                .finish())
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header((LOCATION, "/login"))
            .finish()
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::SEE_OTHER
    }
}
