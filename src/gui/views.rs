use actix_web::rt;
use iced::{
    widget::{button, column, container, row, scrollable, text, text_input, Column},
    Alignment, Element, Length, Sandbox,
};

use crate::server::{app_settings::AppSettings, launch_server};

use super::messages::ServerControlMessage;
use super::state::AppState;

impl AppState {
    /// Applies `change` to the `media_directories` setting and saves the settings
    fn update_media_directories(&mut self, change: impl FnOnce(&mut Vec<String>)) {
        let mut settings = AppSettings::instance().lock().unwrap();
        change(&mut settings.media_directories);

        self.settings_error = match settings.save() {
            Ok(_) => None,
            Err(_) => Some("Error saving the settings".to_string()),
        };
    }

    fn media_directories_view(&self) -> Element<'_, ServerControlMessage> {
        let media_directories = AppSettings::instance()
            .lock()
            .unwrap()
            .media_directories
            .clone();

        let list: Element<'_, ServerControlMessage> = if media_directories.is_empty() {
            text("No media directory").into()
        } else {
            let rows = media_directories.into_iter().enumerate().map(|(index, dir)| {
                row![
                    text(dir).width(Length::Fill),
                    button("Remove")
                        .on_press(ServerControlMessage::RemoveMediaDirectoryPressed(index))
                        .padding(5)
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into()
            });
            scrollable(Column::with_children(rows.collect()).spacing(10).padding(10))
                .height(Length::Fixed(200.0))
                .into()
        };

        let settings_error = match &self.settings_error {
            Some(error) => column![text(error)],
            None => column![],
        };

        column![
            text("Media directories:"),
            list,
            settings_error,
            button("Add directory")
                .on_press(ServerControlMessage::AddMediaDirectoryPressed)
                .padding(10)
        ]
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
    }
}

impl Sandbox for AppState {
    type Message = ServerControlMessage;

    fn view(&self) -> Element<'_, Self::Message> {
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

        let col = column![
            start_server_error,
            state,
            settings,
            self.media_directories_view()
        ]
        .width(500)
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
            settings_error: None,
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
            ServerControlMessage::AddMediaDirectoryPressed => {
                let picked_directory = rfd::FileDialog::new()
                    .set_title("Select a media directory")
                    .pick_folder();

                if let Some(directory) = picked_directory {
                    let directory = directory.to_string_lossy().into_owned();
                    self.update_media_directories(|dirs| {
                        if !dirs.contains(&directory) {
                            dirs.push(directory);
                        }
                    });
                }
            }
            ServerControlMessage::RemoveMediaDirectoryPressed(index) => {
                self.update_media_directories(|dirs| {
                    if index < dirs.len() {
                        dirs.remove(index);
                    }
                });
            }
        }
    }
}
