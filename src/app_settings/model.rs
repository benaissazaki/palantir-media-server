use serde_json;
use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
};

#[cfg(test)]
const SETTINGS_FILE_NAME: &str = "app_settings_test.json";

#[cfg(not(test))]
const SETTINGS_FILE_NAME: &str = "app_settings.json";

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct AppSettings {
    pub media_directories: Vec<String>,
}

impl AppSettings {
    /// Read the settings from `SETTINGS_FILE_NAME` into an `AppSettings` struct
    /// # Errors
    /// This function returns an error if:
    /// - The file does not exist
    /// - The file's content could not be read into a string
    /// - The file's content could not be deserialized into an `AppSettings`
    pub fn load() -> Result<Self, std::io::Error> {
        let mut file = match std::fs::File::open(SETTINGS_FILE_NAME) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(e);
        }

        match serde_json::from_str(&contents) {
            Ok(v) => return Ok(v),
            Err(e) => return Err(e.into()),
        };
    }

    /// Serialize the settings into JSON format and write them into `SETTINGS_FILE_NAME`
    /// # Errors
    /// This function returns an error if:
    /// - The file could not be opened
    /// - The AppSettings object could not be serialized into a JSON string
    /// - The JSON string could not be written into the `SETTINGS_FILE_NAME`
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // The file is created if it does not exist
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

impl ToString for AppSettings {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
