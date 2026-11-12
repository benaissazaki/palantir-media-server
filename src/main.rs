use iced::{window, Sandbox, Settings};
use crate::gui::state::AppState;

mod server;
mod gui;

pub fn main() -> iced::Result {
    // Set the exe's parent directory to be the working directory if release
    // so that the settings file and the client are found wherever the app is launched from
    #[cfg(not(debug_assertions))]
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            if let Err(err) = std::env::set_current_dir(&exe_dir) {
                eprintln!("Failed to set current directory: {}", err);
            }
        }
    }

    AppState::run(Settings {
        window: window::Settings {
            size: (560, 680),
            min_size: Some((440, 560)),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
