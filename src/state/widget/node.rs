use iced::{
    canvas::{Frame, Path, Stroke, Text},
    HorizontalAlignment, Point, Rectangle, Vector, VerticalAlignment,
};

use crate::state::style::{NodeGraphStyle, NodeStyle};

mod viewer;
pub use viewer::Viewer;

pub trait Node {
    fn set_position(&mut self, position: Point);

    fn set_translation(&mut self, translation: Vector);

    fn translate(&mut self);

    fn get_translation(&self) -> Vector;

    fn rect(&self) -> Rectangle;

    fn style(&self) -> NodeStyle {
        NodeStyle::default()
    }

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

    fn draw(
        &self,
        frame: &mut Frame,
        bounds: &Rectangle,
        label: Option<&String>,
        selected: bool,
        _working: bool,
        font_size: f32,
        node_graph_style: &NodeGraphStyle,
    ) {
        let translated_rect = self.rect() + self.get_translation();
        if let Some(rect) = bounds.intersection(&translated_rect) {
            let node_style = self.style();
            let node = Path::rectangle(rect.position(), rect.size());
            frame.fill(&node, node_style.background);
            frame.stroke(
                &node,
                Stroke {
                    width: node_graph_style.border_width,
                    color: if selected {
                        node_graph_style.selected_color
                    } else {
                        node_graph_style.border_color
                    },
                    ..Stroke::default()
                },
            );

            if let Some(label) = label {
                if bounds.contains(translated_rect.center()) {
                    frame.with_save(|frame| {
                        frame.translate(Vector::new(rect.center_x(), rect.center_y()));
                        frame.fill_text(Text {
                            content: label.to_string(),
                            color: node_style.text_color,
                            size: font_size,
                            horizontal_alignment: HorizontalAlignment::Center,
                            vertical_alignment: VerticalAlignment::Center,
                            ..Text::default()
                        })
                    });
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Read,
    Viewer,
}

impl From<NodeType> for String {
    fn from(node_type: NodeType) -> String {
        match node_type {
            NodeType::Read => "Read".to_string(),
            NodeType::Viewer => "Viewer".to_string(),
        }
    }
}

pub fn create_node(node_type: NodeType) -> Box<dyn Node> {
    match node_type {
        NodeType::Viewer => Box::new(Viewer::default()),
        NodeType::Read => Box::new(Viewer::default()),
    }
}
