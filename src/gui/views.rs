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
        let start_server_error = match &self.start_server_error {
            Some(error) => column![text(error)],
            None => column![],
        };

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

        let mut host_input = text_input("Host", self.host.as_str()).padding(10);
        let mut port_input = text_input("Port", self.port.as_str()).padding(10);

        if !self.server_handle.is_some() {
            host_input = host_input.on_input(|host| ServerControlMessage::HostChanged(host));
            port_input = port_input.on_input(|port| ServerControlMessage::PortChanged(port))
        }

        let settings = column![
            row![text("Host:"), host_input]
                .spacing(20)
                .align_items(Alignment::Center),
            row![text("Port:"), port_input]
                .spacing(20)
                .align_items(Alignment::Center)
        ]
        .spacing(20);

        let col = column![start_server_error, state, settings]
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
            start_server_error: None,
        }
    }

    fn title(&self) -> String {
        "Palantir Media Server".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ServerControlMessage::StartServerPressed => {
                let port = match self.port.parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => {
                        self.start_server_error = Some("Invalid port".to_string());
                        return;
                    }
                };

                match launch_server(self.host.clone(), port) {
                    Ok(server_handle) => {
                        self.server_handle = Some(server_handle);
                        self.start_server_error = None;
                    }
                    Err(_) => {
                        self.start_server_error = Some("Error starting the server".to_string())
                    }
                };
            }
            ServerControlMessage::StopServerPressed => {
                rt::System::new().block_on(self.server_handle.as_ref().unwrap().stop(true));
                self.start_server_error = None;
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
