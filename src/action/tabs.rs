pub mod node_graph;
pub mod viewer;

use node_graph::Message as NodeGraphMessage;
use viewer::Message as ViewerMessage;


#[derive(Debug, Clone)]
pub enum Message {
    NodeGraph(NodeGraphMessage),
    Viewer(ViewerMessage),
}

impl std::cmp::PartialEq<String> for Message {
    fn eq(&self, other: &String) -> bool {
        match self {
            Message::NodeGraph(..) => {
                other == "NodeGraph"
            }
            Message::Viewer(..) => {
                other == "Viewer"
            }
        }
    }
}
