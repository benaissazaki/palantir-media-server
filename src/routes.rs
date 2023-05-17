use std::path::PathBuf;

use actix_files;
use actix_web::{get, web, HttpResponse, Responder};

use crate::directory_scanner;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/* === MEDIA FILES === */
#[get("/media/{media_name}")]
pub async fn get_media_file(media_file_str: web::Path<String>) -> impl Responder {
    let media_file = PathBuf::from(media_file_str.into_inner());

    match directory_scanner::is_file_in_media_directories(media_file.clone()) {
        true => actix_files::NamedFile::open(media_file),
        false => actix_files::NamedFile::open(""),
    }
}
