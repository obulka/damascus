// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use core::ops::RangeInclusive;

use eframe::egui;

use super::UIData;
use crate::icons::Icons;

pub mod boolean;
pub mod boolean_vec3;
pub mod combo_box;
pub mod float;
pub mod integer;
pub mod mat3;
pub mod mat4;
pub mod material;
pub mod unsigned_integer;
pub mod unsigned_integer_vec3;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub trait UIInput<T> {
    fn new(value: T) -> Self;

    fn create_parameter_label(&self, ui: &mut egui::Ui, label: &str) {
        if let Some(tooltip) = &self.ui_data().tooltip() {
            ui.add(egui::Label::new(label).selectable(false))
                .on_hover_text(tooltip);
            return;
        }
        ui.add(egui::Label::new(label).selectable(false));
    }

    fn create_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        if *self.ui_data().hidden() {
            return false;
        }
        self.show_ui(ui, label)
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        self.create_parameter_label(ui, label);
        false
    }

    fn value(&self) -> &T;

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
    fn create_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        if *self.ui_data().hidden() {
            return false;
        }
        RangedInput::show_ui(self, ui, label)
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut has_changed = false;
        ui.horizontal(|ui| {
            self.create_parameter_label(ui, label);
            has_changed |= ui.add(self.create_slider()).changed();
        });
        has_changed
    }

    fn create_slider(&mut self) -> egui::Slider<'_> {
        let range: RangeInclusive<T> = self.range().clone();
        egui::Slider::new(self.value_mut(), range).clamp_to_range(false)
    }

    fn value_mut(&mut self) -> &mut T;

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

pub trait Collapsible<T>: UIInput<T> {
    fn with_collapsed(self) -> Self;

    fn collapse_button(&mut self, ui: &mut egui::Ui) -> bool {
        let toggle_icon = egui::Image::new(if self.collapsed() {
            Icons::ArrowRight.source()
        } else {
            Icons::ArrowLeft.source()
        })
        .maintain_aspect_ratio(true)
        .fit_to_exact_size(egui::Vec2::splat(ui.available_size().y / 2.));

        if ui
            .add_enabled(self.collapse_enabled(), egui::ImageButton::new(toggle_icon))
            .clicked()
        {
            self.toggle_collapsed();
            return true;
        }
        false
    }

    fn collapse_enabled(&self) -> bool {
        true
    }

    fn collapse(&mut self);

    fn expand(&mut self);

    fn collapsed(&self) -> bool;

    fn toggle_collapsed(&mut self) {
        if self.collapsed() {
            self.expand();
        } else {
            self.collapse();
        }
    }
}

pub trait Connection<T>: UIInput<T> {
    fn connect(&mut self);

    fn disconnect(&mut self);

    fn connected(&self) -> bool;

    fn toggle_connected(&mut self) {
        if self.connected() {
            self.disconnect();
        } else {
            self.connect();
        }
    }
}

pub fn create_drag_value_ui<T: eframe::emath::Numeric>(
    ui: &mut egui::Ui,
    value: &mut T,
) -> egui::Response {
    ui.add(egui::DragValue::new(value))
}
