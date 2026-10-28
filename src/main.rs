use std::env;
use std::path::PathBuf;

use iced::widget::{button, column, container, text};
use iced::{Element, Size, color};
use iced::{Length, window};

fn main() -> iced::Result {
    iced::application(FFMacroApp::default, FFMacroApp::update, FFMacroApp::view)
        .title(FFMacroApp::title)
        .window(window::Settings {
            size: Size::new(600.0, 300.0),
            resizable: false,
            ..Default::default()
        })
        .run()
}

struct FFMacroApp {
    state: Result<PathBuf, String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    RunProcess,
}

impl Default for FFMacroApp {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();

        // build state from path existence
        let state = args
            .get(1)
            .ok_or_else(|| "No media path specified".to_string())
            .map(PathBuf::from)
            .and_then(|p| {
                if p.exists() {
                    Ok(p)
                } else {
                    Err(format!("Path does not exist: {}", p.display()))
                }
            });

        Self { state }
    }
}

impl FFMacroApp {
    fn title(&self) -> String {
        "FFMacro".to_string()
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::RunProcess => {
                // button click here
                iced::Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let path = match &self.state {
            Ok(path) => path,
            Err(error) => {
                return container(
                    text(format!("Error: {error}"))
                        .size(16)
                        .color(color!(0xf43f5e)),
                )
                .padding(8)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(color!(0x09090b).into()),
                    ..Default::default()
                })
                .into();
            }
        };

        let content = column![
            text(format!("Path: {}", path.display())).size(16),
            button("Start Processing").on_press(Message::RunProcess)
        ]
        .spacing(8);

        container(content)
            .padding(8)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(color!(0x09090b).into()),
                ..Default::default()
            })
            .into()
    }
}
