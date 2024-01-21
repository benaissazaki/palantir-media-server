use iced::{Sandbox, Settings};
use crate::gui::state::AppState;

mod server;
mod gui;

pub fn main() -> iced::Result {
    AppState::run(Settings::default())
}