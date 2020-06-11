use iced::{Command, Container, Element, Length, Point};

use super::TabContent;
use crate::action::{
    panel::Message as PanelMessage,
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
        state.add_node(NodeType::Viewer, Point::ORIGIN);
        Self { state: state }
    }

    pub fn clear_node_caches_command() -> Command<DamascusMessage> {
        Command::perform(
            async move {
                PanelMessage::TabContent(TabContentMessage::NodeGraph(Message::ClearNodeCaches))
            },
            DamascusMessage::Panel,
        )
    }

    pub fn clear_cache_command() -> Command<DamascusMessage> {
        Command::perform(
            async move { PanelMessage::TabContent(TabContentMessage::NodeGraph(Message::ClearCache)) },
            DamascusMessage::Panel,
        )
    }
}

impl TabContent for NodeGraph {
    fn update(&mut self, message: TabContentMessage) -> Command<DamascusMessage> {
        if let TabContentMessage::NodeGraph(message) = message {
            match message {
                Message::ToggleGrid => self.state.toggle_lines(),
                Message::Next => {}
                Message::AddNode(..) => {}
                Message::ClearCache => {
                    self.state.clear_cache();
                }
                Message::ClearNodeCaches => {
                    self.state.clear_node_caches();
                }
                Message::ClearSelected => {
                    self.state.clear_selected();
                    return NodeGraph::clear_node_caches_command();
                }
                Message::DeselectNode(label) => {
                    self.state.deselect_node(label);
                    return NodeGraph::clear_node_caches_command();
                }
                Message::SelectNode(label) => {
                    self.state.select_node(label);
                    return NodeGraph::clear_node_caches_command();
                }
            }
        }
        Command::none()
    }

    fn view(&mut self, config: &Config) -> Element<DamascusMessage> {
        let content = self.state.view(config).map(|message| {
            DamascusMessage::Panel(PanelMessage::TabContent(TabContentMessage::NodeGraph(
                message,
            )))
        });

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(1)
            .into()
    }
}
