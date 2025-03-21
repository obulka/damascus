// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui;
use glam;

use super::{create_drag_value_ui, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Mat4 {
    value: glam::Mat4,
    ui_data: UIData,
}

impl UIInput<glam::Mat4> for Mat4 {
    fn new(value: glam::Mat4) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut has_changed = false;
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            ui.vertical(|ui| {
                has_changed |= create_drag_value_ui(ui, &mut self.value.x_axis.x).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.x_axis.y).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.x_axis.z).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.x_axis.w).changed();
            });
            ui.vertical(|ui| {
                has_changed |= create_drag_value_ui(ui, &mut self.value.y_axis.x).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.y_axis.y).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.y_axis.z).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.y_axis.w).changed();
            });
            ui.vertical(|ui| {
                has_changed |= create_drag_value_ui(ui, &mut self.value.z_axis.x).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.z_axis.y).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.z_axis.z).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.z_axis.w).changed();
            });
            ui.vertical(|ui| {
                has_changed |= create_drag_value_ui(ui, &mut self.value.w_axis.x).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.w_axis.y).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.w_axis.z).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.w_axis.w).changed();
            });
        });
        has_changed
    }

    fn show_ui_connected(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
        });
        false
    }

    fn value(&self) -> &glam::Mat4 {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
