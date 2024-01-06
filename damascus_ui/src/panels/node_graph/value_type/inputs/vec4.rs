use eframe::egui;
use glam;

use crate::panels::node_graph::value_type::{
    create_drag_value_ui, inputs::Colour, UIData, UIInput,
};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Vec4 {
    value: [f32; 4],
    ui_data: UIData,
    pub is_colour: bool,
}

impl Vec4 {
    pub fn from_vec4(value: glam::Vec4) -> Self {
        return Self {
            value: value.to_array(),
            ..Default::default()
        };
    }

    pub fn as_vec4(&self) -> glam::Vec4 {
        glam::Vec4::from_array(self.value)
    }
}

impl UIInput<[f32; 4]> for Vec4 {
    fn new(value: [f32; 4]) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            create_drag_value_ui(ui, &mut self.value[0]);
            create_drag_value_ui(ui, &mut self.value[1]);
            create_drag_value_ui(ui, &mut self.value[2]);
            create_drag_value_ui(ui, &mut self.value[3]);
            if self.is_colour {
                ui.color_edit_button_rgba_unmultiplied(&mut self.value);
            }
        });
        false
    }

    fn value(&self) -> &[f32; 4] {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}

impl Colour<[f32; 4]> for Vec4 {
    fn is_colour(&self) -> &bool {
        &self.is_colour
    }

    fn is_colour_mut(&mut self) -> &mut bool {
        &mut self.is_colour
    }
}
