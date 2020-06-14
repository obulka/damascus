// Standard Imports
use std::convert::TryFrom;

// Local Imports
pub mod node_graph;
pub mod viewer;

use super::{panel::Message as PanelMessage, Message as DamascusMessage};
use crate::DamascusError;
use node_graph::Message as NodeGraphMessage;
use viewer::Message as ViewerMessage;

#[derive(Debug, Clone)]
pub enum Message {
    NodeGraph(NodeGraphMessage),
    Viewer(ViewerMessage),
}

impl From<Message> for PanelMessage {
    fn from(message: Message) -> PanelMessage {
        PanelMessage::TabContent(message)
    }
}

impl TryFrom<DamascusMessage> for Message {
    type Error = &'static DamascusError;

    fn try_from(message: DamascusMessage) -> Result<Self, Self::Error> {
        if let DamascusMessage::Panel(PanelMessage::TabContent(message)) = message {
            Ok(message)
        } else {
            Err(&DamascusError::UpdateError)
        }
    }
}
