// Local Imports
pub mod node_graph;
pub mod viewer;

use crate::update::Message;
pub use node_graph::NodeGraphMessage;
pub use viewer::ViewerMessage;

#[derive(Debug, Clone)]
pub enum TabContentMessage {
    NodeGraph((Option<String>, NodeGraphMessage)),
    Viewer((Option<String>, ViewerMessage)),
}

impl From<TabContentMessage> for Message {
    fn from(message: TabContentMessage) -> Message {
        Message::TabContent(message)
    }
}
