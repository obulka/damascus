use eframe::egui;

use crate::panels::node_graph::value_type::{UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Bool {
    value: bool,
    ui_data: UIData,
}

impl UIInput<bool> for Bool {
    fn new(value: bool) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            ui.add(egui::Checkbox::new(&mut self.value, ""));
        });
        false
    }

    fn value(&self) -> &bool {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
