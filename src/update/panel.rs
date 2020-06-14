// Standard Imports
use std::convert::TryFrom;

// 3rd Party Imports
use iced::{pane_grid, Command, Subscription};

// Local Imports
use super::{tabs::TabContentMessage, BaseMessage, Update};
use crate::model::panel::Panel;
use crate::view::widget::TabType;
use crate::DamascusError;

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

impl TryFrom<BaseMessage> for PanelMessage {
    type Error = &'static DamascusError;

    fn try_from(message: BaseMessage) -> Result<Self, Self::Error> {
        if let BaseMessage::Panel(message) = message {
            Ok(message)
        } else {
            Err(&DamascusError::UpdateError)
        }
    }
}

impl Update for Panel {
    type Message = TabContentMessage;

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
