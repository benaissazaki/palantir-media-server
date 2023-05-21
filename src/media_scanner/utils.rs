use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

pub const MEDIA_FILES_EXTENSIONS: &'static [&'static str] = &["mp3", "mp4", "avi", "wav", "mkv"];

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaFilesResponse {
    pub length: usize,
    pub items: Vec<String>,
}

impl PartialEq for MediaFilesResponse {
    fn eq(&self, other: &Self) -> bool {
        let self_items_set: HashSet<&String> = self.items.iter().collect();
        let other_items_set: HashSet<&String> = other.items.iter().collect();

        self_items_set == other_items_set
    }
}

/// Returns a vector listing the media files in `dir_path`
pub fn scan_for_media_files(dir_path: PathBuf) -> Vec<String> {
    let mut media_files = Vec::new();

    let read_dir = match fs::read_dir(dir_path) {
        Ok(r) => r,
        Err(_) => return vec![],
    };

    for entry in read_dir {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.is_dir() {
            media_files.extend(scan_for_media_files(path));
        } else {
            if let Some(ext) = path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    if MEDIA_FILES_EXTENSIONS.contains(&ext_str) {
                        media_files.push(
                            path.canonicalize()
                                .unwrap_or(path)
                                .to_string_lossy()
                                .into_owned(),
                        );
                    }
                }
            }
        }
    }
    media_files
}
