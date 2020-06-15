// 3rd Party Imports
use iced::{Rectangle, Vector};

// Local Imports
use crate::view::node::NodeView;
use crate::model::node::Viewer;

impl NodeView for Viewer {
    fn get_translation(&self) -> Vector {
        self.translation
    }

    fn set_translation(&mut self, translation: Vector) {
        self.translation = translation;
    }

    fn rect(&self) -> Rectangle {
        self.rectangle
    }
}
