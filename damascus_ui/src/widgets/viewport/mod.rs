// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crate::widgets::Style;

use super::Widget;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ViewportMessage {
    None,
}

#[derive(Clone, Copy, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Viewport {}

impl Widget<ViewportMessage> for Viewport {
    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, ViewportMessage> {
        style.text("Viewport").into()
    }
}
