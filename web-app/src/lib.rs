use actix_files as fs;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use secrecy::{ExposeSecret, Secret};
use std::net::TcpListener;
use tera::Tera;

mod authentication;
pub mod configuration;
mod login;
mod logout;
mod videos;

pub struct AppState {
    tera: Tera,
    username: String,
    password: Secret<String>,
}

pub async fn run(
    listener: TcpListener,
    config: configuration::Settings,
) -> Result<(), std::io::Error> {
    let key = Key::from(config.secret_key.expose_secret().as_bytes());
    let message_framework =
        FlashMessagesFramework::builder(CookieMessageStore::builder(key.clone()).build()).build();
    let redis_store = RedisSessionStore::new(config.redis_uri)
        .await
        .expect("Unable to open redis");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                tera: Tera::new("templates/**/*").unwrap(),
                username: config.username.clone(),
                password: config.password.clone(),
            }))
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(redis_store.clone(), key.clone()))
            .route("/", web::get().to(videos::get))
            .route("/login", web::get().to(login::get))
            .route("/login", web::post().to(login::post))
            .route("/logout", web::post().to(logout::post))
            .service(fs::Files::new("/static", "static"))
            .service(fs::Files::new("/data", config.data_directory.clone()))
    })
    .listen(listener)?
    .run()
    .await?;
    Ok(())
}
