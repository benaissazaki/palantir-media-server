use crate::server::app_settings::AppSettings;
use std::path::PathBuf;

/// Verifies that the file is in the `media_directories` app setting
///
/// Both the file and the media directories are canonicalized before being compared
/// so that paths containing `..` or symlinks can't escape the media directories
pub fn is_file_in_media_directories(file: PathBuf) -> bool {
    let file = match file.canonicalize() {
        Ok(f) => f,
        Err(_) => return false,
    };

    let media_directories = AppSettings::instance()
        .lock()
        .unwrap()
        .media_directories
        .clone();

    media_directories
        .iter()
        .filter_map(|dir| PathBuf::from(dir).canonicalize().ok())
        .any(|dir| file.starts_with(dir))
}
