use iced::{
    Command,
    Element,
    Subscription,
};

mod node_graph;
mod viewer;

use crate::action::{
    Message as DamascusMessage,
    tabs::Message,
};
use node_graph::NodeGraph;
use viewer::Viewer;
use crate::state::{
    Config,
    widget::TabType,
};


impl From<TabType> for String {
    fn from(tab_type: TabType) -> Self {
        match tab_type {
            TabType::NodeGraph => "NodeGraph".to_string(),
            TabType::Viewer => "Viewer".to_string(),
        }
    }
}


impl std::cmp::PartialEq<String> for Message {
    fn eq(&self, other: &String) -> bool {
        match self {
            Message::NodeGraph(..) => {
                let tab_type_string: String = TabType::NodeGraph.into();
                *other == tab_type_string
            }
            Message::Viewer(..) => {
                let tab_type_string: String = TabType::Viewer.into();
                *other == tab_type_string
            }
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
