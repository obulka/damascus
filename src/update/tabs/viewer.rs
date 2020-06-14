// Standard Imports
use std::convert::TryFrom;
use std::time::Instant;

// Local Imports
use crate::DamascusError;
use crate::model::tabs::viewer::grid;
use crate::update::{
    panel::Message as PanelMessage, tabs::Message as TabContentMessage, Message as DamascusMessage,
};

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

impl TryFrom<DamascusMessage> for Message {
    type Error = &'static DamascusError;

    fn try_from(message: DamascusMessage) -> Result<Self, Self::Error> {
        if let DamascusMessage::Panel(PanelMessage::TabContent(TabContentMessage::Viewer(
            message,
        ))) = message
        {
            Ok(message)
        } else {
            Err(&DamascusError::UpdateError)
        }
    }
}
