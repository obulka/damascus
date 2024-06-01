// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use eframe::egui;
use glam;

use super::{create_drag_value_ui, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Mat3 {
    value: glam::Mat3,
    ui_data: UIData,
}

impl UIInput<glam::Mat3> for Mat3 {
    fn new(value: glam::Mat3) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut has_changed = false;
        ui.vertical(|ui| {
            self.create_parameter_label(ui, label);
            ui.horizontal(|ui| {
                has_changed |= create_drag_value_ui(ui, &mut self.value.x_axis.x).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.x_axis.y).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.x_axis.z).changed();
            });
            ui.horizontal(|ui| {
                has_changed |= create_drag_value_ui(ui, &mut self.value.y_axis.x).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.y_axis.y).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.y_axis.z).changed();
            });
            ui.horizontal(|ui| {
                has_changed |= create_drag_value_ui(ui, &mut self.value.z_axis.x).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.z_axis.y).changed();
                has_changed |= create_drag_value_ui(ui, &mut self.value.z_axis.z).changed();
            });
        });
        has_changed
    }

    fn value(&self) -> &glam::Mat3 {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
