use crate::authentication::reject_anonymous_users;
use actix_files as fs;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_lab::middleware::from_fn;
use secrecy::{ExposeSecret, Secret};
use std::net::TcpListener;
use tera::Tera;

mod authentication;
pub mod configuration;
mod routes;
mod utils;
mod video_file;

pub struct AppState {
    tera: Tera,
    username: String,
    password: Secret<String>,
    data_path: String,
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
                data_path: config.data_directory.clone(),
            }))
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(redis_store.clone(), key.clone()))
            .route("/login", web::get().to(routes::login::get))
            .route("/login", web::post().to(routes::login::post))
            .service(
                web::scope("/admin")
                    .wrap(from_fn(reject_anonymous_users))
                    .route("/", web::get().to(routes::videos::get))
                    .route("/video/{id}", web::get().to(routes::video::get))
                    .route("/video/delete/{id}", web::post().to(routes::video::delete))
                    .route("/logout", web::post().to(routes::logout::post))
                    .service(fs::Files::new("/data", config.data_directory.clone())),
            )
            .service(fs::Files::new("/static", "static"))
            .default_service(web::get().to(routes::login::get))
    })
    .listen(listener)?
    .run()
    .await?;
    Ok(())
}
