use crate::server::{app_settings::AppSettings, media_scanner::utils};
use actix_web::{get, web, HttpResponse, Responder};
use std::path::PathBuf;

use super::utils::MediaFilesResponse;

/// Returns a list of media files in the directories
/// specified in the `media_directories` app setting
#[get("/media")]
async fn get_media_files() -> impl Responder {
    let settings = AppSettings::instance().lock().unwrap();

    let media_directories = settings.media_directories.clone();

    let media_files: Vec<String> = media_directories
        .into_iter()
        .flat_map(|dir| utils::scan_for_media_files(PathBuf::from(dir)))
        .collect();

    let response = MediaFilesResponse {
        length: media_files.clone().len(),
        items: media_files,
    };

    drop(settings);
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&response).unwrap_or_default());
}

#[get("/subtitles/{directory}")]
async fn get_subtitles_in_dir(directory: web::Path<String>) -> impl Responder {
    let subtitles = utils::scan_for_subtitles_in_dir(PathBuf::from(directory.to_string()));
    let response = MediaFilesResponse {
        length: subtitles.clone().len(),
        items: subtitles,
    };
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&response).unwrap_or_default());
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_media_files);
    cfg.service(get_subtitles_in_dir);
}
