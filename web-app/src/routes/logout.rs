use crate::utils::see_other;
use actix_session::Session;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

pub async fn post(session: Session) -> Result<HttpResponse, actix_web::Error> {
    session.purge();
    FlashMessage::error("You have successfully logged out.").send();
    return Ok(see_other("/login"));
}
