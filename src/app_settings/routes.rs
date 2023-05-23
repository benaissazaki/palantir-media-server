use crate::app_settings::model::{AppSettings};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/setting")]
async fn get_settings() -> impl Responder {
    let settings = AppSettings::instance().lock().unwrap();

    let settings_str = settings.to_string();

    drop(settings);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(settings_str)
}

#[post("/setting")]
async fn set_settings(
    settings_value: web::Json<AppSettings>,
) -> impl Responder {
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
