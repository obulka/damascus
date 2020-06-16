// Standard Imports
use std::time::{Duration, Instant};

// 3rd Party Imports
use iced::{time, Command, Subscription};

// Local Imports
use crate::model::tabs::viewer::{grid, Viewer};
use crate::update::{tabs::TabContentMessage, Message, Update};

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

impl Update<TabContentMessage> for Viewer {
    fn update(&mut self, message: TabContentMessage) -> Command<Message> {
        if let TabContentMessage::Viewer((id, message)) = message {
            if id.is_none() || id.unwrap() == self.id {
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

                            return Command::perform(task, Message::TabContent);
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
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.is_playing {
            let id = self.id.clone();
            time::every(Duration::from_millis(1000 / self.speed as u64)).map(move |instant| {
                TabContentMessage::Viewer((Some(id.clone()), ViewerMessage::Tick(instant))).into()
            })
        } else {
            Subscription::none()
        }
    }
}
