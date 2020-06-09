// 3rd Party Imports
use iced::{pane_grid, keyboard};

// Local Imports
pub mod tabs;

use crate::state::{
    style::Theme,
    widget::TabType,
};
use tabs::Message as TabContentMessage;


#[derive(Debug, Clone)]
pub enum Message {
    TabContent(TabContentMessage),
    MoveTab((pane_grid::Pane, usize, pane_grid::Pane)),
    OpenTabFocused(TabType),
    CloseTab(pane_grid::Pane, usize),
    FocusTab((pane_grid::Pane, usize)),
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
        KeyCode::V => Some(Message::OpenTabFocused(TabType::Viewer)),
        KeyCode::G => Some(Message::OpenTabFocused(TabType::NodeGraph)),
        KeyCode::T => Some(Message::ToggleTheme),
        KeyCode::W => Some(Message::CloseFocused),
        _ => direction.map(Message::FocusAdjacent),
    }
}
