// Standard Imports
use std::f32::consts::PI;

// 3rd Party Imports
use iced::{
    canvas::{Frame, LineCap, Path, Stroke, Text},
    HorizontalAlignment, Point, Rectangle, Vector, VerticalAlignment,
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

    fn connection_stroke(&self, node_graph_style: NodeGraphStyle) -> Stroke {
        Stroke {
            width: node_graph_style.connection_width,
            color: node_graph_style.connection_color,
            line_cap: LineCap::Butt,
            ..Stroke::default()
        }
    }

    fn connection_path(&self) -> Path {
        Path::line(Point::ORIGIN, Point::new(2.0, 0.0))
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

    fn draw_connections(
        &self,
        frame: &mut Frame,
        _bounds: &Rectangle,
        _render_text: bool,
        config: &Config,
    ) {
        let num_disconnected = self.num_disconnected();
        if num_disconnected > 0 {
            let rotation = -PI / 4.0;
            let rotation_step = if num_disconnected > 1 {
                -PI / (2.0 * (num_disconnected - 1) as f32)
            } else {
                0.0
            };
            let node_position = self.translated_rect().center();
            let input_location = Vector::new(node_position.x, node_position.y);
            frame.with_save(|frame| {
                frame.translate(input_location);
                frame.rotate(rotation);
                for _ in 0..num_disconnected {
                    frame.stroke(
                        &self.connection_path(),
                        self.connection_stroke(config.theme.into()),
                    );
                    frame.rotate(rotation_step);
                }
            })
        }
    }
}
