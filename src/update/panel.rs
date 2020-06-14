// 3rd Party Imports
use iced::{pane_grid, Command, Subscription};

// Local Imports
use super::{tabs::TabContentMessage, BaseMessage, Update};
use crate::model::panel::Panel;
use crate::view::widget::TabType;

pub trait PanelUpdate: Update<TabContentMessage> {
    fn update_view_state(&mut self, pane: pane_grid::Pane, focus: Option<pane_grid::Focus>);
}

#[derive(Debug, Clone)]
pub enum PanelMessage {
    TabContent(TabContentMessage),
    MoveTab((pane_grid::Pane, usize, pane_grid::Pane)),
    OpenTabFocused(TabType),
    CloseTab(pane_grid::Pane, usize),
    FocusTab((pane_grid::Pane, usize)),
}

impl From<PanelMessage> for BaseMessage {
    fn from(message: PanelMessage) -> BaseMessage {
        BaseMessage::Panel(message)
    }
}

impl PanelUpdate for Panel {
    fn update_view_state(&mut self, pane: pane_grid::Pane, focus: Option<pane_grid::Focus>) {
        self.pane = Some(pane);
        self.focus = focus.is_some();
    }
}

impl Update<TabContentMessage> for Panel {
    fn update(&mut self, message: TabContentMessage) -> Command<BaseMessage> {
        if let Some(focused_label) = self.get_focused_label() {
            if message == *focused_label {
                if let Some(focused_content) = self.get_mut_focused_content() {
                    return focused_content.update(message);
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<BaseMessage> {
        if let Some(focused_content) = self.get_focused_content() {
            return focused_content.subscription();
        }
        Subscription::none()
    }
}
