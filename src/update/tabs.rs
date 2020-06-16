// Local Imports
pub mod node_graph;
pub mod viewer;

pub use node_graph::NodeGraphMessage;
pub use viewer::ViewerMessage;
use crate::update::Message;

#[derive(Debug, Clone)]
pub enum TabContentMessage {
    NodeGraph((String, NodeGraphMessage)),
    Viewer((String, ViewerMessage)),
}

impl From<TabContentMessage> for Message {
    fn from(message: TabContentMessage) -> Message {
        Message::TabContent(message)
    }
}
