use serde_json;
use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
};

const SETTING_FILE_NAME: &str = "app_settings.json";

pub fn get_setting(setting_name: String) -> Option<serde_json::Value> {
    // Open the settings file
    let mut file = match std::fs::File::open(SETTING_FILE_NAME) {
        Ok(f) => f,
        Err(_) => return None,
    };

    // Read the file contents into a string
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return None;
    }

    // Parse the JSON string into a Value
    let settings: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(_) => return None,
    };

    // Return the setting's value
    settings.get(setting_name).cloned()
}

pub fn set_setting(
    setting_name: String,
    setting_value: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(SETTING_FILE_NAME)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut settings: serde_json::Value = if contents.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::from_str(&contents)?
    };

    settings[setting_name] = setting_value;

    let new_contents = serde_json::to_string_pretty(&settings)?;

    file.set_len(new_contents.len() as u64)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(new_contents.as_bytes())?;

    Ok(())
}
