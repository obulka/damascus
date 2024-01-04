use eframe::egui;

use crate::panels::node_graph::value_type::{UIData, UIInput};

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Bool {
    value: bool,
    ui_data: Option<UIData>,
}

impl Bool {
    pub fn new(value: bool, ui_data: Option<UIData>) -> Self {
        Self {
            value: value,
            ui_data: ui_data,
        }
    }
}

impl UIInput<bool> for Bool {
    fn create_ui(&mut self, ui: &mut egui::Ui, label: &str) {
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            ui.add(egui::Checkbox::new(&mut self.value, ""));
        });
    }

    fn get_value(&self) -> &bool {
        &self.value
    }

    fn get_ui_data(&self) -> &Option<UIData> {
        &self.ui_data
    }

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData> {
        &mut self.ui_data
    }
}
