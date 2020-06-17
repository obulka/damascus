// 3rd Party Imports
use iced::{
    canvas::{Frame, Path, Stroke, Text},
    HorizontalAlignment, Rectangle, Vector, VerticalAlignment,
};

// Local Imports
mod circle;
mod dot;
mod rect;

use crate::model::node::NodeState;
use crate::view::{
    theme::{NodeGraphStyle, NodeStyle},
    Config,
};

pub trait NodeView: NodeState {
    fn style(&self) -> NodeStyle {
        NodeStyle::default()
    }

    fn get_path(&self) -> Path {
        let rect = self.translated_rect();
        Path::rectangle(rect.position(), rect.size())
    }

    fn draw(&self, frame: &mut Frame, bounds: &Rectangle, render_text: bool, config: &Config) {
        let rect = self.translated_rect();
        if let Some(_) = bounds.intersection(&rect) {
            let node_graph_style: NodeGraphStyle = config.theme.into();
            let node_style = self.style();
            let node = self.get_path();
            frame.fill(&node, node_style.background);
            frame.stroke(
                &node,
                Stroke {
                    width: node_graph_style.border_width,
                    color: if self.selected() {
                        node_graph_style.selected_color
                    } else {
                        node_graph_style.border_color
                    },
                    ..Stroke::default()
                },
            );

            if render_text {
                if bounds.contains(rect.center()) {
                    frame.with_save(|frame| {
                        frame.translate(Vector::new(rect.center_x(), rect.center_y()));
                        // Note that text will be overlayed until iced supports vectorial text
                        frame.fill_text(Text {
                            content: self.get_label().to_string(),
                            color: node_style.text_color,
                            size: config.font_size as f32,
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
