use iced::{Command, Point};

use crate::action::{
    panel::Message as PanelMessage, tabs::Message as TabContentMessage, Message as DamascusMessage,
};
use crate::state::widget::NodeType;

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    ToggleGrid,
    ClearCache,
    ClearNodeCaches,
    ClearSelected,
    AddNode(NodeType, Point),
    DeselectNode(String),
    SelectNode(String),
    BeginSelecting(Point),
    ExpandSelection(Point),
    CompleteSelection,
}

impl From<Message> for TabContentMessage {
    fn from(message: Message) -> TabContentMessage {
        TabContentMessage::NodeGraph(message)
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

pub fn clear_node_caches_command() -> Command<DamascusMessage> {
    Command::perform(
        async move { Message::ClearNodeCaches.into() },
        DamascusMessage::Panel,
    )
}

pub fn clear_cache_command() -> Command<DamascusMessage> {
    Command::perform(
        async move { Message::ClearCache.into() },
        DamascusMessage::Panel,
    )
}
