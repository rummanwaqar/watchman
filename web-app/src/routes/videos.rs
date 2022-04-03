use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use tera::Context;

pub async fn get(data: web::Data<AppState>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("username", "adminis");
    let rendered = data.tera.render("videos.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
