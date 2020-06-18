// Local Imports
pub mod node_graph;
pub mod viewer;

use crate::update::{tabs::TabContentMessage, Update};
use crate::view::View;

pub use node_graph::NodeGraph;
pub use viewer::Viewer;

pub trait TabContent: View + Update<TabContentMessage> {
    fn get_id(&self) -> &String;

    fn set_id(&mut self, id: String);
}

pub fn tab_content_from_type(tab_type: TabType) -> Box<dyn TabContent> {
    match tab_type {
        TabType::Viewer => Box::new(Viewer::new()),
        TabType::NodeGraph => Box::new(NodeGraph::new()),
    }
}

#[derive(Debug, Clone)]
pub enum TabType {
    NodeGraph,
    Viewer,
}

impl From<TabType> for Box<dyn TabContent> {
    fn from(tab_type: TabType) -> Self {
        match tab_type {
            TabType::NodeGraph => Box::new(NodeGraph::new()),
            TabType::Viewer => Box::new(Viewer::new()),
        }
    }
}

impl From<TabType> for String {
    fn from(tab_type: TabType) -> Self {
        match tab_type {
            TabType::NodeGraph => "NodeGraph".to_string(),
            TabType::Viewer => "Viewer".to_string(),
        }
    }
}

impl std::cmp::PartialEq<String> for TabContentMessage {
    fn eq(&self, other: &String) -> bool {
        match self {
            TabContentMessage::NodeGraph(..) => {
                let tab_type_string: String = TabType::NodeGraph.into();
                *other == tab_type_string
            }
            TabContentMessage::Viewer(..) => {
                let tab_type_string: String = TabType::Viewer.into();
                *other == tab_type_string
            }
        }
    }
}
