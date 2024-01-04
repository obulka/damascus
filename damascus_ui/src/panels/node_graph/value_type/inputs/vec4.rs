use eframe::egui;
use glam;

use crate::panels::node_graph::value_type::{UIData, UIInput};

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Vec4 {
    pub value: [f32; 4],
    pub ui_data: Option<UIData>,
    pub is_colour: bool,
}

impl Vec4 {
    fn create_drag_value_ui(ui: &mut egui::Ui, value: &mut f32) {
        ui.add(egui::DragValue::new(value).max_decimals(100));
    }

    pub fn new(value: glam::Vec4, ui_data: Option<UIData>, is_colour: bool) -> Self {
        return Self {
            value: value.to_array(),
            ui_data: ui_data,
            is_colour: is_colour,
        };
    }

    pub fn as_vec4(&self) -> glam::Vec4 {
        glam::Vec4::from_array(self.value)
    }
}

impl UIInput<[f32; 4]> for Vec4 {
    fn create_ui(&mut self, ui: &mut egui::Ui, label: &str) {
        ui.horizontal(|ui| {
            ui.label(label);
            Self::create_drag_value_ui(ui, &mut self.value[0]);
            Self::create_drag_value_ui(ui, &mut self.value[1]);
            Self::create_drag_value_ui(ui, &mut self.value[2]);
            Self::create_drag_value_ui(ui, &mut self.value[3]);
            if self.is_colour {
                ui.color_edit_button_rgba_unmultiplied(&mut self.value);
            }
        });
    }

    fn get_value(&self) -> [f32; 4] {
        self.value
    }

    fn get_value_mut(&mut self) -> &mut [f32; 4] {
        &mut self.value
    }

    fn get_ui_data(&self) -> &Option<UIData> {
        &self.ui_data
    }

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData> {
        &mut self.ui_data
    }
}
