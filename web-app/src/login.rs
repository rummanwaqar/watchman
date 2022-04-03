use actix_session::Session;
use actix_web::error::InternalError;
use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder, ResponseError};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages, Level};
use secrecy::Secret;
use tera::Context;

use crate::authentication::{validate_credentials, AuthError, Credentials};
use crate::AppState;

pub async fn get(
    data: web::Data<AppState>,
    flash_messages: IncomingFlashMessages,
) -> impl Responder {
    let mut error_message = String::new();
    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        error_message += m.content();
    }
    if error_message.is_empty() {
        error_message = "Please enter your login and password!".to_owned();
    }
    let mut ctx = Context::new();
    ctx.insert("message", error_message.as_str());
    let rendered = data.tera.render("login.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

pub async fn post(
    form: web::Form<FormData>,
    data: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };

    match validate_credentials(credentials, data) {
        Ok(_) => {
            session.renew();
            session
                .insert("username", "admin")
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish())
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            Err(login_redirect(e))
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

fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    let response = HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish();
    InternalError::from_response(e, response)
}
