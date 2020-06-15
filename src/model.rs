// 3rd Party Imports
use iced::{canvas::Program, pane_grid};

// Local Imports
use crate::update::{CanvasUpdate, Message, Update};
use crate::view::{CanvasView, Config, View};
use panel::Panel;

pub mod node;
pub mod panel;
pub mod tabs;

mod widget;

pub use widget::*;

pub trait Model<UpdateMessage>: Update<UpdateMessage> + View {}
pub trait CanvasModel<UpdateMessage>:
    CanvasUpdate<UpdateMessage> + CanvasView + Program<UpdateMessage>
{
}

pub struct Damascus {
    pub config: Config,
    pub panes: pane_grid::State<Panel>,
}

impl Model<Message> for Damascus {}
