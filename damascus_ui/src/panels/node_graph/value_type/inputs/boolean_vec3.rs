// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use eframe::egui;
use glam;

use super::{UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BVec3 {
    value: glam::BVec3,
    ui_data: UIData,
}

impl UIInput<glam::BVec3> for BVec3 {
    fn new(value: glam::BVec3) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut has_changed = false;
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            has_changed |= ui.add(egui::Checkbox::new(&mut self.value.x, "")).changed();
            has_changed |= ui.add(egui::Checkbox::new(&mut self.value.y, "")).changed();
            has_changed |= ui.add(egui::Checkbox::new(&mut self.value.z, "")).changed();
        });
        has_changed
    }

    fn value(&self) -> &glam::BVec3 {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
