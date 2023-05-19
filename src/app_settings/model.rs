use serde_json;
use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
};

const SETTINGS_FILE_NAME: &str = "app_settings.json";

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct AppSettings {
    pub media_directories: Vec<String>,
}

impl AppSettings {
    pub fn load() -> Result<Self,  std::io::Error> {
        // Open the settings file
        let mut file = match std::fs::File::open(SETTINGS_FILE_NAME) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        // Read the file contents into a string
        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(e);
        }

        // Parse the JSON string into a Settings
        match serde_json::from_str(&contents) {
            Ok(v) => return Ok(v),
            Err(e) => return Err(e.into()),
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

impl ToString for AppSettings {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
