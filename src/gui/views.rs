use actix_web::rt;
use iced::{
    alignment::Horizontal,
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, text_input, Column,
    },
    Alignment, Element, Length, Sandbox, Theme,
};

use crate::server::{app_settings::AppSettings, launch_server};

use super::messages::ServerControlMessage;
use super::state::AppState;
use super::style;

/// A small muted label displayed above an input
fn label(content: &str) -> iced::widget::Text<'static> {
    text(content.to_string()).size(13).style(style::MUTED)
}

fn card_title(content: &str) -> iced::widget::Text<'static> {
    text(content.to_string()).size(16).font(style::BOLD)
}

fn error_message(error: &str) -> iced::widget::Text<'_> {
    text(error).size(13).style(style::DANGER)
}

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

    fn server_view(&self) -> Element<'_, ServerControlMessage> {
        let is_running = self.server_handle.is_some();

        let (status_color, status_text) = match is_running {
            true => (style::SUCCESS, "Running"),
            false => (style::MUTED, "Stopped"),
        };
        let status = row![
            container(column![])
                .width(8)
                .height(8)
                .style(style::dot(status_color)),
            text(status_text).size(13).style(status_color),
        ]
        .spacing(8)
        .align_items(Alignment::Center);

        let mut host_input = text_input("127.0.0.1", self.host.as_str())
            .padding(10)
            .size(14)
            .style(style::text_input_style());
        let mut port_input = text_input("8080", self.port.as_str())
            .padding(10)
            .size(14)
            .style(style::text_input_style());

        if !is_running {
            host_input = host_input.on_input(ServerControlMessage::HostChanged);
            port_input = port_input.on_input(ServerControlMessage::PortChanged);
        }

        let inputs = row![
            column![label("Host"), host_input]
                .spacing(6)
                .width(Length::Fill),
            column![label("Port"), port_input].spacing(6).width(110),
        ]
        .spacing(12);

        let (button_text, button_style, message) = match is_running {
            true => (
                "Stop server",
                style::Button::Danger,
                ServerControlMessage::StopServerPressed,
            ),
            false => (
                "Start server",
                style::Button::Primary,
                ServerControlMessage::StartServerPressed,
            ),
        };
        let action = button(
            text(button_text)
                .size(15)
                .font(style::BOLD)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(Length::Fill)
        .padding(12)
        .style(style::button_style(button_style))
        .on_press(message);

        let mut content = column![
            row![card_title("Server"), horizontal_space(Length::Fill), status]
                .align_items(Alignment::Center),
            inputs,
            action,
        ]
        .spacing(16);

        if is_running {
            content = content.push(
                row![
                    label("Open"),
                    text(format!("http://{}:{}", self.host, self.port))
                        .size(13)
                        .style(style::ACCENT),
                    label("in your browser"),
                ]
                .spacing(5),
            );
        }

        if let Some(error) = &self.start_server_error {
            content = content.push(error_message(error));
        }

        container(content)
        .padding(20)
        .width(Length::Fill)
        .style(style::card)
        .into()
    }

    fn media_directories_view(&self) -> Element<'_, ServerControlMessage> {
        let media_directories = AppSettings::instance()
            .lock()
            .unwrap()
            .media_directories
            .clone();

        let list: Element<'_, ServerControlMessage> = if media_directories.is_empty() {
            container(
                text("No directories yet. Add one to start sharing your media.")
                    .size(13)
                    .style(style::MUTED),
            )
            .width(Length::Fill)
            .padding(24)
            .center_x()
            .style(style::list_item)
            .into()
        } else {
            let rows = media_directories.into_iter().enumerate().map(|(index, dir)| {
                container(
                    row![
                        text(dir).size(14).width(Length::Fill),
                        button(text("Remove").size(13))
                            .padding([4, 8])
                            .style(style::button_style(style::Button::Link))
                            .on_press(ServerControlMessage::RemoveMediaDirectoryPressed(index)),
                    ]
                    .spacing(12)
                    .align_items(Alignment::Center),
                )
                .padding([6, 6, 6, 14])
                .style(style::list_item)
                .into()
            });
            scrollable(
                Column::with_children(rows.collect())
                    .spacing(8)
                    // Leave room for the scrollbar
                    .padding([0, 8, 0, 0]),
            )
            .direction(scrollable::Direction::Vertical(
                scrollable::Properties::new().width(4).scroller_width(4),
            ))
            .height(Length::Fill)
            .style(style::scrollable_style())
            .into()
        };

        let mut content = column![
            row![
                card_title("Media directories"),
                horizontal_space(Length::Fill),
                button(text("Add directory").size(13))
                    .padding([6, 12])
                    .style(style::button_style(style::Button::Secondary))
                    .on_press(ServerControlMessage::AddMediaDirectoryPressed),
            ]
            .align_items(Alignment::Center),
            list,
        ]
        .spacing(16);

        if let Some(error) = &self.settings_error {
            content = content.push(error_message(error));
        }

        container(content)
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::card)
        .into()
    }
}

impl Sandbox for AppState {
    type Message = ServerControlMessage;

    fn view(&self) -> Element<'_, Self::Message> {
        let header = column![
            text("Palantir").size(28).font(style::BOLD),
            text("Stream your media library to any device on your network")
                .size(14)
                .style(style::MUTED),
        ]
        .spacing(4);

        let content = column![header, self.server_view(), self.media_directories_view()]
            .spacing(20)
            .max_width(560);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(24)
            .center_x()
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

    fn theme(&self) -> Theme {
        style::theme()
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
