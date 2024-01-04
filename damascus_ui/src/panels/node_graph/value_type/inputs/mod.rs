use core::ops::RangeInclusive;

use eframe::egui;

use crate::panels::node_graph::value_type::UIData;

pub mod boolean;
pub mod combo_box;
pub mod float;
pub mod integer;
pub mod unsigned_integer;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub trait UIInput<T> {
    fn create_parameter_label(&self, ui: &mut egui::Ui, label: &str) {
        if let Some(ui_data) = self.get_ui_data() {
            if let Some(tooltip) = &ui_data.tooltip {
                ui.label(label).on_hover_text(tooltip);
                return;
            }
        }
        ui.label(label);
    }

    fn create_ui(&mut self, ui: &mut egui::Ui, label: &str) {
        self.create_parameter_label(ui, label);
    }

    fn get_value(&self) -> &T;

    fn get_ui_data(&self) -> &Option<UIData>;

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData>;
}

pub trait RangedInput<T: eframe::emath::Numeric>: UIInput<T> {
    fn create_ui(&mut self, ui: &mut egui::Ui, label: &str) {
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            ui.add(self.create_slider());
        });
    }

    fn create_slider(&mut self) -> egui::Slider<'_> {
        let range: RangeInclusive<T> = self.get_range();
        egui::Slider::new(self.get_value_mut(), range).clamp_to_range(false)
    }

    fn get_value_mut(&mut self) -> &mut T;

    fn with_range(value: T, ui_data: Option<UIData>, range: RangeInclusive<T>) -> Self;

    fn get_range(&self) -> RangeInclusive<T>;
}

pub fn create_drag_value_ui(ui: &mut egui::Ui, value: &mut f32) {
    ui.add(egui::DragValue::new(value).max_decimals(100));
}
