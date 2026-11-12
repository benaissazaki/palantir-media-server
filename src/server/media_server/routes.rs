use std::path::PathBuf;

use actix_files;
use actix_web::{get, web, Either, HttpResponse, Responder};

use crate::server::media_server::utils;

#[get("/media/{media_name}")]
async fn get_media_file(media_file_str: web::Path<String>) -> impl Responder {
    let media_file = PathBuf::from(media_file_str.into_inner());

    if !utils::is_file_in_media_directories(media_file.clone()) {
        return Either::Left(actix_files::NamedFile::open(""));
    }

    // Browsers only support WebVTT subtitles, so SRT files are converted on the fly
    if utils::is_srt_file(&media_file) {
        return Either::Right(match std::fs::read(&media_file) {
            Ok(bytes) => HttpResponse::Ok()
                .content_type("text/vtt; charset=utf-8")
                .body(utils::srt_to_vtt(&utils::decode_text(bytes))),
            Err(_) => HttpResponse::NotFound().finish(),
        });
    }

    Either::Left(actix_files::NamedFile::open(media_file))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_media_file);
}
