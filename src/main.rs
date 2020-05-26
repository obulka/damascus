// 3rd Party Imports
extern crate lazy_static;
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
