// 3rd Party Imports
use iced::{Point, Rectangle, Size, Vector};

// Local Imports
use crate::model::node::{Node, NodeModel};

pub struct DotNode {
    pub rectangle: Rectangle,
    pub translation: Vector,
    label: String,
    selected: bool,
    working: bool,
}

impl DotNode {
    pub fn position(mut self, position: Point) -> Self {
        self.rectangle = Rectangle {
            x: position.x,
            y: position.y,
            ..self.rectangle
        };
        self
    }
}

impl Default for DotNode {
    fn default() -> Self {
        Self {
            rectangle: Rectangle::with_size(Size::new(1.0, 1.0)),
            translation: Vector::default(),
            label: "".to_string(),
            selected: false,
            working: false,
        }
    }
}

impl Node for DotNode {}

impl NodeModel for DotNode {
    fn get_label(&self) -> &String {
        &self.label
    }

    fn set_label(&mut self, label: String) {
        self.label = label;
    }

    fn select(&mut self) {
        self.selected = true;
    }

    fn deselect(&mut self) {
        self.selected = false;
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn working(&self) -> bool {
        self.working
    }

    fn snap(&mut self) {
        self.rectangle.x = self.rectangle.x.round();
        self.rectangle.y = self.rectangle.y.round();
    }

    fn set_position(&mut self, position: Point) {
        self.rectangle.x = position.x;
        self.rectangle.y = position.y;
    }

    fn set_translation(&mut self, translation: Vector) {
        self.translation = translation;
    }

    fn translate(&mut self) {
        self.rectangle = self.rectangle + self.translation;
        self.translation = Vector::default();
    }

    fn get_translation(&self) -> Vector {
        self.translation
    }

    fn rect(&self) -> Rectangle {
        self.rectangle
    }

    fn parents(&self) -> Option<Vec<String>> {
        None
    }

    fn children(&self) -> Option<Vec<String>> {
        None
    }

    fn min_inputs(&self) -> usize {
        1
    }

    fn max_inputs(&self) -> usize {
        1
    }
}
