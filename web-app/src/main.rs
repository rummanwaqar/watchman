mod routes;

use crate::routes::{admin_dashboard, home, login, login_form};

mod authentication;

use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use actix_web::cookie::Key;
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;

use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key =
        "my-very-long-key-should-be-removed-from-here-and-moved-to-application-settings".as_bytes();
    let message_store = CookieMessageStore::builder(Key::from(key.clone())).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();

    let redis_uri = "redis://127.0.0.1:6379";
    let redis_store = RedisSessionStore::new(redis_uri)
        .await
        .expect("Unable to open redis");

    HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                Key::from(key.clone()),
            ))
            .route("/", web::get().to(home))
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            .route("/admin/dashboard", web::get().to(admin_dashboard))
            .service(fs::Files::new("/data", "/home/rumman/Videos"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
