// 3rd Party Imports
use iced::{
    canvas::{Frame, Path, Stroke, Text},
    HorizontalAlignment, Rectangle, Vector, VerticalAlignment,
};

// Local Imports
mod viewer;

use crate::view::{theme::{NodeGraphStyle, NodeStyle}};

pub trait NodeView {
    fn get_translation(&self) -> Vector {
        Vector::default()
    }

    fn set_translation(&mut self, translation: Vector);

    fn rect(&self) -> Rectangle {
        Rectangle::default()
    }

    fn style(&self) -> NodeStyle {
        NodeStyle::default()
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
