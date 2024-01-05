use std::fmt::Display;
use std::str::FromStr;

use eframe::egui;
use strum::IntoEnumIterator;

use crate::panels::node_graph::value_type::{UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ComboBox {
    selected: String,
    options: Vec<String>,
    ui_data: UIData,
}

impl ComboBox {
    pub fn from_enum<E: IntoEnumIterator + Display + FromStr>(enumeration: E) -> Self {
        let mut options = vec![];
        for enum_option in E::iter() {
            options.push(format!("{}", enum_option));
        }
        Self {
            selected: format!("{}", enumeration),
            options: options,
            ..Default::default()
        }
    }

    #[inline]
    pub fn with_options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }

    pub fn as_enum<E: IntoEnumIterator + Display + FromStr>(&self) -> anyhow::Result<E> {
        if let Ok(enum_value) = E::from_str(self.value()) {
            Ok(enum_value)
        } else {
            anyhow::bail!(format!("Could not cast {} to enum", self.value()))
        }
    }
}

impl UIInput<String> for ComboBox {
    fn new(selected: String) -> Self {
        Self {
            selected: selected,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) {
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

    fn value(&self) -> &String {
        &self.selected
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
