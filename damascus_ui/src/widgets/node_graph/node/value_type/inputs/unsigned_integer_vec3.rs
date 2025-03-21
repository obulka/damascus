// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui;
use glam;

use super::{create_drag_value_ui, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UVec3 {
    value: glam::UVec3,
    ui_data: UIData,
}

impl UIInput<glam::UVec3> for UVec3 {
    fn new(value: glam::UVec3) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut has_changed = false;
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            has_changed |= create_drag_value_ui(ui, &mut self.value.x).changed();
            has_changed |= create_drag_value_ui(ui, &mut self.value.y).changed();
            has_changed |= create_drag_value_ui(ui, &mut self.value.z).changed();
        });
        has_changed
    }

    fn value(&self) -> &glam::UVec3 {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
