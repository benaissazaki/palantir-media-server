use std::fs;
use std::path::PathBuf;

const MEDIA_FILES_EXTENSIONS: &'static [&'static str] = &["mp3", "mp4", "avi", "wav", "mkv"];

pub fn scan_for_media_files(dir_path: PathBuf) -> Vec<PathBuf> {
    let mut media_files = Vec::new();

    let read_dir = match fs::read_dir(dir_path) {
        Ok(r) => r,
        Err(_) => return vec![],
    };

    for entry in read_dir {
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
