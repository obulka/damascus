// Local Imports
mod circle;
mod dot;
mod rect;

use crate::model::node::NodeModel;

pub trait NodeUpdate: NodeModel {}
