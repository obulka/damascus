// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui::{self, include_image};
use strum::EnumString;

#[derive(Debug, EnumString)]
pub enum Icons {
    Pause,
    Play,
    ArrowRight,
    ArrowLeft,
    Refresh,
}

impl Icons {
    pub fn source(&self) -> egui::ImageSource<'_> {
        match *self {
            Self::Pause => include_image!("../assets/icons/pause.svg"),
            Self::Play => include_image!("../assets/icons/play.svg"),
            Self::ArrowRight => include_image!("../assets/icons/arrow_right.svg"),
            Self::ArrowLeft => include_image!("../assets/icons/arrow_left.svg"),
            Self::Refresh => include_image!("../assets/icons/refresh.svg"),
        }
    }

    pub fn forget(&self, ctx: &egui::Context) {
        ctx.forget_image(self.source().uri().unwrap());
    }
}
