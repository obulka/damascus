// 3rd Party Imports
use iced::{canvas::Frame, Rectangle};

// Local Imports
use crate::model::node::Viewer;
use crate::view::{node::NodeView, CanvasItemView, Config};

impl NodeView for Viewer {}

impl CanvasItemView for Viewer {
    fn draw(&self, frame: &mut Frame, bounds: &Rectangle, render_text: bool, config: &Config) {
        NodeView::rectangular_draw(self, frame, bounds, render_text, config);
    }
}
