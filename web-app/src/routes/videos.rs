use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use tera::Context;

pub async fn get(data: web::Data<AppState>, username: web::ReqData<String>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("username", username.into_inner().as_str());
    let rendered = data.tera.render("videos.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
