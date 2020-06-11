use iced::{Command, Element, Subscription};

pub mod node_graph;
pub mod viewer;

use crate::action::{tabs::Message, Message as DamascusMessage};
use crate::state::{widget::TabType, Config};
use node_graph::NodeGraph;
use viewer::Viewer;

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
        TabType::Viewer => Box::new(Viewer::new()),
        TabType::NodeGraph => Box::new(NodeGraph::new()),
    }
}
