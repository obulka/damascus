// 3rd Party Imports
use iced::{canvas::Program, pane_grid};

// Local Imports
use crate::update::{CanvasItemUpdate, CanvasUpdate, Message, Update};
use crate::view::{CanvasItemView, CanvasView, Config, View};
use panel::Panel;

pub mod panel;
pub mod tabs;

mod widget;

pub use widget::*;
pub use tabs::node_graph::node;

pub trait Model<UpdateMessage>: Update<UpdateMessage> + View {}
pub trait CanvasModel<UpdateMessage>:
    CanvasUpdate<UpdateMessage> + CanvasView + Program<UpdateMessage>
{
}

pub trait CanvasItemModel: CanvasItemView + CanvasItemUpdate {}

pub struct Damascus {
    pub config: Config,
    pub panes: pane_grid::State<Panel>,
}

impl Model<Message> for Damascus {}
