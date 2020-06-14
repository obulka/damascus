// Standard Imports
use std::{error::Error, fmt};

// 3rd Party Imports
use iced::{executor, pane_grid, Application, Command, Element, Subscription};

// Local Imports
pub mod model;
pub mod update;
pub mod view;

pub use model::Damascus;
use model::{panel::Panel, Config};
use update::{Message, Update};
use view::View;

#[derive(Debug, Copy, Clone)]
pub enum DamascusError {
    ModelError,
    ViewError,
    UpdateError,
}

impl fmt::Display for DamascusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DamascusError::ModelError => write!(f, "Model Error"),
            DamascusError::ViewError => write!(f, "Update Error"),
            DamascusError::UpdateError => write!(f, "View Error"),
        }
    }
}

impl Error for DamascusError {}

impl Application for Damascus {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = Config;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let (panes, _) = pane_grid::State::new(Panel::new());
        (
            Damascus {
                config: flags,
                panes: panes,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.config.title.clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        if let Some(command) = Update::update(self, message) {
            return command;
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Update::subscription(self)
    }

    fn view(&mut self) -> Element<Message> {
        View::view(self, &self.config.clone())
    }
}
