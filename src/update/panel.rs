// 3rd Party Imports
use iced::{pane_grid, Command, Subscription};

// Local Imports
use super::{tabs::TabContentMessage, Message, Update};
use crate::model::panel::Panel;

pub trait PanelUpdate: Update<TabContentMessage> {
    fn update_view_state(&mut self, pane: pane_grid::Pane, focus: Option<pane_grid::Focus>);
}

impl PanelUpdate for Panel {
    fn update_view_state(&mut self, pane: pane_grid::Pane, focus: Option<pane_grid::Focus>) {
        self.pane = Some(pane);
        self.focus = focus.is_some();
    }
}

impl Update<TabContentMessage> for Panel {
    fn update(&mut self, message: TabContentMessage) -> Command<Message> {
        if let Some(focused_label) = self.get_focused_label() {
            if let Some(focused_content) = self.get_mut_focused_content() {
                if *focused_content.get_id() == focused_label {
                    return focused_content.update(message);
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if let Some(focused_content) = self.get_focused_content() {
            return focused_content.subscription();
        }
        Subscription::none()
    }
}
