// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced::widget::{Svg, svg};

use strum::EnumString;

#[derive(Debug, EnumString)]
pub enum Icons {
    ArrowLeft,
    ArrowRight,
    Close,
    Detach,
    File,
    HorizontalSplit,
    Maximize,
    Minimize,
    Pause,
    Play,
    Refresh,
    VerticalSplit,
}

impl Icons {
    pub fn as_svg(&self) -> Svg<'_> {
        svg(match *self {
            Self::ArrowLeft => concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/arrow_left.svg"),
            Self::ArrowRight => {
                concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/arrow_right.svg")
            }
            Self::Close => concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/close.svg"),
            Self::Detach => concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/detach.svg"),
            Self::File => concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/file.svg"),
            Self::HorizontalSplit => concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/icons/horizontal_split.svg"
            ),
            Self::Maximize => concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/maximize.svg"),
            Self::Minimize => concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/minimize.svg"),
            Self::Pause => concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/pause.svg"),
            Self::Play => concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/play.svg"),
            Self::Refresh => concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/refresh.svg"),
            Self::VerticalSplit => concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/icons/vertical_split.svg"
            ),
        })
    }

    // pub fn as_button<Message>(&self) -> Button<'static, Message> {

    // }
}
