pub mod node_graph;
pub mod viewer;

use crate::update::tabs::TabContentMessage;
use crate::view::TabType;

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
