use actix_web::{get, post, web, HttpResponse, Responder};

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

#[post("/setting/{setting_name}")]
pub async fn set_setting(
    setting_name: web::Path<String>,
    setting_value: web::Json<serde_json::Value>,
) -> impl Responder {
    match settings::set_setting(setting_name.into_inner(), setting_value.into_inner()["value"].clone()) {
        Ok(_) => return HttpResponse::Created().finish(),
        Err(_) => return HttpResponse::UnprocessableEntity().finish(),
    };
}
