use std::path::PathBuf;

use iced::widget::{button, column, text};
use iced::{Element, Theme};
use mkwii_distro_tool_config::{self as config, Config, ParentMessage};
use mkwii_distro_tool_logger as logger;

use crate::workspace::Workspace;

mod workspace;

fn main() -> iced::Result {
    logger::init();

    iced::application(ThisApp::default, ThisApp::update, ThisApp::view)
        .theme(ThisApp::theme)
        .title(ThisApp::title)
        .run()
}

#[derive(Default)]
pub struct ThisApp {
    pub current_workspace: Workspace,
    pub config_path: Option<PathBuf>,
    pub is_unsaved: bool,
    pub config: Config,
    theme: Option<Theme>,
}

impl ThisApp {
    fn title(&self) -> String {
        let unsaved_label = if self.is_unsaved { "*" } else { "" };
        if let Some(path) = self.config_path.as_ref() {
            format!("mkwii-distro-tool - {}{unsaved_label}", path.display())
        } else {
            format!("mkwii-distro-tool{unsaved_label}")
        }
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::ThemeChanged(theme) => self.theme = Some(theme),
            Message::OpenProject => {
                if let Some(path) = rfd::FileDialog::new().add_filter("Config file", &["pul"]).pick_file() {
                    log::debug!("config path: {:?}", path);
                    self.config_path = Some(path.clone());
                    if let Ok(config) = Config::read_from_path(path) {
                        self.config = config;
                    }
                }
            }
            Message::Config(msg) => match msg {
                ParentMessage::Error(err) => log::error!("{err}"),
                _ => self.config.update(msg),
            },
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            button("Open").on_press(Message::OpenProject),
            self.config_path.as_ref().and_then(|v| Some(text(format!("{}", v.display())))),
            self.config.view().map(Message::Config)
        ]
        .into()
    }

    fn theme(&self) -> Option<Theme> {
        self.theme.clone()
    }
}

#[derive(Clone)]
enum Message {
    ThemeChanged(Theme),

    OpenProject,
    SaveProject,

    Config(config::ParentMessage),
}
