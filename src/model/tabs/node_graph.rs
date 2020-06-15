use iced::{
    canvas::{Cache, Cursor, Event, Geometry, Program},
    mouse, Point, Rectangle, Size, Vector,
};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::ops::RangeInclusive; // Security not important

use super::TabContent;
use crate::model::{
    node::{create_node, Node, NodeType},
    CanvasModel, Config, Model,
};
use crate::update::{
    tabs::{
        node_graph::{Interaction, NodeGraphMessage},
        TabContentMessage,
    },
    CanvasUpdate,
};
use crate::view::CanvasView;

pub struct NodeGraph {
    pub nodes: HashMap<String, Box<dyn Node>>,
    pub selected_nodes: HashSet<String>,
    pub selection_box: Option<Rectangle>,
    pub grid_size: f32,
    pub interaction: Interaction,
    pub connection_cache: Cache,
    pub node_cache: Cache,
    pub grid_cache: Cache,
    pub selection_box_cache: Cache,
    pub translation: Vector,
    pub scaling: f32,
    pub show_lines: bool,
    pub config: Config,
}

impl Model<TabContentMessage> for NodeGraph {}
impl CanvasModel<NodeGraphMessage> for NodeGraph {}
impl TabContent for NodeGraph {}

impl Default for NodeGraph {
    fn default() -> Self {
        Self {
            nodes: HashMap::default(),
            selected_nodes: HashSet::default(),
            selection_box: None,
            grid_size: 20.0,
            interaction: Interaction::None,
            connection_cache: Cache::default(),
            node_cache: Cache::default(),
            grid_cache: Cache::default(),
            selection_box_cache: Cache::default(),
            translation: Vector::default(),
            scaling: 1.0,
            show_lines: true,
            config: Config::default(),
        }
    }
}

impl NodeGraph {
    pub const MIN_SCALING: f32 = 0.1;
    pub const MAX_SCALING: f32 = 2.0;

    pub fn new() -> Self {
        let mut node_graph = NodeGraph::default();
        for i in 0..5 {
            node_graph.add_node(NodeType::Viewer, Point::new(i as f32, i as f32));
        }
        node_graph
    }

    pub fn initialize_selection_box(&mut self, start_position: Point) {
        self.selection_box = Some(Rectangle::new(start_position, Size::new(0.0, 0.0)));
    }

    pub fn expand_selection_box(&mut self, to_position: Point) {
        if let Some(mut selection_box) = self.selection_box {
            let corner = selection_box.position();

            selection_box.width = to_position.x - corner.x;
            selection_box.height = to_position.y - corner.y;

            let mut top_left = corner;
            let mut top_left_size = selection_box.size();

            if selection_box.width < 0.0 {
                top_left.x += selection_box.width;
                top_left_size.width *= -1.0;
            }
            if selection_box.height < 0.0 {
                top_left.y += selection_box.height;
                top_left_size.height *= -1.0;
            }

            let top_left_rect = Rectangle::new(top_left, top_left_size);

            for (node_label, node) in self.nodes.iter() {
                if top_left_rect.contains(node.rect().center()) {
                    self.selected_nodes.insert(node_label.to_string());
                } else {
                    self.selected_nodes.remove(node_label);
                }
            }
            self.selection_box = Some(selection_box);
        }
        self.selection_box_cache.clear();
    }

    pub fn close_selection_box(&mut self) {
        self.selection_box = None;
        self.selection_box_cache.clear();
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

    pub fn move_selected(&mut self) {
        for node_label in self.selected_nodes.iter() {
            if let Some(node) = self.nodes.get_mut(node_label) {
                node.translate();
            }
        }
    }

    pub fn translate(&mut self, translation: Vector) {
        self.translation = translation;
    }

    pub fn translate_selected(&mut self, mut translation: Vector) {
        if self.show_lines {
            translation = Vector {
                x: translation.x.round(),
                y: translation.y.round(),
            };
        }
        for node_label in self.selected_nodes.iter() {
            if let Some(node) = self.nodes.get_mut(node_label) {
                node.set_translation(translation);
            }
        }
    }

    pub fn snap_nodes(&mut self) {
        for (_, node) in self.nodes.iter_mut() {
            node.snap();
        }
    }

    pub fn toggle_lines(&mut self) {
        self.show_lines = !self.are_lines_visible();
        if self.show_lines {
            self.snap_nodes();
            self.clear_node_caches();
        }
    }

    pub fn are_lines_visible(&self) -> bool {
        self.show_lines
    }

    pub fn zoom(&mut self, scroll_delta: f32, cursor_position: Option<Point>) {
        let old_scaling = self.scaling;

        self.scaling = (self.scaling * (1.0 + scroll_delta / 30.0))
            .max(Self::MIN_SCALING)
            .min(Self::MAX_SCALING);

        if let Some(cursor_to_center) = cursor_position {
            let factor = self.scaling - old_scaling;

            self.translation = self.translation
                - Vector::new(
                    cursor_to_center.x * factor / (old_scaling * old_scaling),
                    cursor_to_center.y * factor / (old_scaling * old_scaling),
                );
        }
    }

    pub fn visible_region(&self, size: Size) -> Region {
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

    pub fn project(&self, position: Point, size: Size) -> Point {
        let region = self.visible_region(size);

        Point::new(
            (position.x / self.scaling + region.x) / self.grid_size,
            (position.y / self.scaling + region.y) / self.grid_size,
        )
    }

    pub fn node_at(&self, position: &Point) -> Option<String> {
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

impl<'a> Program<NodeGraphMessage> for NodeGraph {
    fn update(
        &mut self,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Option<NodeGraphMessage> {
        CanvasUpdate::update(self, event, bounds, cursor)
    }

    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        CanvasView::draw(self, bounds, cursor)
    }

    fn mouse_interaction(&self, bounds: Rectangle, cursor: Cursor) -> mouse::Interaction {
        CanvasUpdate::mouse_interaction(self, bounds, cursor)
    }
}
