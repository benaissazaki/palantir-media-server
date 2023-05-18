use crate::app_settings::utils;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/setting")]
async fn get_settings() -> impl Responder {
    let settings = utils::Settings::load();

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
    settings_value: web::Json<utils::Settings>,
) -> impl Responder {
    match settings_value.into_inner().save() {
        Ok(_) => return HttpResponse::Created().finish(),
        Err(_) => return HttpResponse::UnprocessableEntity().finish(),
    };
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_settings).service(set_settings);
}
