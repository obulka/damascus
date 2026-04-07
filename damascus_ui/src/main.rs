// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use iced;

use damascus_ui::app::Damascus;

fn main() -> iced::Result {
    iced::daemon(Damascus::new, Damascus::update, Damascus::view)
        .subscription(Damascus::subscription)
        .title(Damascus::title)
        .default_font(Damascus::default_font())
        .theme(Damascus::theme)
        .scale_factor(Damascus::scale_factor)
        .run()
}
