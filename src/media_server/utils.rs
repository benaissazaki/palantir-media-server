use std::fs;
use std::path::PathBuf;

use crate::app_settings;

pub fn is_file_in_media_directories(file: PathBuf) -> bool {
    if is_attempting_directory_traversal(file.clone()) {
        return false;
    }
    
    let media_directories = app_settings::Settings::load().media_directories;

    media_directories
        .into_iter()
        .any(|dir| file.starts_with(dir))
}

fn is_attempting_directory_traversal(path: PathBuf) -> bool {
    let canonical_path = match fs::canonicalize(&path) {
        Ok(path) => path,
        Err(_) => return true, // Unable to canonicalize, assume directory traversal
    };

    path != canonical_path
}
