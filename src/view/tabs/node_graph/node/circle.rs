// 3rd Party Imports
use iced::{
    canvas::{Frame, Path, Stroke, Text},
    HorizontalAlignment, Rectangle, Vector, VerticalAlignment,
};

// Local Imports
use crate::model::node::{NodeState, CircleNode};
use crate::view::{node::NodeView, theme::NodeGraphStyle, CanvasItemView, Config};

impl NodeView for CircleNode {}

impl CanvasItemView for CircleNode {
    fn draw(&self, frame: &mut Frame, bounds: &Rectangle, render_text: bool, config: &Config) {
        let translated_rect = self.rect() + self.get_translation();
        if let Some(rect) = bounds.intersection(&translated_rect) {
            let node_graph_style: NodeGraphStyle = config.theme.into();
            let node_style = self.style();
            let node = Path::rectangle(rect.position(), rect.size());
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
                if bounds.contains(translated_rect.center()) {
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
