#[cfg(test)]
mod tests {
    use crate::server::media_scanner::{
        routes::{get_media_files, get_subtitles_in_dir},
        tests::tests::helpers::{get_media_actual_response, get_subtitles_actual_response, get_subtitles_expected_response, get_media_expected_response, setup, teardown},
    };
    use actix_web::{test, App};

    /// Creates media files and expects the route to return them
    #[actix_web::test]
    async fn get_media_returns_correct_files() {
        setup();

        let mut app = test::init_service(App::new().service(get_media_files)).await;

        let req = test::TestRequest::get().uri("/media").to_request();
        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), 200);

        let expected_response = get_media_expected_response();

        let actual_response = get_media_actual_response(res).await;

        teardown();
        assert_eq!(expected_response, actual_response);
    }

    // Creates subtitle files and expects the route to return them
    #[actix_web::test]
    async fn get_subtitles_returns_correct_files() {
        setup();

        let mut app = test::init_service(App::new().service(get_subtitles_in_dir)).await;

        let scanned_dir = "testdirs%2Fdir1";
        let req = test::TestRequest::get().uri(format!("/subtitles/{}", scanned_dir).as_str()).to_request();
        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), 200);

        let expected_response = get_subtitles_expected_response();

        let actual_response = get_subtitles_actual_response(res).await;

        teardown();
        assert_eq!(expected_response, actual_response);
    }

    mod helpers {
        use crate::{
            server::app_settings::AppSettings,
            server::media_scanner::utils::{MediaFilesResponse, MEDIA_FILES_EXTENSIONS, SUBTITLES_EXTENSION},
        };
        use actix_web::{dev::ServiceResponse, test};
        use std::{
            fs::{self, File},
            path::PathBuf,
        };

        const CREATED_FILES: [&str; 5] = [
            "testdirs/dir1/test.mp4",
            "testdirs/dir1/test.mp3",
            "testdirs/dir1/sub.srt",
            "testdirs/dir2/test.txt",
            "testdirs/dir2/test.mkv",
        ];

        pub fn setup() {
            populate_media_dirs().unwrap();
            set_media_dirs_setting().unwrap();
        }

        pub fn teardown() {
            fs::remove_dir_all("testdirs").unwrap();
        }

        pub async fn get_media_actual_response(response: ServiceResponse) -> MediaFilesResponse {
            let actual_response_str =
                String::from_utf8((test::read_body(response).await).to_vec()).unwrap();
            let actual_response: MediaFilesResponse =
                serde_json::from_str(actual_response_str.as_str()).unwrap();

            actual_response
        }

        pub async fn get_subtitles_actual_response(response: ServiceResponse) -> MediaFilesResponse {
            let actual_response_str =
                String::from_utf8((test::read_body(response).await).to_vec()).unwrap();
            let actual_response: MediaFilesResponse =
                serde_json::from_str(actual_response_str.as_str()).unwrap();

            actual_response
        }

        pub fn get_media_expected_response() -> MediaFilesResponse {
            let expected_response: Vec<String> = filter_media_files(&CREATED_FILES)
                .iter()
                .map(|f| normalize_path(f))
                .collect();

            MediaFilesResponse {
                items: expected_response.clone(),
                length: expected_response.len(),
            }
        }

        pub fn get_subtitles_expected_response() -> MediaFilesResponse {
            let expected_response: Vec<String> = filter_subtitle_files(&CREATED_FILES)
                .iter()
                .map(|f| normalize_path(f))
                .collect();

            MediaFilesResponse {
                items: expected_response.clone(),
                length: expected_response.len(),
            }
        }

        /// Create fake media files for testing purposes
        pub fn populate_media_dirs() -> Result<(), std::io::Error> {
            for file_path in CREATED_FILES {
                let path = std::path::Path::new(file_path);

                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }

                if !path.exists() {
                    File::create(path)?;
                }
            }
            Ok(())
        }

        pub fn set_media_dirs_setting() -> Result<(), Box<dyn std::error::Error>> {
            let mut settings = AppSettings::instance().lock().unwrap();

            settings.media_directories =  vec!["testdirs/dir1".to_string(), "testdirs/dir2".to_string()];
            settings.save()?;
            drop(settings);
            Ok(())
        }

        /// Returns only files whose extensions are in `MEDIA_FILES_EXTENSION`
        pub fn filter_media_files<'a>(file_paths: &'a [&'a str]) -> Vec<&'a str> {
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

        /// Returns only files whose extensions are in `SUBTITLES_EXTENSION`
        pub fn filter_subtitle_files<'a>(file_paths: &'a [&'a str]) -> Vec<&'a str> {
            file_paths
                .iter()
                .filter(|&&file_path| {
                    if let Some(extension) = std::path::Path::new(file_path)
                        .extension()
                        .and_then(std::ffi::OsStr::to_str)
                    {
                        SUBTITLES_EXTENSION.contains(&extension)
                    } else {
                        false
                    }
                })
                .copied()
                .collect()
        }

        /// Canonicalize paths to avoid OS specific differences i.e: / or \
        pub fn normalize_path(f: &str) -> String {    
            let path = PathBuf::from(f);
            path.canonicalize().unwrap().to_string_lossy().into_owned().replace("\\\\?\\", "")
        }
    }
}
