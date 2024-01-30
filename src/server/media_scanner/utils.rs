use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use crate::server::media_server;

pub const MEDIA_FILES_EXTENSIONS: &'static [&'static str] = &["mp3", "mp4", "avi", "wav", "mkv"];
pub const SUBTITLES_EXTENSION: &'static [&'static str] = &["srt", "sub", "ssa", "smi", "vtt"];

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

/// Returns a vector listing the files in `dir_path` whose extension is contained in `extensions`
fn scan_directory_for_extensions(dir_path: PathBuf, extensions:  &'static [&'static str]) -> Vec<String> {
    let mut files = Vec::new();

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
            files.extend(scan_directory_for_extensions(path, extensions));
        } else {
            if let Some(ext) = path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    if extensions.contains(&ext_str) {
                        files.push(
                            path.canonicalize()
                                .unwrap_or(path)
                                .to_string_lossy()
                                .into_owned()
                                .replace("\\\\?\\", ""),
                        );
                    }
                }
            }
        }
    }
    files
}

/// Returns a vector listing the media files in `dir_path`
pub fn scan_for_media_files(dir_path: PathBuf) -> Vec<String> {
    scan_directory_for_extensions(dir_path, MEDIA_FILES_EXTENSIONS)
}

/// Returns a vector containing subtitles contained in a directory
pub fn scan_for_subtitles_in_dir(dir_path: PathBuf) -> Vec<String> {
    if !media_server::utils::is_file_in_media_directories(dir_path.clone()){
        return vec![];
    };

     scan_directory_for_extensions(dir_path, SUBTITLES_EXTENSION)
}