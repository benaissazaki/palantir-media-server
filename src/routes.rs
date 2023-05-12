use actix_web::{get, post, web, HttpResponse, Responder};
use serde;

use crate::settings;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/* === SETTINGS === */
#[get("/setting/{setting_name}")]
pub async fn get_setting(setting_name: web::Path<String>) -> impl Responder {
    let response = serde_json::json!({
        "value": settings::get_setting(setting_name.into_inner()),
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response.to_string())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SetSettingRequest {
    value: serde_json::Value,
}

#[post("/setting/{setting_name}")]
pub async fn set_setting(
    setting_name: web::Path<String>,
    setting_value: web::Json<SetSettingRequest>,
) -> impl Responder {
    match settings::set_setting(setting_name.into_inner(), setting_value.into_inner().value) {
        Ok(_) => return HttpResponse::Created().finish(),
        Err(_) => return HttpResponse::UnprocessableEntity().finish(),
    };
}
