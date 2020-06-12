use iced::{
    canvas::{self, Cache, Canvas, Cursor, Event, Geometry, Path, Stroke},
    mouse, Element, Length, Point, Rectangle, Size, Vector,
};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet}; // Security not important
use std::ops::RangeInclusive;

use crate::action::tabs::node_graph::Message;
use crate::state::{
    node::{create_node, NodeType},
    Config, Node,
};

pub struct State {
    nodes: HashMap<String, Box<dyn Node>>,
    selected_nodes: HashSet<String>,
    grid_size: f32,
    interaction: Interaction,
    connection_cache: Cache,
    node_cache: Cache,
    grid_cache: Cache,
    translation: Vector,
    scaling: f32,
    show_lines: bool,
    config: Config,
}

impl Default for State {
    fn default() -> Self {
        Self {
            nodes: HashMap::default(),
            selected_nodes: HashSet::default(),
            grid_size: 20.0,
            interaction: Interaction::None,
            connection_cache: Cache::default(),
            node_cache: Cache::default(),
            grid_cache: Cache::default(),
            translation: Vector::default(),
            scaling: 1.0,
            show_lines: true,
            config: Config::default(),
        }
    }
}

impl State {
    const MIN_SCALING: f32 = 0.1;
    const MAX_SCALING: f32 = 2.0;

    pub fn new(nodes: HashMap<String, Box<dyn Node>>) -> Self {
        Self {
            nodes: nodes,
            ..Self::default()
        }
    }

    pub fn view<'a>(&'a mut self, config: &Config) -> Element<'a, Message> {
        self.config = *config;
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn deselect_node(&mut self, label: String) {
        self.selected_nodes.remove(&label);
    }

    pub fn select_node(&mut self, label: String) {
        self.selected_nodes.insert(label);
    }

    pub fn clear_selected(&mut self) {
        self.selected_nodes.clear();
    }

    pub fn clear_node_caches(&mut self) {
        self.node_cache.clear();
        self.connection_cache.clear();
    }

    pub fn clear_cache(&mut self) {
        self.node_cache.clear();
        self.connection_cache.clear();
        self.grid_cache.clear();
    }

    pub fn toggle_lines(&mut self) {
        self.show_lines = !self.are_lines_visible();
    }

    pub fn are_lines_visible(&self) -> bool {
        self.show_lines
    }

    fn visible_region(&self, size: Size) -> Region {
        let width = size.width / self.scaling;
        let height = size.height / self.scaling;

        Region {
            x: -self.translation.x - width / 2.0,
            y: -self.translation.y - height / 2.0,
            width: width,
            height: height,
            grid_size: self.grid_size,
        }
    }

    fn project(&self, position: Point, size: Size) -> Point {
        let region = self.visible_region(size);

        Point::new(
            (position.x / self.scaling + region.x) / self.grid_size,
            (position.y / self.scaling + region.y) / self.grid_size,
        )
    }

    fn node_at(&self, position: &Point) -> Option<String> {
        for (label, node) in self.nodes.iter() {
            if node.contains(*position) {
                return Some(label.clone());
            }
        }
        None
    }

    pub fn add_node(&mut self, node_type: NodeType, position: Point) {
        let default_label: String = node_type.clone().into();
        let mut label = default_label.clone();

        let mut count = 0;
        while self.nodes.contains_key(&label) {
            label = format!("{}{}", default_label, count);
            count += 1;
        }
        let mut node = create_node(node_type);
        node.set_position(position);

        self.nodes.insert(label, node);
    }
}

impl<'a> canvas::Program<Message> for State {
    fn update(&mut self, event: Event, bounds: Rectangle, cursor: Cursor) -> Option<Message> {
        if let Event::Mouse(mouse::Event::ButtonReleased(button)) = event {
            match button {
                mouse::Button::Left => {
                    match self.interaction {
                        Interaction::MovingSelected{ .. } => {
                            for node_label in self.selected_nodes.iter() {
                                if let Some(node) = self.nodes.get_mut(node_label) {
                                    node.translate();
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    self.interaction = Interaction::None;
                    return None;
                }
            }
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
                                position: grid_position,
                            };
                            return Some(Message::SelectNode(label));
                        }
                        self.interaction = Interaction::SelectingNodes {
                            start: cursor_position,
                        };
                        Some(Message::ClearSelected)
                    }
                    _ => None,
                },
                mouse::Event::CursorMoved { .. } => match &self.interaction {
                    Interaction::Panning { translation, start } => {
                        self.translation =
                            *translation + (cursor_position - *start) * (1.0 / self.scaling);

                        Some(Message::ClearCache)
                    }
                    Interaction::MovingSelected {
                        position,
                    } => {
                        let grid_position = self.project(cursor_position, bounds.size());
                        let translation = grid_position - *position;
                        for node_label in self.selected_nodes.iter() {
                            if let Some(node) = self.nodes.get_mut(node_label) {
                                node.set_translation(translation);
                            }
                        }
                        Some(Message::ClearNodeCaches)
                    }
                    _ => None,
                },
                mouse::Event::WheelScrolled { delta } => match delta {
                    mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                        if y < 0.0 && self.scaling > Self::MIN_SCALING
                            || y > 0.0 && self.scaling < Self::MAX_SCALING
                        {
                            let old_scaling = self.scaling;

                            self.scaling = (self.scaling * (1.0 + y / 30.0))
                                .max(Self::MIN_SCALING)
                                .min(Self::MAX_SCALING);

                            if let Some(cursor_to_center) = cursor.position_from(bounds.center()) {
                                let factor = self.scaling - old_scaling;

                                self.translation = self.translation
                                    - Vector::new(
                                        cursor_to_center.x * factor / (old_scaling * old_scaling),
                                        cursor_to_center.y * factor / (old_scaling * old_scaling),
                                    );
                            }

                            return Some(Message::ClearCache);
                        }

                        None
                    }
                },
                _ => None,
            },
        }
    }

    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);
        let lower_lod = self.scaling < 0.6;

        let nodes = self.node_cache.draw(bounds.size(), |frame| {
            frame.with_save(|frame| {
                frame.translate(center);
                frame.scale(self.scaling);
                frame.translate(self.translation);
                frame.scale(self.grid_size);
                let width = 1.0 / self.grid_size;
                frame.translate(Vector::new(-width / 2.0, -width / 2.0));

                let visible_bounds: Rectangle = self.visible_region(frame.size()).into();

                let font_size = self.config.font_size as f32;
                let node_graph_style = &self.config.theme.into();
                for (label, node) in self.nodes.iter() {
                    node.draw(
                        frame,
                        &visible_bounds,
                        if lower_lod { None } else { Some(label) },
                        self.selected_nodes.contains(label),
                        false,
                        font_size,
                        node_graph_style,
                    );
                }
            })
        });

        if lower_lod || !self.show_lines {
            vec![nodes]
        } else {
            let grid = self.grid_cache.draw(bounds.size(), |frame| {
                frame.translate(center);
                frame.scale(self.scaling);
                frame.translate(self.translation);
                frame.scale(self.grid_size);

                let region = self.visible_region(frame.size());
                let rows = region.rows();
                let columns = region.columns();
                let width = 1.0 / self.grid_size;
                let color = self.config.theme.secondary_color();

                frame.translate(Vector::new(-width / 2.0, -width / 2.0));

                for row in region.rows() {
                    let line = Path::line(
                        Point::new(*columns.start() as f32, row as f32),
                        Point::new(*columns.end() as f32, row as f32),
                    );
                    frame.stroke(
                        &line,
                        Stroke {
                            width: 1.0,
                            color: color,
                            ..Stroke::default()
                        },
                    );
                }

                for column in region.columns() {
                    let line = Path::line(
                        Point::new(column as f32, *rows.start() as f32),
                        Point::new(column as f32, *rows.end() as f32),
                    );
                    frame.stroke(
                        &line,
                        Stroke {
                            width: 1.0,
                            color: color,
                            ..Stroke::default()
                        },
                    );
                }
            });

            vec![grid, nodes]
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

pub struct Region {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    grid_size: f32,
}

impl Region {
    fn rows(&self) -> RangeInclusive<isize> {
        let first_row = (self.y / self.grid_size).floor() as isize;

        let visible_rows = (self.height / self.grid_size).ceil() as isize;

        first_row..=first_row + visible_rows + 1
    }

    fn columns(&self) -> RangeInclusive<isize> {
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

enum Interaction {
    None,
    Panning {
        translation: Vector,
        start: Point,
    },
    MovingSelected {
        position: Point,
    },
    SelectingNodes {
        start: Point,
    },
}
