use eframe::egui;
use glam;

use crate::panels::node_graph::value_type::{create_drag_value_ui, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Vec2 {
    value: glam::Vec2,
    ui_data: UIData,
}

impl UIInput<glam::Vec2> for Vec2 {
    fn new(value: glam::Vec2) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut has_changed = false;
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            has_changed |= create_drag_value_ui(ui, &mut self.value.x).changed();
            has_changed |= create_drag_value_ui(ui, &mut self.value.y).changed();
        });
        has_changed
    }

    fn value(&self) -> &glam::Vec2 {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
