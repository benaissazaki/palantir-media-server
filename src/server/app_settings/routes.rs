use crate::server::app_settings::model::AppSettings;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

#[get("/setting")]
async fn get_settings() -> impl Responder {
    let settings = AppSettings::instance().lock().unwrap();

    let settings_str = settings.to_string();

    drop(settings);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(settings_str)
}

/// Updates the settings. Only allowed from the machine running the server
/// so that other devices on the network can't change the media directories
#[post("/setting")]
async fn set_settings(
    req: HttpRequest,
    settings_value: web::Json<AppSettings>,
) -> impl Responder {
    let is_local_request = req
        .peer_addr()
        .map(|addr| addr.ip().is_loopback())
        .unwrap_or(false);

    if !is_local_request {
        return HttpResponse::Forbidden().finish();
    }

    let mut settings = AppSettings::instance().lock().unwrap();

    settings.media_directories = settings_value.media_directories.clone();
    match settings.save() {
        Ok(_) => {
            drop(settings);
            return HttpResponse::Created().finish()},
        Err(_) => {
            drop(settings);
            return HttpResponse::UnprocessableEntity().finish()
        },
    };
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_settings).service(set_settings);
}
