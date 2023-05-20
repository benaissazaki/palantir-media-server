#[cfg(test)]
mod tests {
    use crate::media_scanner::{
        routes::get_media_files,
        tests::tests::helpers::{get_actual_response, get_expected_response, setup, teardown},
    };
    use actix_web::{test, App};

    #[actix_web::test]
    async fn get_returns_correct_files() {
        // Create media files and expect the route to return them
        setup();

        let mut app = test::init_service(App::new().service(get_media_files)).await;

        let req = test::TestRequest::get().uri("/media").to_request();
        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), 200);

        let expected_response = get_expected_response();

        let actual_response = get_actual_response(res).await;

        let is_success = expected_response == actual_response;

        teardown();
        assert!(is_success);
    }

    mod helpers {
        use crate::{
            app_settings::AppSettings,
            media_scanner::utils::{MediaFilesResponse, MEDIA_FILES_EXTENSIONS},
        };
        use actix_web::{dev::ServiceResponse, test};
        use std::{
            fs::{self, File},
            path::PathBuf,
        };

        const CREATED_FILES: [&str; 4] = [
            "testdirs/dir1/test.mp4",
            "testdirs/dir1/test.mp3",
            "testdirs/dir2/test.txt",
            "testdirs/dir2/test.mkv",
        ];

        pub async fn get_actual_response(response: ServiceResponse) -> MediaFilesResponse {
            let actual_response_str =
                String::from_utf8((test::read_body(response).await).to_vec()).unwrap();
            let actual_response: MediaFilesResponse =
                serde_json::from_str(actual_response_str.as_str()).unwrap();

            actual_response
        }

        pub fn get_expected_response() -> MediaFilesResponse {
            let expected_response: Vec<String> = filter_media_files(&CREATED_FILES)
                .iter()
                .map(|f| normalize_path(f))
                .collect();

            MediaFilesResponse {
                items: expected_response.clone(),
                length: expected_response.len(),
            }
        }

        pub fn setup() {
            populate_media_dirs().unwrap();
            set_media_dirs().unwrap();
        }

        pub fn teardown() {
            fs::remove_dir_all("testdirs").unwrap();
        }

        pub fn populate_media_dirs() -> Result<(), std::io::Error> {
            for file_path in CREATED_FILES {
                let path = std::path::Path::new(file_path);

                // Create the ancestor directories if they don't exist
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }

                // Create the file if it doesn't exist
                if !path.exists() {
                    File::create(path)?;
                }
            }
            Ok(())
        }

        pub fn set_media_dirs() -> Result<(), Box<dyn std::error::Error>> {
            AppSettings {
                media_directories: vec!["testdirs/dir1".to_string(), "testdirs/dir2".to_string()],
            }
            .save()?;
            Ok(())
        }

        pub fn filter_media_files<'a>(file_paths: &'a [&'a str]) -> Vec<&'a str> {
            // Returns only files whose extensions are in MEDIA_FILES_EXTENSION
            file_paths
                .iter()
                .filter(|&&file_path| {
                    if let Some(extension) = std::path::Path::new(file_path)
                        .extension()
                        .and_then(std::ffi::OsStr::to_str)
                    {
                        MEDIA_FILES_EXTENSIONS.contains(&extension)
                    } else {
                        false
                    }
                })
                .copied()
                .collect()
        }

        pub fn normalize_path(f: &str) -> String {
            // Canonicalize paths to avoid OS specific differences i.e: / or \
            let path = PathBuf::from(f);
            path.canonicalize().unwrap().to_string_lossy().into_owned()
        }
    }
}
