use eframe::egui;
use glam;

use crate::panels::node_graph::value_type::{create_drag_value_ui, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Vec3 {
    value: [f32; 3],
    ui_data: UIData,
    pub is_colour: bool,
}

impl Vec3 {
    pub fn new(value: glam::Vec3, ui_data: UIData, is_colour: bool) -> Self {
        return Self {
            value: value.to_array(),
            ui_data: ui_data, // TODO builder
            is_colour: is_colour,
        };
    }

    pub fn as_vec3(&self) -> glam::Vec3 {
        glam::Vec3::from_array(self.value)
    }
}

impl UIInput<[f32; 3]> for Vec3 {
    fn new(value: [f32; 3]) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) {
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            create_drag_value_ui(ui, &mut self.value[0]);
            create_drag_value_ui(ui, &mut self.value[1]);
            create_drag_value_ui(ui, &mut self.value[2]);
            if self.is_colour {
                ui.color_edit_button_rgb(&mut self.value);
            }
        });
    }

    fn value(&self) -> &[f32; 3] {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
