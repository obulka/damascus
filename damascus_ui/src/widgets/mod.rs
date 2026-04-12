// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt::Debug;

use iced;

use crate::app::Context;

pub mod dialog;
pub mod menu;
pub mod node_graph;
pub mod toolbar;
// pub mod viewport;
pub mod panel;
pub mod style;

pub trait Widget<WidgetMessage: Clone>:
    Clone + Debug + Default + for<'a> serde::Deserialize<'a> + serde::Serialize
{
    fn new() -> Self {
        Self::default()
    }

    fn update(
        &mut self,
        _context: &mut Context,
        _message: WidgetMessage,
    ) -> iced::Task<WidgetMessage> {
        iced::Task::none()
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        _preferences: &'a style::Preferences,
    ) -> iced::Element<'a, WidgetMessage> {
        None::<iced::Element<'a, WidgetMessage>>.into()
    }
}
