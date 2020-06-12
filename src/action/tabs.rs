pub mod node_graph;
pub mod viewer;

use super::panel::Message as PanelMessage;
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
