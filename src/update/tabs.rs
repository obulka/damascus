// Standard Imports
use std::convert::TryFrom;

// Local Imports
use crate::DamascusError;
use super::panel::PanelMessage;

pub mod node_graph;
pub mod viewer;

pub use node_graph::NodeGraphMessage;
pub use viewer::ViewerMessage;

#[derive(Debug, Clone)]
pub enum TabContentMessage {
    NodeGraph(NodeGraphMessage),
    Viewer(ViewerMessage),
}

impl From<TabContentMessage> for PanelMessage {
    fn from(message: TabContentMessage) -> PanelMessage {
        PanelMessage::TabContent(message)
    }
}

impl TryFrom<PanelMessage> for TabContentMessage {
    type Error = &'static DamascusError;

    fn try_from(message: PanelMessage) -> Result<Self, Self::Error> {
        if let PanelMessage::TabContent(message) = message {
            Ok(message)
        } else {
            Err(&DamascusError::UpdateError)
        }
    }
}
