// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui;

use damascus::{Enum, Enumerator};

use super::{UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ComboBox {
    value: Enum,
    ui_data: UIData,
}

impl ComboBox {
    pub fn from<E: Enumerator>(enumerator: E) -> Self {
        Self {
            value: Enum::from(enumerator),
            ..Default::default()
        }
    }
}

impl UIInput<Enum> for ComboBox {
    fn new(value: Enum) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut has_changed = false;
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            egui::ComboBox::from_id_salt(label)
                .selected_text(&self.value.variant)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for enum_option in self.value.variants.iter() {
                        has_changed |= ui
                            .selectable_value(
                                &mut self.value.variant,
                                enum_option.to_string(),
                                enum_option,
                            )
                            .changed();
                    }
                });
        });
        has_changed
    }

    fn value(&self) -> &Enum {
        &self.value
    }

    fn deref(self) -> Enum {
        self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
