use std::path::PathBuf;

use actix_files;
use actix_web::{get, web, HttpResponse, Responder};

use crate::{directory_scanner, app_settings};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/* === MEDIA FILES === */
#[get("/media")]
pub async fn get_media_files() -> impl Responder {
    let media_directories: Vec<PathBuf> =
        match app_settings::get_setting("media_directories".to_string()) {
            Some(p) => p
                .as_array()
                .unwrap()
                .into_iter()
                .map(|v| PathBuf::from(v.as_str().unwrap()))
                .collect(),
            None => return HttpResponse::NoContent().finish(),
        };

    let media_files: Vec<PathBuf> = media_directories
        .into_iter()
        .flat_map(directory_scanner::scan_for_media_files)
        .collect();

    let response = serde_json::json!({
        "length": media_files.len(),
        "items": media_files
    });

    return HttpResponse::Ok()
        .content_type("application/json")
        .body(response.to_string());
}

#[get("/media/{media_name}")]
pub async fn get_media_file(
    media_file_str: web::Path<String>,
) -> impl Responder {
    let media_file = PathBuf::from(media_file_str.into_inner());

    match directory_scanner::is_file_in_media_directories(media_file.clone()) {
        true => actix_files::NamedFile::open(media_file),
        false => actix_files::NamedFile::open(""),
    }

}
