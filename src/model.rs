// 3rd Party Imports
use iced::pane_grid;

// Local Imports
use crate::update::{CanvasUpdate, Message, Update};
use crate::view::{Config, CanvasView, View};
use panel::Panel;

pub mod node;
pub mod panel;
pub mod tabs;

pub trait Model<UpdateMessage>: Update<UpdateMessage> + View {}
pub trait CanvasModel<UpdateMessage>: CanvasUpdate<UpdateMessage> + CanvasView {}

pub struct Damascus {
    pub config: Config,
    pub panes: pane_grid::State<Panel>,
}

impl Model<Message> for Damascus {}
