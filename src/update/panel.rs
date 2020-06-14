// Standard Imports
use std::convert::TryFrom;

// 3rd Party Imports
use iced::pane_grid;

// Local Imports
use super::{tabs::Message as TabContentMessage, Message as DamascusMessage};
use crate::view::widget::TabType;
use crate::DamascusError;

#[derive(Debug, Clone)]
pub enum Message {
    TabContent(TabContentMessage),
    MoveTab((pane_grid::Pane, usize, pane_grid::Pane)),
    OpenTabFocused(TabType),
    CloseTab(pane_grid::Pane, usize),
    FocusTab((pane_grid::Pane, usize)),
}

impl From<Message> for DamascusMessage {
    fn from(message: Message) -> DamascusMessage {
        DamascusMessage::Panel(message)
    }
}

impl TryFrom<DamascusMessage> for Message {
    type Error = &'static DamascusError;

    fn try_from(message: DamascusMessage) -> Result<Self, Self::Error> {
        if let DamascusMessage::Panel(message) = message {
            Ok(message)
        } else {
            Err(&DamascusError::UpdateError)
        }
    }
}
