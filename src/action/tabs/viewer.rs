use std::time::Instant;

use crate::action::{
    panel::Message as PanelMessage, tabs::Message as TabContentMessage, Message as DamascusMessage,
};
use crate::model::tabs::viewer::grid;

#[derive(Debug, Clone)]
pub enum Message {
    Grid(grid::Message),
    Tick(Instant),
    TogglePlayback,
    ToggleGrid(bool),
    Next,
    Clear,
    SpeedChanged(f32),
}

impl From<Message> for TabContentMessage {
    fn from(message: Message) -> TabContentMessage {
        TabContentMessage::Viewer(message)
    }
}

impl From<Message> for PanelMessage {
    fn from(message: Message) -> PanelMessage {
        let message: TabContentMessage = message.into();
        message.into()
    }
}

impl From<Message> for DamascusMessage {
    fn from(message: Message) -> DamascusMessage {
        let message: PanelMessage = message.into();
        message.into()
    }
}
