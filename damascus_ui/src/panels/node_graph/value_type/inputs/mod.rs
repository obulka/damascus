use core::ops::RangeInclusive;

use eframe::egui;

use crate::panels::node_graph::value_type::UIData;

pub mod boolean;
pub mod combo_box;
pub mod float;
pub mod integer;
pub mod mat3;
pub mod mat4;
pub mod unsigned_integer;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub trait UIInput<T> {
    fn new(value: T) -> Self;

    fn create_parameter_label(&self, ui: &mut egui::Ui, label: &str) {
        if let Some(tooltip) = &self.ui_data().tooltip() {
            ui.label(label).on_hover_text(tooltip);
            return;
        }
        ui.label(label);
    }

    #[inline]
    fn create_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        if *self.ui_data().hidden() {
            return false;
        }
        self.show_ui(ui, label)
    }

    #[inline]
    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        self.create_parameter_label(ui, label);
        false
    }

    fn value(&self) -> &T;

    #[inline]
    fn with_ui_data(mut self, ui_data: UIData) -> Self
    where
        Self: Sized,
    {
        *self.ui_data_mut() = ui_data;
        self
    }

    fn ui_data(&self) -> &UIData;

    fn ui_data_mut(&mut self) -> &mut UIData;
}

pub trait RangedInput<T: eframe::emath::Numeric>: UIInput<T> {
    #[inline]
    fn create_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        if *self.ui_data().hidden() {
            return false;
        }
        RangedInput::show_ui(self, ui, label)
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            ui.add(self.create_slider());
        });
        false // TODO
    }

    #[inline]
    fn create_slider(&mut self) -> egui::Slider<'_> {
        let range: RangeInclusive<T> = self.range().clone();
        egui::Slider::new(self.value_mut(), range).clamp_to_range(false)
    }

    fn value_mut(&mut self) -> &mut T;

    #[inline]
    fn with_range(mut self, range: RangeInclusive<T>) -> Self
    where
        Self: Sized,
    {
        *self.range_mut() = range;
        self
    }

    fn range(&self) -> &RangeInclusive<T>;

    fn range_mut(&mut self) -> &mut RangeInclusive<T>;
}

pub trait Colour<T>: UIInput<T> {
    #[inline]
    fn as_colour(mut self) -> Self
    where
        Self: Sized,
    {
        *self.is_colour_mut() = true;
        self
    }

    fn is_colour(&self) -> &bool;

    fn is_colour_mut(&mut self) -> &mut bool;
}

pub fn create_drag_value_ui(ui: &mut egui::Ui, value: &mut f32) {
    ui.add(
        egui::DragValue::new(value).max_decimals(100),
        // .update_while_editing(false), // TODO was this added in a later version?
    );
}
