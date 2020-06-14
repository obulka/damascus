// Standard Imports
use std::convert::TryFrom;

// Local Imports
pub mod node_graph;
pub mod viewer;

use super::panel::PanelMessage;
use crate::DamascusError;
use node_graph::NodeGraphMessage;
use viewer::ViewerMessage;

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
