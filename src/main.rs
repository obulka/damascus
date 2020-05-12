// 3rd Party Imports
use iced::{
    Application,
    Settings,
};
// use tokio::sync::{mpsc, oneshot};

// Local Imports
use damascus::Damascus;


pub fn main() {
    Damascus::run(Settings::default())
}
