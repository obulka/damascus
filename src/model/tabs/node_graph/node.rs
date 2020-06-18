// 3rd Party Imports
use iced::{Point, Rectangle, Vector};

// Local Imports
mod circle;
mod dot;
mod rect;

pub use circle::CircleNode;
pub use dot::DotNode;
pub use rect::RectNode;

use crate::view::node::NodeView;
use crate::update::node::NodeUpdate;

#[derive(Debug, Clone)]
pub enum NodeType {
    Dot,
    Read,
    Viewer,
}

pub fn create_node(node_type: NodeType) -> Box<dyn Node> {
    match node_type {
        NodeType::Dot => Box::new(DotNode::default()),
        NodeType::Viewer => Box::new(RectNode::default()),
        NodeType::Read => Box::new(CircleNode::default()),
    }
}

pub trait Node: NodeView + NodeUpdate {}

pub trait NodeModel {
    fn get_label(&self) -> &String;

    fn set_label(&mut self, label: String);

    fn select(&mut self);

    fn deselect(&mut self);

    fn selected(&self) -> bool;

    fn working(&self) -> bool;

    fn set_position(&mut self, position: Point);

    fn get_translation(&self) -> Vector {
        Vector::default()
    }

    fn set_translation(&mut self, translation: Vector);

    fn translate(&mut self);

    fn rect(&self) -> Rectangle {
        Rectangle::default()
    }

    fn snap(&mut self);

    fn position_vector(&self) -> Vector {
        let position = self.get_position();
        Vector::new(position.x, position.y)
    }

    fn get_position(&self) -> Point {
        self.rect().position()
    }

    fn contains(&self, point: Point) -> bool {
        self.rect().contains(point)
    }

    fn translated_rect(&self) -> Rectangle {
        self.rect() + self.get_translation()
    }

    fn intersection(&self, other_rectangle: &Rectangle) -> Option<Rectangle> {
        self.rect().intersection(other_rectangle)
    }

    fn parents(&self) -> Option<Vec<String>> {
        None
    }

    fn children(&self) -> Option<Vec<String>> {
        None
    }

    fn min_inputs(&self) -> usize {
        0
    }

    fn num_disconnected(&self) -> usize {
        if let Some(parents) = self.parents() {
            return parents.len() - self.min_inputs();
        }
        self.min_inputs()
    }

    fn get_connection_at(&self, _position: Point) -> Option<String> {
        None
    }
}
