// Local Imports
mod circle;
mod rect;

use crate::model::node::NodeState;
use crate::view::{theme::NodeStyle, CanvasItemView};

pub trait NodeView: CanvasItemView + NodeState {
    fn style(&self) -> NodeStyle {
        NodeStyle::default()
    }
}
