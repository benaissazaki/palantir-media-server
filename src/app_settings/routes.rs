use actix_web::{get, post, web, HttpResponse, Responder};
use crate::app_settings::utils;

#[get("/setting/{setting_name}")]
async fn get_setting(setting_name: web::Path<String>) -> impl Responder {
    let response = serde_json::json!({
        "value": utils::get_setting(setting_name.into_inner()),
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response.to_string())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SetSettingRequest {
    value: serde_json::Value,
}

#[post("/setting/{setting_name}")]
async fn set_setting(
    setting_name: web::Path<String>,
    setting_value: web::Json<SetSettingRequest>,
) -> impl Responder {
    match utils::set_setting(setting_name.into_inner(), setting_value.into_inner().value) {
        Ok(_) => return HttpResponse::Created().finish(),
        Err(_) => return HttpResponse::UnprocessableEntity().finish(),
    };
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_setting).service(set_setting);
}
