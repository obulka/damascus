use iced::Element;

pub mod viewer;

use crate::action::Message;
use crate::state::Config;
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
    fn view(&self, config: &Config) -> Element<Message>;
}
