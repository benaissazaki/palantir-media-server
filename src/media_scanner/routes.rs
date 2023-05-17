use std::path::PathBuf;
use actix_web::{get, web, HttpResponse, Responder};

use crate::{app_settings, media_scanner::utils};

#[get("/media")]
async fn get_media_files() -> impl Responder {
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
        .flat_map(utils::scan_for_media_files)
        .collect();

    let response = serde_json::json!({
        "length": media_files.len(),
        "items": media_files
    });

    return HttpResponse::Ok()
        .content_type("application/json")
        .body(response.to_string());
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_media_files);
}
