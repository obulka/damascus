use iced::{Command, Element, Subscription};

pub mod node_graph;
pub mod viewer;

use crate::model::Config;
use crate::update::{tabs::TabContentMessage, BaseMessage};
use crate::view::widget::TabType;
use node_graph::NodeGraph;
use viewer::Viewer;

pub trait TabContent {
    fn update(&mut self, _message: TabContentMessage) -> Command<BaseMessage> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<BaseMessage> {
        Subscription::none()
    }

    fn view(&mut self, config: &Config) -> Element<BaseMessage>;
}

pub fn tab_content_from_type(tab_type: TabType) -> Box<dyn TabContent> {
    match tab_type {
        TabType::Viewer => Box::new(Viewer::new()),
        TabType::NodeGraph => Box::new(NodeGraph::new()),
    }
}
