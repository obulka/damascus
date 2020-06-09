use crate::model::tabs::viewer::grid;
use std::time::Instant;

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
