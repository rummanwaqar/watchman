mod routes;
use crate::routes::home;

use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .service(fs::Files::new("/data", "/home/rumman/Videos"))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}