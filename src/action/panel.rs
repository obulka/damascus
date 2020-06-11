use iced::pane_grid;

use super::{Message as DamascusMessage, tabs::Message as TabContentMessage};
use crate::state::widget::TabType;

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
