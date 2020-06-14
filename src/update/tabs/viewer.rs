// // Standard Imports
// use std::convert::TryFrom;
// use std::time::Instant;

// // Local Imports
// use crate::model::tabs::viewer::grid;
// use crate::update::{panel::PanelMessage, tabs::TabContentMessage, BaseMessage};
// use crate::DamascusError;

// #[derive(Debug, Clone)]
// pub enum ViewerMessage {
//     Grid(grid::Message),
//     Tick(Instant),
//     TogglePlayback,
//     ToggleGrid(bool),
//     Next,
//     Clear,
//     SpeedChanged(f32),
// }

// impl From<ViewerMessage> for TabContentMessage {
//     fn from(message: ViewerMessage) -> TabContentMessage {
//         TabContentMessage::Viewer(message)
//     }
// }

// impl From<ViewerMessage> for PanelMessage {
//     fn from(message: ViewerMessage) -> PanelMessage {
//         let message: TabContentMessage = message.into();
//         message.into()
//     }
// }

// impl From<ViewerMessage> for BaseMessage {
//     fn from(message: ViewerMessage) -> BaseMessage {
//         let message: PanelMessage = message.into();
//         message.into()
//     }
// }

// impl TryFrom<BaseMessage> for ViewerMessage {
//     type Error = &'static DamascusError;

//     fn try_from(message: BaseMessage) -> Result<Self, Self::Error> {
//         if let BaseMessage::Panel(PanelMessage::TabContent(TabContentMessage::Viewer(message))) =
//             message
//         {
//             Ok(message)
//         } else {
//             Err(&DamascusError::UpdateError)
//         }
//     }
// }
