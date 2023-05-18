use serde_json;
use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
};

const SETTINGS_FILE_NAME: &str = "app_settings.json";

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub media_directories: Option<Vec<String>>,
}

impl Settings {
    pub fn load() -> Option<Self> {
        // Open the settings file
        let mut file = match std::fs::File::open(SETTINGS_FILE_NAME) {
            Ok(f) => f,
            Err(_) => return None,
        };

        // Read the file contents into a string
        let mut contents = String::new();
        if let Err(_) = file.read_to_string(&mut contents) {
            return None;
        }

        // Parse the JSON string into a Value
        let settings: Settings = match serde_json::from_str(&contents) {
            Ok(v) => return v,
            Err(_) => return None,
        };
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(SETTINGS_FILE_NAME)?;

        let json_string = serde_json::to_string_pretty(self)?;

        file.set_len(json_string.len() as u64)?;
        file.seek(SeekFrom::Start(0))?;
        file.write_all(json_string.as_bytes())?;

        Ok(())
    }
}
