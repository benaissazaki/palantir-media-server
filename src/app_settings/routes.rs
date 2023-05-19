use crate::app_settings::model::{AppSettings};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/setting")]
async fn get_settings() -> impl Responder {
    let settings = AppSettings::load().unwrap_or_default();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(settings.to_string())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SetSettingRequest {
    value: serde_json::Value,
}

#[post("/setting")]
async fn set_settings(
    settings_value: web::Json<AppSettings>,
) -> impl Responder {
    match settings_value.into_inner().save() {
        Ok(_) => return HttpResponse::Created().finish(),
        Err(_) => return HttpResponse::UnprocessableEntity().finish(),
    };
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_settings).service(set_settings);
}
