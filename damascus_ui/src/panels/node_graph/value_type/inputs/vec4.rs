use eframe::egui;
use glam;

use super::{create_drag_value_ui, Colour, UIData, UIInput};

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
        let mut has_changed = false;
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            has_changed |= create_drag_value_ui(ui, &mut self.value[0]).changed();
            has_changed |= create_drag_value_ui(ui, &mut self.value[1]).changed();
            has_changed |= create_drag_value_ui(ui, &mut self.value[2]).changed();
            has_changed |= create_drag_value_ui(ui, &mut self.value[3]).changed();
            if self.is_colour {
                has_changed |= ui
                    .color_edit_button_rgba_unmultiplied(&mut self.value)
                    .changed();
            }
        });
        has_changed
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
