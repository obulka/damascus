// 3rd Party Imports
use iced::{button, canvas::Program, pane_grid};
use rustc_hash::FxHashMap as HashMap;

// Local Imports
use crate::model::tabs::{TabContent, TabType};
use crate::view::Config;
use panel::Panel;

pub mod core;
pub mod panel;
pub mod tabs;

mod widget;

pub use tabs::node_graph::node;
pub use widget::*;

pub trait Model {}
pub trait CanvasModel<UpdateMessage>: Program<UpdateMessage> {}

pub struct Damascus {
    pub config: Config,
    pub panes: pane_grid::State<Panel>,
    pub tabs: HashMap<String, pane_grid::Pane>,
}

impl Model for Damascus {}

impl Damascus {
    pub fn close_tab(&mut self, tab_label: &String) -> Option<String> {
        if let Some(pane) = self.tabs.get(tab_label) {
            if let Some(panel) = self.panes.get_mut(&pane) {
                let (new_focus, _, _) = (*panel).close_tab(tab_label);
                self.tabs.remove(tab_label);
                return new_focus;
            }
        }
        None
    }

    pub fn move_tab(&mut self, tab_label: &String, target_pane: pane_grid::Pane) -> Option<String> {
        if let Some(pane) = self.tabs.get(tab_label) {
            if let Some(panel) = self.panes.get_mut(&pane) {
                let (new_focus, tab, tab_content) = (*panel).close_tab(tab_label);
                if let Some(target_panel) = self.panes.get_mut(&target_pane) {
                    (*target_panel).open_tab_with_content(tab, tab_content);
                    let key = self.tabs.get_mut(tab_label).unwrap(); // Should never be None
                    *key = target_pane;
                }
                return new_focus;
            }
        }
        None
    }

    pub fn open_tab_focused(&mut self, tab_type: TabType) {
        if let Some(active_pane) = self.panes.active() {
            if let Some(panel) = self.panes.get_mut(&active_pane) {
                let default_label: String = tab_type.clone().into();
                let mut label = default_label.clone();
                let mut count = 0;
                while self.tabs.contains_key(&label) {
                    label = format!("{}{}", default_label, count);
                    count += 1;
                }
                self.tabs.insert(label.clone(), active_pane);
                let mut tab_content: Box<dyn TabContent> = tab_type.into();
                tab_content.set_id(label.clone());
                (*panel).open_tab_with_content((label, button::State::new()), tab_content);
            }
        }
    }
}
