// 3rd Party Imports
use iced::pane_grid;

// Local Imports
pub mod panel;
pub mod tabs;

use crate::update::Update;
use crate::view::{style::Theme, View};
use panel::Panel;

pub trait Model: Update + View {}

pub struct Damascus {
    pub config: Config,
    pub panes: pane_grid::State<Panel>,
}

impl Model for Damascus {}

#[derive(Debug, Clone)]
pub struct Config {
    pub font_size: u16,
    pub tab_bar_height: u32,
    pub theme: Theme,
    pub title: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            font_size: 13,
            tab_bar_height: 36,
            theme: Theme::default(),
            title: "Damascus".to_string(),
        }
    }
}
