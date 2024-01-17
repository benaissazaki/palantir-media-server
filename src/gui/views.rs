use actix_web::rt;
use iced::{
    widget::{button, column, text, text_input},
    Element, Sandbox,
};

use crate::server::launch_server;

use super::messages::ServerControlMessage;
use super::state::AppState;

impl Sandbox for AppState {
    type Message = ServerControlMessage;

    fn view(&self) -> Element<Self::Message> {
        let state = match self.server_handle {
            None => {
                column![button("Start server").on_press(ServerControlMessage::StartServerPressed)]
            }
            Some(_) => column![
                text(format!("Server running at {}:{}", self.host, self.port)),
                button("Stop server").on_press(ServerControlMessage::StopServerPressed)
            ],
        };

        let settings = column![
            text_input("Host", self.host.as_str()).on_input(|host| ServerControlMessage::HostChanged(host)),
            text_input("Port", self.port.as_str()).on_input(|port| ServerControlMessage::PortChanged(port))
        ];

        column![state, settings].into()
    }

    fn new() -> Self {
        Self {
            server_handle: None,
            host: "127.0.0.1".to_string(),
            port: "8080".to_string(),
        }
    }

    fn title(&self) -> String {
        "Palantir Media Server".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ServerControlMessage::StartServerPressed => {
                self.server_handle = Some(launch_server(self.host.clone(), str::parse(self.port.as_str()).unwrap()));
            }
            ServerControlMessage::StopServerPressed => {
                rt::System::new().block_on(self.server_handle.as_ref().unwrap().stop(true));
                self.server_handle = None;
            }
            ServerControlMessage::HostChanged(host) => {
                self.host = host;
            },
            ServerControlMessage::PortChanged(port) => {
                println!("{}", port);
                self.port = port;
            },
        }
    }
}
