use std::path::PathBuf;

use actix_files;
use actix_web::{get, web, Responder};

use crate::server::media_server::utils;

#[get("/media/{media_name}")]
async fn get_media_file(media_file_str: web::Path<String>) -> impl Responder {
    let media_file = PathBuf::from(media_file_str.into_inner());

    match utils::is_file_in_media_directories(media_file.clone()) {
        true => actix_files::NamedFile::open(media_file),
        false => actix_files::NamedFile::open(""),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_media_file);
}
