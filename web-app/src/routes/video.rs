use crate::utils::see_other;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;
use tera::Context;

pub async fn get(
    data: web::Data<AppState>,
    username: web::ReqData<String>,
    path: web::Path<String>,
) -> impl Responder {
    let filename = path.into_inner();
    let path = Path::new(data.data_path.as_str()).join(&filename);
    if !path.exists() && path.extension().unwrap() == "mp4" {
        return see_other("/admin/");
    }
    let date_time: DateTime<Utc> = fs::metadata(&path).unwrap().created().unwrap().into();
    let mut ctx = Context::new();
    ctx.insert("username", username.into_inner().as_str());
    ctx.insert("filename", filename.as_str());
    ctx.insert("date_time", date_time.format("%v %r").to_string().as_str());
    let rendered = data.tera.render("video.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn delete(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    let path = Path::new(data.data_path.as_str()).join(&filename);
    if !path.exists() && path.extension().unwrap() == "mp4" {
        return see_other("/admin/");
    }
    fs::remove_file(path);
    see_other("/admin")
}
