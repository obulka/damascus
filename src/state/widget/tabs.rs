use iced::{
    Command,
    Element,
    Subscription,
};

use crate::state::Config;
use crate::action::{
    Message as DamascusMessage,
    tabs::Message,
};

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
    fn update(&mut self, _message: Message) -> Command<DamascusMessage> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<DamascusMessage> {
        Subscription::none()
    }

    fn view(&mut self, config: &Config) -> Element<DamascusMessage>;
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
