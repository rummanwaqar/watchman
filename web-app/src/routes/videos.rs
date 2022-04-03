use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use tera::Context;

pub async fn get(data: web::Data<AppState>, username: web::ReqData<String>) -> impl Responder {
    // for entry in fs::read_dir(config.data_directory)? {
    //     println!("{:?}", entry);
    //     if let Ok(entry) = entry {
    //         println!("{:?}", entry.path().as_path().extension().unwrap());
    //         println!("{:?}", entry.metadata()?.created().unwrap());
    //     }
    // }
    //
    // Ok(())
    let mut ctx = Context::new();
    ctx.insert("username", username.into_inner().as_str());
    let rendered = data.tera.render("videos.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
