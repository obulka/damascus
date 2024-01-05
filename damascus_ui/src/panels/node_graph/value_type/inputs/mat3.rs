use eframe::egui;
use glam;

use crate::panels::node_graph::value_type::{create_drag_value_ui, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Mat3 {
    value: glam::Mat3,
    ui_data: UIData,
}

impl UIInput<glam::Mat3> for Mat3 {
    fn new(value: glam::Mat3) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) {
        ui.vertical(|ui| {
            self.create_parameter_label(ui, label);
            ui.horizontal(|ui| {
                create_drag_value_ui(ui, &mut self.value.x_axis.x);
                create_drag_value_ui(ui, &mut self.value.x_axis.y);
                create_drag_value_ui(ui, &mut self.value.x_axis.z);
            });
            ui.horizontal(|ui| {
                create_drag_value_ui(ui, &mut self.value.y_axis.x);
                create_drag_value_ui(ui, &mut self.value.y_axis.y);
                create_drag_value_ui(ui, &mut self.value.y_axis.z);
            });
            ui.horizontal(|ui| {
                create_drag_value_ui(ui, &mut self.value.z_axis.x);
                create_drag_value_ui(ui, &mut self.value.z_axis.y);
                create_drag_value_ui(ui, &mut self.value.z_axis.z);
            });
        });
    }

    fn value(&self) -> &glam::Mat3 {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
