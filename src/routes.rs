use actix_web::{get, web, HttpResponse, Responder};

use crate::settings;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/* === SETTINGS === */
#[get("/setting/{setting_name}")]
pub async fn get_setting(setting_name: web::Path<String>) -> impl Responder {
    let response = serde_json::json!({
        "value": settings::get_setting(setting_name.to_string()),
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response.to_string())
}
