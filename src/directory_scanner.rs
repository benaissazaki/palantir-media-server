use std::fs;
use std::path::PathBuf;

use crate::settings;

const MEDIA_FILES_EXTENSIONS: &'static [&'static str] = &["mp3", "mp4", "avi", "wav", "mkv"];

pub fn scan_for_media_files(dir_path: PathBuf) -> Vec<PathBuf> {
    let mut media_files = Vec::new();

    for entry in fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            media_files.extend(scan_for_media_files(path));
        } else {
            if let Some(ext) = path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    if MEDIA_FILES_EXTENSIONS.contains(&ext_str) {
                        media_files.push(path);
                    }
                }
            }
        }
    }
    media_files
}

pub fn is_file_in_media_directories(file: PathBuf) -> bool {
    if is_attempting_directory_traversal(file.clone()) {
        return false;
    }
    
    let media_directories: Vec<PathBuf> =
        match settings::get_setting("media_directories".to_string()) {
            Some(p) => p
                .as_array()
                .unwrap()
                .into_iter()
                .map(|v| PathBuf::from(v.as_str().unwrap()))
                .collect(),
            None => return false,
        };

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
