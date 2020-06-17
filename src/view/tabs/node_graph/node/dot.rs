// 3rd Party Imports
use iced::{
    canvas::{Frame, Path, Stroke},
    Rectangle,
};

// Local Imports
use crate::model::node::{DotNode, NodeState};
use crate::view::{node::NodeView, theme::NodeGraphStyle, Config};

impl NodeView for DotNode {
    fn get_path(&self) -> Path {
        let rect = self.translated_rect();
        Path::circle(rect.center(), rect.size().width / 2.0)
    }

    fn draw(&self, frame: &mut Frame, bounds: &Rectangle, _render_text: bool, config: &Config) {
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
        }
    }
}
