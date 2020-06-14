// Standard Imports
use std::convert::TryFrom;

// 3rd Party Imports
use iced::{Command, Point, Vector};

// Local Imports
use crate::update::{
    BaseMessage, panel::PanelMessage, tabs::TabContentMessage,
};
use crate::view::widget::NodeType;
use crate::DamascusError;

#[derive(Debug, Clone)]
pub enum NodeGraphMessage {
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

impl From<NodeGraphMessage> for TabContentMessage {
    fn from(message: NodeGraphMessage) -> TabContentMessage {
        TabContentMessage::NodeGraph(message)
    }
}

impl From<NodeGraphMessage> for PanelMessage {
    fn from(message: NodeGraphMessage) -> PanelMessage {
        let message: TabContentMessage = message.into();
        message.into()
    }
}

impl From<NodeGraphMessage> for BaseMessage {
    fn from(message: NodeGraphMessage) -> BaseMessage {
        let message: PanelMessage = message.into();
        message.into()
    }
}

impl TryFrom<BaseMessage> for NodeGraphMessage {
    type Error = &'static DamascusError;

    fn try_from(message: BaseMessage) -> Result<Self, Self::Error> {
        if let BaseMessage::Panel(PanelMessage::TabContent(TabContentMessage::NodeGraph(
            message,
        ))) = message
        {
            Ok(message)
        } else {
            Err(&DamascusError::UpdateError)
        }
    }
}

pub fn clear_node_caches_command() -> Command<BaseMessage> {
    Command::perform(
        async move { NodeGraphMessage::ClearNodeCaches.into() },
        BaseMessage::Panel,
    )
}

pub fn clear_cache_command() -> Command<BaseMessage> {
    Command::perform(
        async move { NodeGraphMessage::ClearCache.into() },
        BaseMessage::Panel,
    )
}
