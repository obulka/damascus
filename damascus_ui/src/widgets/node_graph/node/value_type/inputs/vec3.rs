// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui;
use glam;

use super::{create_drag_value_ui, Collapsible, Colour, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Vec3 {
    value: [f32; 3],
    ui_data: UIData,
    collapsed: bool,
    pub is_colour: bool,
}

impl Vec3 {
    pub fn from_vec3(value: glam::Vec3) -> Self {
        return Self {
            value: value.to_array(),
            ..Default::default()
        };
    }

    pub fn as_vec3(&self) -> glam::Vec3 {
        glam::Vec3::from_array(self.value)
    }
}

impl UIInput<[f32; 3]> for Vec3 {
    fn new(value: [f32; 3]) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut has_changed = false;
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            has_changed |= create_drag_value_ui(ui, &mut self.value[0]).changed();
            if self.collapsed() {
                self.value[1] = self.value[0];
                self.value[2] = self.value[0];
            } else {
                has_changed |= create_drag_value_ui(ui, &mut self.value[1]).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value[2]).changed();
            }
            if self.is_colour && !self.collapsed() {
                has_changed |= ui.color_edit_button_rgb(&mut self.value).changed();
            }
            has_changed |= self.collapse_button(ui);
        });
        has_changed
    }

    fn value(&self) -> &[f32; 3] {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}

impl Colour<[f32; 3]> for Vec3 {
    fn is_colour(&self) -> &bool {
        &self.is_colour
    }

    fn is_colour_mut(&mut self) -> &mut bool {
        &mut self.is_colour
    }
}

impl Collapsible<[f32; 3]> for Vec3 {
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
