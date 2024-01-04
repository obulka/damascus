use eframe::egui;
use glam;

use crate::panels::node_graph::value_type::{create_drag_value_ui, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Mat4 {
    value: glam::Mat4,
    ui_data: Option<UIData>,
}

impl Mat4 {
    pub fn new(value: glam::Mat4, ui_data: Option<UIData>) -> Self {
        return Self {
            value: value,
            ui_data: ui_data,
        };
    }
}

impl UIInput<glam::Mat4> for Mat4 {
    fn create_ui(&mut self, ui: &mut egui::Ui, label: &str) {
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
    }

    fn get_value(&self) -> &glam::Mat4 {
        &self.value
    }

    fn get_ui_data(&self) -> &Option<UIData> {
        &self.ui_data
    }

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData> {
        &mut self.ui_data
    }
}
