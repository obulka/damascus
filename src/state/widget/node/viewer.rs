use super::Node;
use iced::{Point, Rectangle, Size, Vector};

pub struct Viewer {
    rectangle: Rectangle,
}

impl Viewer {
    pub fn position(mut self, position: Point) -> Self {
        self.rectangle = Rectangle {
            x: position.x,
            y: position.y,
            ..self.rectangle
        };
        self
    }
}

impl Default for Viewer {
    fn default() -> Self {
        Self {
            rectangle: Rectangle::with_size(Size::new(5.0, 3.0)),
        }
    }
}

impl Node for Viewer {
    fn rect(&self) -> Rectangle {
        self.rectangle
    }

    fn set_position(&mut self, position: Point) {
        self.rectangle.x = position.x;
        self.rectangle.y = position.y;
    }

    fn translate(&mut self, translation: Vector) {
        self.rectangle = self.rectangle + translation;
    }
}
