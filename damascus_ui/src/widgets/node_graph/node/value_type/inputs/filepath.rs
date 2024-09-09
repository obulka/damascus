// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui;

use super::{UIData, UIInput};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Filepath {
    path_string: String,
    value: Box<std::path::Path>,
    ui_data: UIData,
}

impl Default for Filepath {
    fn default() -> Self {
        Self {
            path_string: String::new(),
            value: std::path::Path::new("").into(),
            ui_data: UIData::default(),
        }
    }
}

impl UIInput<Box<std::path::Path>> for Filepath {
    fn new(value: Box<std::path::Path>) -> Self {
        if let Some(path_string) = (*value).to_str() {
            return Self {
                path_string: path_string.to_string(),
                value: value,
                ..Default::default()
            };
        }
        Self::default()
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut has_changed = false;
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.path_string).desired_width(f32::INFINITY),
            );
            self.value = std::path::Path::new(&self.path_string).into();
            has_changed =
                response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
        });

        has_changed
    }

    fn value(&self) -> &Box<std::path::Path> {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}