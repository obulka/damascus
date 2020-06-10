// 3rd Party Imports
use iced::{keyboard, pane_grid};

// Local Imports
pub mod panel;
pub mod tabs;

use crate::state::{style::Theme, widget::TabType};
use panel::Message as PanelMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Panel(PanelMessage),
    ThemeChanged(Theme),
    ToggleTheme,
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    PaneDragged(pane_grid::DragEvent),
    FloatPane(pane_grid::Pane),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    CloseFocused,
}

pub fn handle_hotkey(event: pane_grid::KeyPressEvent) -> Option<Message> {
    use keyboard::KeyCode;
    use pane_grid::Direction;

    let direction = match event.key_code {
        KeyCode::Up => Some(Direction::Up),
        KeyCode::Down => Some(Direction::Down),
        KeyCode::Left => Some(Direction::Left),
        KeyCode::Right => Some(Direction::Right),
        _ => None,
    };

    match event.key_code {
        KeyCode::V => Some(Message::Panel(PanelMessage::OpenTabFocused(
            TabType::Viewer,
        ))),
        KeyCode::G => Some(Message::Panel(PanelMessage::OpenTabFocused(
            TabType::NodeGraph,
        ))),
        KeyCode::T => Some(Message::ToggleTheme),
        KeyCode::W => Some(Message::CloseFocused),
        _ => direction.map(Message::FocusAdjacent),
    }
}
