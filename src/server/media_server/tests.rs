#[cfg(test)]
mod tests {
    use crate::server::{app_settings::AppSettings, media_server::routes::get_media_file};
    use actix_web::{test, App};
    use std::fs::{self, File};

    fn setup() {
        fs::create_dir_all("testdirs/media_dir").unwrap();
        File::create("testdirs/media_dir/test.mp4").unwrap();
        File::create("testdirs/secret.txt").unwrap();

        let mut settings = AppSettings::instance().lock().unwrap();
        settings.media_directories = vec!["testdirs/media_dir".to_string()];
        settings.save().unwrap();
    }

    fn teardown() {
        fs::remove_dir_all("testdirs").unwrap();
    }

    async fn get_status(file: &str) -> u16 {
        let app = test::init_service(App::new().service(get_media_file)).await;
        let req = test::TestRequest::get()
            .uri(format!("/media/{}", file).as_str())
            .to_request();
        test::call_service(&app, req).await.status().as_u16()
    }

    /// Serves a file located in a media directory
    #[actix_web::test]
    async fn get_media_file_serves_file_in_media_directories() {
        setup();
        let status = get_status("testdirs%2Fmedia_dir%2Ftest.mp4").await;
        teardown();

        assert_eq!(status, 200);
    }

    /// Refuses to serve a file outside of the media directories using '..'
    #[actix_web::test]
    async fn get_media_file_refuses_directory_traversal() {
        setup();
        let status = get_status("testdirs%2Fmedia_dir%2F..%2Fsecret.txt").await;
        teardown();

        assert_eq!(status, 404);
    }

    /// Refuses to serve a file outside of the media directories
    #[actix_web::test]
    async fn get_media_file_refuses_file_outside_media_directories() {
        setup();
        let status = get_status("testdirs%2Fsecret.txt").await;
        teardown();

        assert_eq!(status, 404);
    }
}
