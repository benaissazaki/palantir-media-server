use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use std::path::PathBuf;

use crate::server::media_server::utils::is_file_in_media_directories;

use super::utils::{self, Capabilities, PlaybackSupport, CAPABILITIES_COOKIE};

/// Tells whether the browser can play a media file, based on the containers and codecs
/// it listed in the capabilities cookie and on the content of the file
#[get("/playback/{media_name}")]
async fn get_playback_support(
    media_file_str: web::Path<String>,
    request: HttpRequest,
) -> impl Responder {
    let media_file = PathBuf::from(media_file_str.into_inner());

    if !is_file_in_media_directories(media_file.clone()) {
        return HttpResponse::NotFound().finish();
    }

    let capabilities = request
        .cookie(CAPABILITIES_COOKIE)
        .map(|cookie| Capabilities::parse(cookie.value()));

    // Reading the file blocks, so it is done outside of the server's async threads
    let info = web::block(move || utils::probe_cached(&media_file))
        .await
        .ok()
        .flatten();

    let support = match (info, capabilities) {
        (Some(info), Some(capabilities)) => utils::check_playback_support(&info, &capabilities),
        // Without the information needed to decide, let the browser try
        _ => PlaybackSupport::playable(),
    };

    HttpResponse::Ok().json(support)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_playback_support);
}
