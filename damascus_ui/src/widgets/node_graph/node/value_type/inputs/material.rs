// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.
use eframe::egui;

use damascus_core::materials;

use super::{Connection, UIData, UIInput};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Material {
    value: materials::Material,
    ui_data: UIData,
    connected: bool,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            value: materials::Material::default(),
            ui_data: UIData::default(),
            connected: false,
        }
    }
}

impl UIInput<materials::Material> for Material {
    fn new(value: materials::Material) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &materials::Material {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
        });
        false
    }
}

impl Connection<materials::Material> for Material {
    fn connect(&mut self) {
        self.connected = true;
    }

    fn disconnect(&mut self) {
        self.connected = false;
    }

    fn connected(&self) -> bool {
        self.connected
    }
}
