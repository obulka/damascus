// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use eframe::egui::{self, include_image};

pub const PAUSE_ICON: egui::ImageSource<'_> = include_image!("../assets/icons/pause.svg");
pub const PLAY_ICON: egui::ImageSource<'_> = include_image!("../assets/icons/play.svg");
