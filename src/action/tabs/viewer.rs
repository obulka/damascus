use std::time::Instant;
use crate::model::tabs::viewer::grid;

#[derive(Debug, Clone)]
pub enum Message {
    Grid(grid::Message),
    Tick(Instant),
    TogglePlayback,
    ToggleGrid(bool),
    Next,
    Clear,
    SpeedChanged(f32),
}
