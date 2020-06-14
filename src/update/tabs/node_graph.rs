// Standard Imports
use std::convert::TryFrom;

// 3rd Party Imports
use iced::{Command, Point, Vector};

// Local Imports
use crate::update::{
    panel::Message as PanelMessage, tabs::Message as TabContentMessage, Message as DamascusMessage,
};
use crate::view::widget::NodeType;
use crate::DamascusError;

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
    TranslateSelected(Vector),
    NodesDropped,
    Translate(Vector),
    Zoom(f32, Option<Point>),
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

impl TryFrom<DamascusMessage> for Message {
    type Error = &'static DamascusError;

    fn try_from(message: DamascusMessage) -> Result<Self, Self::Error> {
        if let DamascusMessage::Panel(PanelMessage::TabContent(TabContentMessage::NodeGraph(
            message,
        ))) = message
        {
            Ok(message)
        } else {
            Err(&DamascusError::UpdateError)
        }
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
