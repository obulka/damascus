// 3rd Party Imports
use iced::canvas::Path;

// Local Imports
use crate::model::node::{CircleNode, NodeModel};
use crate::view::{
    node::NodeView,
    theme::{NodeStyle, CIRCLE_NODE_STYLE},
};

impl NodeView for CircleNode {
    fn style(&self) -> NodeStyle {
        CIRCLE_NODE_STYLE
    }

    fn get_path(&self) -> Path {
        let rect = self.translated_rect();
        Path::circle(rect.center(), rect.size().width / 2.0)
    }
}
