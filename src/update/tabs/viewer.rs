// Standard Imports
use std::convert::TryFrom;
use std::time::{Duration, Instant};

// 3rd Party Imports
use iced::{time, Command, Subscription};

// Local Imports
use crate::model::tabs::viewer::{grid, Viewer};
use crate::update::{panel::PanelMessage, tabs::TabContentMessage, BaseMessage, Update};
use crate::DamascusError;

#[derive(Debug, Clone)]
pub enum ViewerMessage {
    Grid(grid::Message),
    Tick(Instant),
    TogglePlayback,
    ToggleGrid(bool),
    Next,
    Clear,
    SpeedChanged(f32),
}

impl From<ViewerMessage> for TabContentMessage {
    fn from(message: ViewerMessage) -> TabContentMessage {
        TabContentMessage::Viewer(message)
    }
}

impl From<ViewerMessage> for PanelMessage {
    fn from(message: ViewerMessage) -> PanelMessage {
        let message: TabContentMessage = message.into();
        message.into()
    }
}

impl From<ViewerMessage> for BaseMessage {
    fn from(message: ViewerMessage) -> BaseMessage {
        let message: PanelMessage = message.into();
        message.into()
    }
}

impl TryFrom<BaseMessage> for ViewerMessage {
    type Error = &'static DamascusError;

    fn try_from(message: BaseMessage) -> Result<Self, Self::Error> {
        if let BaseMessage::Panel(PanelMessage::TabContent(TabContentMessage::Viewer(message))) =
            message
        {
            Ok(message)
        } else {
            Err(&DamascusError::UpdateError)
        }
    }
}

impl Update for Viewer {
    type Message = TabContentMessage;

    fn update(&mut self, message: TabContentMessage) -> Command<BaseMessage> {
        if let TabContentMessage::Viewer(message) = message {
            match message {
                ViewerMessage::Grid(message) => {
                    self.grid.update(message);
                }
                ViewerMessage::Tick(_) | ViewerMessage::Next => {
                    self.queued_ticks = (self.queued_ticks + 1).min(self.speed);

                    if let Some(task) = self.grid.tick(self.queued_ticks) {
                        if let Some(speed) = self.next_speed.take() {
                            self.speed = speed;
                        }

                        self.queued_ticks = 0;

                        return Command::perform(task, BaseMessage::Panel);
                    }
                }
                ViewerMessage::TogglePlayback => {
                    self.is_playing = !self.is_playing;
                }
                ViewerMessage::ToggleGrid(show_grid_lines) => {
                    self.grid.toggle_lines(show_grid_lines);
                }
                ViewerMessage::Clear => {
                    self.grid.clear();
                }
                ViewerMessage::SpeedChanged(speed) => {
                    if self.is_playing {
                        self.next_speed = Some(speed.round() as usize);
                    } else {
                        self.speed = speed.round() as usize;
                    }
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<BaseMessage> {
        if self.is_playing {
            time::every(Duration::from_millis(1000 / self.speed as u64))
                .map(|instant| ViewerMessage::Tick(instant).into())
        } else {
            Subscription::none()
        }
    }
}
