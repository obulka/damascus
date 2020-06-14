// Local Imports
pub mod node_graph;
pub mod viewer;

use crate::model::Model;
use crate::update::{tabs::TabContentMessage};
use crate::view::widget::TabType;

pub use node_graph::NodeGraph;
// pub use viewer::Viewer;

pub trait TabContent: Model {
    // fn update(&mut self, _message: TabContentMessage) -> Command<BaseMessage> {
    //     Command::none()
    // }

    // fn subscription(&self) -> Subscription<BaseMessage> {
    //     Subscription::none()
    // }

    // fn view(&mut self, config: &Config) -> Element<BaseMessage>;
}

pub fn tab_content_from_type(tab_type: TabType) -> Box<dyn TabContent<Message = TabContentMessage>> {
    match tab_type {
        TabType::Viewer => Box::new(NodeGraph::new()),
        TabType::NodeGraph => Box::new(NodeGraph::new()),
    }
}
