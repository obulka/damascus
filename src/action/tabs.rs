pub mod node_graph;
pub mod viewer;

use node_graph::Message as NodeGraphMessage;
use viewer::Message as ViewerMessage;


#[derive(Debug, Clone)]
pub enum Message {
    NodeGraph(NodeGraphMessage),
    Viewer(ViewerMessage),
}
