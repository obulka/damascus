use super::Node;
use iced::{Point, Rectangle, Size, Vector};

pub struct Viewer {
    rectangle: Rectangle,
    translation: Vector,
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
            rectangle: Rectangle::with_size(Size::new(4.0, 1.0)),
            translation: Vector::default(),
        }
    }
}

impl Node for Viewer {
    fn rect(&self) -> Rectangle {
        self.rectangle
    }

    fn snap(&mut self) {
        self.rectangle.x = self.rectangle.x.round();
        self.rectangle.y = self.rectangle.y.round();
    }

    fn set_position(&mut self, position: Point) {
        self.rectangle.x = position.x;
        self.rectangle.y = position.y;
    }

    fn translate(&mut self) {
        self.rectangle = self.rectangle + self.translation;
        self.translation = Vector::default();
    }

    fn set_translation(&mut self, translation: Vector) {
        self.translation = translation;
    }

    fn get_translation(&self) -> Vector {
        self.translation
    }
}
