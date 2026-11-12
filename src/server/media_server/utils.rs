use crate::server::app_settings::AppSettings;
use std::path::{Path, PathBuf};

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

/// Returns true if the file has an `.srt` extension
pub fn is_srt_file(file: &Path) -> bool {
    file.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("srt"))
        .unwrap_or(false)
}

/// Decodes a text file's content as UTF-8, falling back to Latin-1
/// which is a common encoding for subtitle files
pub fn decode_text(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(e) => e.into_bytes().iter().map(|&b| b as char).collect(),
    }
}

/// Converts SRT subtitles into WebVTT subtitles
pub fn srt_to_vtt(srt: &str) -> String {
    let srt = srt.trim_start_matches('\u{feff}').replace("\r\n", "\n");

    let cues: Vec<String> = srt
        .lines()
        .map(|line| match line.contains("-->") {
            // SRT timestamps use ',' as the milliseconds separator whereas WebVTT uses '.'
            true => line.replace(',', "."),
            false => line.to_string(),
        })
        .collect();

    format!("WEBVTT\n\n{}\n", cues.join("\n"))
}
