use crate::utils::see_other;
use crate::video_file::VideoFile;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use std::fs;
use std::path::Path;
use tera::Context;

pub async fn get(
    data: web::Data<AppState>,
    username: web::ReqData<String>,
    path: web::Path<String>,
) -> impl Responder {
    match VideoFile::new(&path.into_inner(), &data.data_path) {
        Some(video_file) => {
            let mut ctx = Context::new();
            ctx.insert("username", username.into_inner().as_str());
            ctx.insert("video", &video_file);
            let rendered = data.tera.render("video.html", &ctx).unwrap();
            HttpResponse::Ok().body(rendered)
        }
        None => see_other("/admin/"),
    }
}

pub async fn delete(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    if let Some(video_file) = VideoFile::new(&path.into_inner(), &data.data_path) {
        fs::remove_file(video_file.path).unwrap();
        fs::remove_file(Path::new(&data.data_path).join(video_file.image_name)).unwrap();
    }
    see_other("/admin")
}
