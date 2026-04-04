// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui;

use super::{UIData, UIInput};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Filepath {
    value: String,
    show_image: bool,
    ui_data: UIData,
}

impl Default for Filepath {
    fn default() -> Self {
        Self {
            value: String::new(),
            show_image: true,
            ui_data: UIData::default(),
        }
    }
}

impl Filepath {
    fn file_button(&mut self, ui: &mut egui::Ui) -> bool {
        if ui.add(egui::Button::new("🗁")).clicked() {
            let mut file_dialog = rfd::FileDialog::new()
                .set_title("load from file")
                .set_file_name(&self.value);
            if let Some(directory) = std::path::Path::new(&self.value).parent() {
                file_dialog = file_dialog.set_directory(directory);
            }

            if let Some(path) = file_dialog.pick_file() {
                self.value = path.display().to_string();
            }
            return true;
        }
        false
    }
}

impl UIInput<String> for Filepath {
    fn new(value: String) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut has_changed = false;
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            ui.style_mut().visuals.extreme_bg_color =
                if self.value.is_empty() || std::path::Path::new(&self.value).is_file() {
                    egui::Color32::from_gray(45)
                } else {
                    egui::Color32::from_rgb(45, 0, 0)
                };
            let response = ui
                .add(
                    egui::TextEdit::singleline(&mut self.value)
                        .desired_width(f32::INFINITY)
                        .hint_text("/path/to/file"),
                )
                .on_hover_text(self.value.clone());
            has_changed =
                response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));

            self.file_button(ui);
        });
        ui.horizontal(|ui| {
            ui.label("🖼");
            ui.add(egui::Checkbox::without_text(&mut self.show_image));
        });
        if self.show_image {
            ui.vertical_centered(|ui| {
                if !self.value.is_empty() && std::path::Path::new(&self.value).is_file() {
                    ui.add(
                        egui::Image::new(format!("file://{}", self.value))
                            .shrink_to_fit()
                            .maintain_aspect_ratio(true)
                            .corner_radius(10.)
                            .show_loading_spinner(true),
                    );
                }
            });
        }

        has_changed
    }

    fn value(&self) -> &String {
        &self.value
    }

    fn deref(self) -> String {
        self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
