// 3rd Party Imports
use iced::{pane_grid, keyboard};

// Local Imports
use crate::state::style::Theme;


#[derive(Debug, Clone)]
pub enum Message {
    AddTabFocused(String),
    ThemeChanged(Theme),
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    CloseFocused,
}


pub fn handle_hotkey(event: pane_grid::KeyPressEvent) -> Option<Message> {
    use keyboard::KeyCode;
    use pane_grid::{Axis, Direction};

    let direction = match event.key_code {
        KeyCode::Up => Some(Direction::Up),
        KeyCode::Down => Some(Direction::Down),
        KeyCode::Left => Some(Direction::Left),
        KeyCode::Right => Some(Direction::Right),
        _ => None,
    };

    match event.key_code {
        KeyCode::V => Some(Message::SplitFocused(Axis::Vertical)),
        KeyCode::H => Some(Message::SplitFocused(Axis::Horizontal)),
        KeyCode::W => Some(Message::CloseFocused),
        _ => direction.map(Message::FocusAdjacent),
    }
}


// impl Action
// {
//     fn handle_event(event: Event, model: &mut Model)
//     {
//         let proposal = match event {
//             Event::MouseClick {..} => Action::CreateWidget{widget: Widget{}},
//             Event::NothingHappened(..) => Action::CloseWidget{widget: Widget{}},
//         };
//         model.submit(proposal);
//     }
// }

