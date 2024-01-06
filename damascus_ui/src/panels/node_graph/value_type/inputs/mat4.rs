use eframe::egui;
use glam;

use crate::panels::node_graph::value_type::{create_drag_value_ui, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Mat4 {
    value: glam::Mat4,
    ui_data: UIData,
}

impl UIInput<glam::Mat4> for Mat4 {
    fn new(value: glam::Mat4) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            ui.vertical(|ui| {
                create_drag_value_ui(ui, &mut self.value.x_axis.x);
                create_drag_value_ui(ui, &mut self.value.x_axis.y);
                create_drag_value_ui(ui, &mut self.value.x_axis.z);
                create_drag_value_ui(ui, &mut self.value.x_axis.w);
            });
            ui.vertical(|ui| {
                create_drag_value_ui(ui, &mut self.value.y_axis.x);
                create_drag_value_ui(ui, &mut self.value.y_axis.y);
                create_drag_value_ui(ui, &mut self.value.y_axis.z);
                create_drag_value_ui(ui, &mut self.value.y_axis.w);
            });
            ui.vertical(|ui| {
                create_drag_value_ui(ui, &mut self.value.z_axis.x);
                create_drag_value_ui(ui, &mut self.value.z_axis.y);
                create_drag_value_ui(ui, &mut self.value.z_axis.z);
                create_drag_value_ui(ui, &mut self.value.z_axis.w);
            });
            ui.vertical(|ui| {
                create_drag_value_ui(ui, &mut self.value.w_axis.x);
                create_drag_value_ui(ui, &mut self.value.w_axis.y);
                create_drag_value_ui(ui, &mut self.value.w_axis.z);
                create_drag_value_ui(ui, &mut self.value.w_axis.w);
            });
        });
        false
    }

    fn value(&self) -> &glam::Mat4 {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
