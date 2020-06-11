use super::Node;
use iced::{Color, Point, Rectangle, Size, Vector};

pub struct Viewer {
    label: String,
    color: Color,
    text_color: Color,
    text_size: f32,
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
            label: "Viewer".to_string(),
            color: Color::WHITE,
            text_color: Color::BLACK,
            text_size: 14.0,
            rectangle: Rectangle::new(Point::ORIGIN, Size::new(5.0, 3.0)),
        }
    }
}

impl Node for Viewer {
    fn label(&self) -> &str {
        &self.label
    }

    fn color(&self) -> Color {
        self.color
    }

    fn text_color(&self) -> Color {
        self.text_color
    }

    fn text_size(&self) -> f32 {
        self.text_size
    }

    fn rect(&self) -> Rectangle {
        self.rectangle
    }

    fn translate(&mut self, translation: Vector) {
        self.rectangle = self.rectangle + translation;
    }
}
