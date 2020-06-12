use iced::{Command, Container, Element, Length, Point};

use super::TabContent;
use crate::action::{
    tabs::{node_graph::Message, Message as TabContentMessage},
    Message as DamascusMessage,
};
use crate::state::{node::NodeType, tabs::node_graph::State, Config};

#[derive(Default)]
pub struct NodeGraph {
    state: State,
}

impl NodeGraph {
    pub fn new() -> Self {
        let mut state = State::default();
        for i in 0..5 {
            state.add_node(NodeType::Viewer, Point::new(i as f32, i as f32));
        }
        Self { state: state }
    }
}

impl TabContent for NodeGraph {
    fn update(&mut self, message: TabContentMessage) -> Command<DamascusMessage> {
        if let TabContentMessage::NodeGraph(message) = message {
            match message {
                Message::ToggleGrid => self.state.toggle_lines(),
                Message::Next => {}
                Message::AddNode(node_type, position) => {
                    self.state.add_node(node_type, position);
                }
                Message::ClearCache => {
                    self.state.clear_cache();
                }
                Message::ClearNodeCaches => {
                    self.state.clear_node_caches();
                }
                Message::ClearSelected => {
                    self.state.clear_selected();
                    self.state.clear_node_caches();
                }
                Message::DeselectNode(label) => {
                    self.state.deselect_node(label);
                    self.state.clear_node_caches();
                }
                Message::SelectNode(label) => {
                    self.state.select_node(label);
                    self.state.clear_node_caches();
                }
                Message::BeginSelecting(start_position) => {
                    self.state.clear_selected();
                    self.state.initialize_selection_box(start_position);
                    self.state.clear_node_caches();
                }
                Message::ExpandSelection(lower_right_position) => {
                    self.state.expand_selection_box(lower_right_position);
                    self.state.clear_node_caches();
                }
                Message::CompleteSelection => {
                    self.state.close_selection_box();
                }
            }
        }
        Command::none()
    }

    fn view(&mut self, config: &Config) -> Element<DamascusMessage> {
        let content = self.state.view(config).map(|message| message.into());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(1)
            .into()
    }
}
