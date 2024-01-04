use eframe::egui;
use glam;

use crate::panels::node_graph::value_type::{create_drag_value_ui, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Vec2 {
    value: glam::Vec2,
    ui_data: Option<UIData>,
}

impl Vec2 {
    pub fn new(value: glam::Vec2, ui_data: Option<UIData>) -> Self {
        return Self {
            value: value,
            ui_data: ui_data,
        };
    }
}

impl UIInput<glam::Vec2> for Vec2 {
    fn create_ui(&mut self, ui: &mut egui::Ui, label: &str) {
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            create_drag_value_ui(ui, &mut self.value.x);
            create_drag_value_ui(ui, &mut self.value.y);
        });
    }

    fn get_value(&self) -> &glam::Vec2 {
        &self.value
    }

    fn get_ui_data(&self) -> &Option<UIData> {
        &self.ui_data
    }

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData> {
        &mut self.ui_data
    }
}
