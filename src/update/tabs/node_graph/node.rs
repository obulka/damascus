// Local Imports
mod circle;
mod rect;

use crate::update::CanvasItemUpdate;

pub trait NodeUpdate: CanvasItemUpdate {}
