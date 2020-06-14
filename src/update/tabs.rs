// Local Imports
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
