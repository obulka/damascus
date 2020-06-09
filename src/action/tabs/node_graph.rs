pub mod grid;

use grid::Message as GridMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Grid(GridMessage),
    Next,
    Clear,
}
