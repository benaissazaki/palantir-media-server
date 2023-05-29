use crate::app_settings::AppSettings;
use std::path::PathBuf;

/// Verifies that the file is in the `media_directories` app setting
pub fn is_file_in_media_directories(file: PathBuf) -> bool {
    if is_attempting_directory_traversal(file.clone()) {
        return false;
    }

    let media_directories = &AppSettings::instance().lock().unwrap().media_directories;

    media_directories
        .into_iter()
        .any(|dir| file.starts_with(dir))
}

fn is_attempting_directory_traversal(path: PathBuf) -> bool {
    // Check for any occurrences of ".." in the file path
    path.components().any(|component| {
        if let std::path::Component::Normal(path) = component {
            path == ".."
        } else {
            false
        }
    })
}
