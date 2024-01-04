use std::fmt::Display;
use std::str::FromStr;

use eframe::egui;
use strum::IntoEnumIterator;

use crate::panels::node_graph::value_type::{UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ComboBox {
    selected: String,
    options: Vec<String>,
    ui_data: Option<UIData>,
}

impl ComboBox {
    pub fn new<E: IntoEnumIterator + Display + FromStr>(
        enumeration: E,
        ui_data: Option<UIData>,
    ) -> Self {
        let mut options = vec![];
        for enum_option in E::iter() {
            options.push(format!("{}", enum_option));
        }
        Self {
            selected: format!("{}", enumeration),
            options: options,
            ui_data: ui_data,
        }
    }

    pub fn as_enum<E: IntoEnumIterator + Display + FromStr>(&self) -> anyhow::Result<E> {
        if let Ok(enum_value) = E::from_str(self.get_value()) {
            Ok(enum_value)
        } else {
            anyhow::bail!(format!("Could not cast {} to enum", self.get_value()))
        }
    }
}

impl UIInput<String> for ComboBox {
    fn create_ui(&mut self, ui: &mut egui::Ui, label: &str) {
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            egui::ComboBox::from_label("")
                .selected_text(&self.selected)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for enum_option in self.options.iter() {
                        ui.selectable_value(
                            &mut self.selected,
                            enum_option.to_string(),
                            enum_option,
                        );
                    }
                })
        });
    }

    fn get_value(&self) -> &String {
        &self.selected
    }

    fn get_ui_data(&self) -> &Option<UIData> {
        &self.ui_data
    }

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData> {
        &mut self.ui_data
    }
}
