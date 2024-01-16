use actix_web::rt;
use iced::{widget::{button, column}, Sandbox, Element};

use crate::server::launch_server;

use super::state::AppState;
use super::messages::ServerControlMessage;

impl Sandbox for AppState {
    type Message = ServerControlMessage;

    fn view(&self) -> Element<Self::Message> {
        column![
            button("Start server").on_press(ServerControlMessage::StartServerPressed),
            button("Stop server").on_press(ServerControlMessage::StopServerPressed),
        ].into()
    }

    fn new() -> Self {
        Self { server_handle: None  }
    }

    fn title(&self) -> String {
        "Palantir Media Server".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ServerControlMessage::StartServerPressed => {
                self.server_handle = Some(launch_server());
            }
            ServerControlMessage::StopServerPressed => {
                print!("Shutting down server");
                rt::System::new().block_on(self.server_handle.as_ref().unwrap().stop(true));
            },
        }
    }
}