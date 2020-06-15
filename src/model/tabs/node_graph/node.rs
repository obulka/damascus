// 3rd Party Imports
use iced::{Point, Rectangle, Vector};

// Local Imports
use crate::model::CanvasItemModel;
use crate::update::node::NodeUpdate;
use crate::view::node::NodeView;

mod viewer;
pub use viewer::Viewer;

#[derive(Debug, Clone)]
pub enum NodeType {
    Read,
    Viewer,
}

pub fn create_node(node_type: NodeType) -> Box<dyn Node> {
    match node_type {
        NodeType::Viewer => Box::new(Viewer::default()),
        NodeType::Read => Box::new(Viewer::default()),
    }
}

pub trait NodeState {
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

    fn intersection(&self, other_rectangle: &Rectangle) -> Option<Rectangle> {
        self.rect().intersection(other_rectangle)
    }
}

pub trait Node: CanvasItemModel + NodeUpdate + NodeView {}
