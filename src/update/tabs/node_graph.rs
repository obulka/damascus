// 3rd Party Imports
use iced::{
    canvas::{Cursor, Event},
    mouse, Command, Point, Rectangle, Vector,
};

// Local Imports
pub mod node;

use crate::model::tabs::node_graph::{node::NodeType, NodeGraph};
use crate::update::{tabs::TabContentMessage, CanvasUpdate, Message, Update};

#[derive(Debug, Clone)]
pub enum NodeGraphMessage {
    Next,
    ToggleGrid,
    ClearCache,
    ClearNodeCaches,
    ClearSelected,
    AddNode(NodeType, Point),
    DeselectNode(String),
    SelectNode(String),
    BeginSelecting(Point),
    ExpandSelection(Point),
    CompleteSelection,
    TranslateSelected(Vector),
    NodesDropped,
    Translate(Vector),
    Zoom(f32, Option<Point>),
}

pub fn clear_node_caches_command() -> Command<Message> {
    Command::perform(
        async move { TabContentMessage::NodeGraph((None, NodeGraphMessage::ClearNodeCaches)) },
        Message::TabContent,
    )
}

pub fn clear_cache_command() -> Command<Message> {
    Command::perform(
        async move { TabContentMessage::NodeGraph((None, NodeGraphMessage::ClearCache)) },
        Message::TabContent,
    )
}

pub enum Interaction {
    None,
    Panning { translation: Vector, start: Point },
    MovingSelected { start: Point },
    SelectingNodes,
}

impl Update<TabContentMessage> for NodeGraph {
    fn update(&mut self, message: TabContentMessage) -> Command<Message> {
        if let TabContentMessage::NodeGraph((id, message)) = message {
            if id.is_none() || id.unwrap() == self.id {
                match message {
                    NodeGraphMessage::ToggleGrid => self.toggle_lines(),
                    NodeGraphMessage::Next => {}
                    NodeGraphMessage::AddNode(node_type, position) => {
                        self.add_node(node_type, position);
                    }
                    NodeGraphMessage::ClearCache => {
                        self.clear_cache();
                    }
                    NodeGraphMessage::ClearNodeCaches => {
                        self.clear_node_caches();
                    }
                    NodeGraphMessage::ClearSelected => {
                        self.clear_selected();
                        self.clear_node_caches();
                    }
                    NodeGraphMessage::DeselectNode(label) => {
                        self.deselect_node(label);
                        self.clear_node_caches();
                    }
                    NodeGraphMessage::SelectNode(label) => {
                        if !self.selected_nodes.contains(&label) {
                            self.clear_selected();
                        }
                        self.select_node(label);
                        self.clear_node_caches();
                    }
                    NodeGraphMessage::BeginSelecting(start_position) => {
                        self.clear_selected();
                        self.initialize_selection_box(start_position);
                        self.clear_node_caches();
                    }
                    NodeGraphMessage::ExpandSelection(lower_right_position) => {
                        self.expand_selection_box(lower_right_position);
                        self.clear_node_caches();
                    }
                    NodeGraphMessage::NodesDropped => {
                        self.move_selected();
                        self.clear_node_caches();
                    }
                    NodeGraphMessage::CompleteSelection => {
                        self.close_selection_box();
                    }
                    NodeGraphMessage::Translate(translation) => {
                        self.translate(translation);
                        self.clear_cache();
                    }
                    NodeGraphMessage::TranslateSelected(translation) => {
                        self.translate_selected(translation);
                        self.clear_node_caches();
                    }
                    NodeGraphMessage::Zoom(scroll_delta, cursor_position) => {
                        self.zoom(scroll_delta, cursor_position);
                        self.clear_cache();
                    }
                }
            }
        }
        Command::none()
    }
}

impl CanvasUpdate<NodeGraphMessage> for NodeGraph {
    fn update(
        &mut self,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<NodeGraphMessage> {
        if let Event::Mouse(mouse::Event::ButtonReleased(button)) = event {
            match button {
                mouse::Button::Left => match self.interaction {
                    Interaction::MovingSelected { .. } => {
                        self.interaction = Interaction::None;
                        return Some(NodeGraphMessage::NodesDropped);
                    }
                    Interaction::SelectingNodes => {
                        self.interaction = Interaction::None;
                        return Some(NodeGraphMessage::CompleteSelection);
                    }
                    _ => {}
                },
                _ => {}
            }
            self.interaction = Interaction::None;
            return None;
        }

        let cursor_position = cursor.position_in(&bounds)?;

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(button) => match button {
                    mouse::Button::Middle => {
                        self.interaction = Interaction::Panning {
                            translation: self.translation,
                            start: cursor_position,
                        };

                        None
                    }
                    mouse::Button::Left => {
                        let grid_position = self.project(cursor_position, bounds.size());

                        if let Some(label) = self.node_at(&grid_position) {
                            self.interaction = Interaction::MovingSelected {
                                start: grid_position,
                            };
                            return Some(NodeGraphMessage::SelectNode(label));
                        }
                        self.interaction = Interaction::SelectingNodes;
                        Some(NodeGraphMessage::BeginSelecting(grid_position))
                    }
                    _ => None,
                },
                mouse::Event::CursorMoved { .. } => match &self.interaction {
                    Interaction::Panning { translation, start } => {
                        Some(NodeGraphMessage::Translate(
                            *translation + (cursor_position - *start) * (1.0 / self.scaling),
                        ))
                    }
                    Interaction::MovingSelected { start } => {
                        let grid_position = self.project(cursor_position, bounds.size());
                        let translation = grid_position - *start;
                        Some(NodeGraphMessage::TranslateSelected(translation))
                    }
                    Interaction::SelectingNodes => {
                        let grid_position = self.project(cursor_position, bounds.size());
                        Some(NodeGraphMessage::ExpandSelection(grid_position))
                    }
                    _ => None,
                },
                mouse::Event::WheelScrolled { delta } => match delta {
                    mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                        if y < 0.0 && self.scaling > Self::MIN_SCALING
                            || y > 0.0 && self.scaling < Self::MAX_SCALING
                        {
                            return Some(NodeGraphMessage::Zoom(
                                y,
                                cursor.position_from(bounds.center()),
                            ));
                        }
                        None
                    }
                },
                _ => None,
            },
        }
    }

    fn mouse_interaction(&self, _bounds: Rectangle, _cursor: Cursor) -> mouse::Interaction {
        match self.interaction {
            Interaction::Panning { .. } | Interaction::MovingSelected { .. } => {
                mouse::Interaction::Grabbing
            }
            _ => mouse::Interaction::default(),
        }
    }
}
