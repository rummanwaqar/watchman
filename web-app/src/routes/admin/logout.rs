use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub async fn log_out(session: Session) -> Result<HttpResponse, actix_web::Error> {
    if let Some(user_id) = session.get::<String>("user_id").map_err(e500)? {
        session.purge();
        FlashMessage::error("You have successfully logged out.").send();
    }
    return Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish());
}
