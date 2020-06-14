use iced::{Command, Container, Element, Length, Point, Rectangle};
use std::ops::RangeInclusive;

use super::TabContent;
use crate::model::Config;
use crate::update::{
    tabs::{node_graph::Message, Message as TabContentMessage},
    Message as DamascusMessage,
};
use crate::view::{node::NodeType, tabs::node_graph::State};

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
                Message::NodesDropped => {
                    self.state.move_selected();
                    self.state.clear_node_caches();
                }
                Message::CompleteSelection => {
                    self.state.close_selection_box();
                }
                Message::Translate(translation) => {
                    self.state.translate(translation);
                    self.state.clear_cache();
                }
                Message::TranslateSelected(translation) => {
                    self.state.translate_selected(translation);
                    self.state.clear_node_caches();
                }
                Message::Zoom(scroll_delta, cursor_position) => {
                    self.state.zoom(scroll_delta, cursor_position);
                    self.state.clear_cache();
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

pub struct Region {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub grid_size: f32,
}

impl Region {
    pub fn rows(&self) -> RangeInclusive<isize> {
        let first_row = (self.y / self.grid_size).floor() as isize;

        let visible_rows = (self.height / self.grid_size).ceil() as isize;

        first_row..=first_row + visible_rows + 1
    }

    pub fn columns(&self) -> RangeInclusive<isize> {
        let first_column = (self.x / self.grid_size).floor() as isize;

        let visible_columns = (self.width / self.grid_size).ceil() as isize;

        first_column..=first_column + visible_columns + 1
    }
}

impl From<Region> for Rectangle {
    fn from(region: Region) -> Rectangle {
        Rectangle {
            x: region.x / region.grid_size,
            y: region.y / region.grid_size,
            width: region.width / region.grid_size,
            height: region.height / region.grid_size,
        }
    }
}
