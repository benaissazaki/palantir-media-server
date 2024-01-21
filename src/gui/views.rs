use actix_web::rt;
use iced::{
    widget::{button, column, container, row, text, text_input},
    Alignment, Element, Length, Sandbox,
};

use crate::server::launch_server;

use super::messages::ServerControlMessage;
use super::state::AppState;

impl Sandbox for AppState {
    type Message = ServerControlMessage;

    fn view(&self) -> Element<Self::Message> {
        let state = match self.server_handle {
            None => column![button("Start server")
                .on_press(ServerControlMessage::StartServerPressed)
                .padding(10)]
            .spacing(20)
            .align_items(Alignment::Center),
            Some(_) => column![
                text(format!("Server running at {}:{}", self.host, self.port)),
                button("Stop server")
                    .on_press(ServerControlMessage::StopServerPressed)
                    .padding(10)
            ]
            .spacing(20)
            .align_items(Alignment::Center),
        };

        let state = container(state).center_x();
        let settings = column![
            row![
                text("Host:"),
                text_input("Host", self.host.as_str())
                    .on_input(|host| ServerControlMessage::HostChanged(host))
                    .padding(10)
            ]
            .spacing(20)
            .align_items(Alignment::Center),
            row![
                text("Port:"),
                text_input("Port", self.port.as_str())
                    .on_input(|port| ServerControlMessage::PortChanged(port))
                    .padding(10)
            ]
            .spacing(20)
            .align_items(Alignment::Center)
        ]
        .spacing(20);

        let col = column![state, settings]
            .width(300)
            .spacing(20)
            .align_items(Alignment::Center);

        container(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
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
                self.server_handle = Some(launch_server(
                    self.host.clone(),
                    str::parse(self.port.as_str()).unwrap(),
                ));
            }
            ServerControlMessage::StopServerPressed => {
                rt::System::new().block_on(self.server_handle.as_ref().unwrap().stop(true));
                self.server_handle = None;
            }
            ServerControlMessage::HostChanged(host) => {
                self.host = host;
            }
            ServerControlMessage::PortChanged(port) => {
                self.port = port;
            }
        }
    }
}
