use crate::{app_settings::AppSettings, media_scanner::utils};
use actix_web::{get, web, HttpResponse, Responder};
use std::path::PathBuf;

use super::utils::MediaFilesResponse;

/// Returns a list of media files in the directories
/// specified in the `media_directories` app setting
#[get("/media")]
async fn get_media_files() -> impl Responder {
    let media_directories = AppSettings::load().unwrap_or_default().media_directories;

    let media_files: Vec<String> = media_directories
        .into_iter()
        .flat_map(|dir| utils::scan_for_media_files(PathBuf::from(dir)))
        .collect();

    let response = MediaFilesResponse {
        length: media_files.clone().len(),
        items: media_files,
    };

    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&response).unwrap_or_default());
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_media_files);
}
