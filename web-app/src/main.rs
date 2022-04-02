mod routes;

use crate::routes::{home, login, login_form};

mod authentication;

use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use actix_web::cookie::Key;
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key = "my-very-long-key-should-be-removed-from-here-and-moved-to-application-settings";
    let message_store = CookieMessageStore::builder(Key::from(key.as_bytes())).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .route("/", web::get().to(home))
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            .service(fs::Files::new("/data", "/home/rumman/Videos"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
