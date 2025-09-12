// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use core::ops::RangeInclusive;

use eframe::egui;
use glam;

use super::{create_drag_value_ui, Collapsible, Colour, RangedInput, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Vec4 {
    value: glam::Vec4,
    ui_data: UIData,
    collapsed: bool,
    pub is_colour: bool,
    pub range: RangeInclusive<f32>,
}

impl Default for Vec4 {
    fn default() -> Self {
        Self {
            value: glam::Vec4::ZERO,
            ui_data: UIData::default(),
            collapsed: false,
            is_colour: false,
            range: 0.0..=1.,
        }
    }
}

impl UIInput<glam::Vec4> for Vec4 {
    fn new(value: glam::Vec4) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &glam::Vec4 {
        &self.value
    }

    fn deref(self) -> glam::Vec4 {
        self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}

impl Colour<glam::Vec4> for Vec4 {
    fn is_colour(&self) -> &bool {
        &self.is_colour
    }

    fn is_colour_mut(&mut self) -> &mut bool {
        &mut self.is_colour
    }
}

impl Collapsible<glam::Vec4> for Vec4 {
    fn with_collapsed(mut self) -> Self {
        self.collapsed = true;
        self
    }

    fn collapse(&mut self) {
        self.collapsed = true;
    }

    fn expand(&mut self) {
        self.collapsed = false;
    }

    fn collapsed(&self) -> bool {
        self.collapsed
    }
}

impl RangedInput<glam::Vec4, f32> for Vec4 {
    fn value_mut(&mut self) -> &mut f32 {
        &mut self.value.x
    }

    fn range_mut(&mut self) -> &mut RangeInclusive<f32> {
        &mut self.range
    }

    fn range(&self) -> &RangeInclusive<f32> {
        &self.range
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut has_changed = false;
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            if self.collapsed() {
                has_changed |= ui.add(self.create_slider()).changed();
                self.value.y = self.value.x;
                self.value.z = self.value.x;
                self.value.w = self.value.x;
            } else {
                has_changed |= create_drag_value_ui(ui, &mut self.value.x).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.y).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.z).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.w).changed();
            }
            if self.is_colour && !self.collapsed() {
                has_changed |= ui
                    .color_edit_button_rgba_unmultiplied(self.value.as_mut())
                    .changed();
            }
            has_changed |= self.collapse_button(ui);
        });
        has_changed
    }
}
