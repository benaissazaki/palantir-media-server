#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use serde_json::json;
    use std::net::SocketAddr;

    use crate::server::app_settings::{
        routes::{get_settings, set_settings},
        AppSettings,
    };

    fn local_addr() -> SocketAddr {
        "127.0.0.1:12345".parse().unwrap()
    }

    #[actix_web::test]
    async fn post_succeeds_with_valid_data() {
        let mut app = test::init_service(App::new().service(set_settings)).await;

        let req = test::TestRequest::post()
            .uri("/setting")
            .peer_addr(local_addr())
            .set_json(json!({ "media_directories": ["testdir/"]}))
            .to_request();

        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), 201);
    }

    #[actix_web::test]
    async fn post_fails_with_invalid_data() {
        let mut app = test::init_service(App::new().service(set_settings)).await;

        let req = test::TestRequest::post()
            .uri("/setting")
            .peer_addr(local_addr())
            .set_json(json!({ "medi_directories": ["testdir/"]}))
            .to_request();

        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), 400);
    }

    #[actix_web::test]
    async fn post_fails_from_remote_address() {
        let mut app = test::init_service(App::new().service(set_settings)).await;

        let req = test::TestRequest::post()
            .uri("/setting")
            .peer_addr("192.168.1.20:12345".parse().unwrap())
            .set_json(json!({ "media_directories": ["/"]}))
            .to_request();

        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), 403);
    }

    #[actix_web::test]
    async fn get_returns_correct_data() {
        let mut app = test::init_service(App::new().service(get_settings)).await;

        let mut new_settings = AppSettings::instance().lock().unwrap();
        new_settings.media_directories = vec!["test/".to_string(), "test2".to_string()];
        new_settings.save().unwrap();

        let new_settings_str = new_settings.to_string();
        drop(new_settings);
        
        let req = test::TestRequest::get().uri("/setting").to_request();

        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), 200);

        let body = test::read_body(res).await;

        
        assert_eq!(body, new_settings_str);
    }
}
