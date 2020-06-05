use iced::Element;

use crate::state::Config;
use crate::action::Message;

pub mod node_graph;
pub mod viewer;

pub use node_graph::NodeGraph;
pub use viewer::Viewer;


#[derive(Debug, Clone)]
pub enum TabType {
    NodeGraph,
    Viewer,
}

impl From<TabType> for String {
    fn from(tab_type: TabType) -> Self {
        match tab_type {
            TabType::NodeGraph => "NodeGraph".to_string(),
            TabType::Viewer => "Viewer".to_string(),
        }
    }
}

pub trait TabContent {
    fn view(&mut self, config: &Config) -> Element<Message>;
}

pub fn tab_content_from_type(tab_type: TabType) -> Box<dyn TabContent> {
    match tab_type {
        TabType::Viewer => {
            Box::new(Viewer{})
        }
        TabType::NodeGraph => {
            Box::new(NodeGraph::new())
        }
    }
}
