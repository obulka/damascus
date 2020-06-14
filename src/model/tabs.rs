// Local Imports
pub mod node_graph;
pub mod viewer;

use crate::model::Model;
use crate::update::tabs::TabContentMessage;
use crate::view::widget::TabType;

pub use node_graph::NodeGraph;
pub use viewer::Viewer;

pub trait TabContent: Model<TabContentMessage> {}

pub fn tab_content_from_type(tab_type: TabType) -> Box<dyn TabContent> {
    match tab_type {
        TabType::Viewer => Box::new(Viewer::new()),
        TabType::NodeGraph => Box::new(NodeGraph::new()),
    }
}
