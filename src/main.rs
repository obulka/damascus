// 3rd Party Imports
use iced::{Application, Settings};

// Local Imports
use damascus::Damascus;

pub fn main() {
    Damascus::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
