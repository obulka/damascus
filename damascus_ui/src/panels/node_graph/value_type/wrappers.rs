use core::ops::RangeInclusive;
use std::fmt::Display;

use glam;
use strum::IntoEnumIterator;

use crate::panels::node_graph::value_type::UIData;

pub trait UIInput<T> {
    fn get_value(&self) -> T;

    fn get_value_mut(&mut self) -> &mut T;

    fn get_ui_data(&self) -> &Option<UIData>;

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData>;
}

pub trait RangedInput<T>: UIInput<T> {
    fn with_range(value: T, ui_data: Option<UIData>, range: RangeInclusive<T>) -> Self;

    fn get_range(&self) -> RangeInclusive<T>;
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Integer {
    pub value: i32,
    pub ui_data: Option<UIData>,
    pub range: RangeInclusive<i32>,
}

impl UIInput<i32> for Integer {
    fn get_value(&self) -> i32 {
        self.value
    }

    fn get_value_mut(&mut self) -> &mut i32 {
        &mut self.value
    }

    fn get_ui_data(&self) -> &Option<UIData> {
        &self.ui_data
    }

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData> {
        &mut self.ui_data
    }
}

impl RangedInput<i32> for Integer {
    fn with_range(value: i32, ui_data: Option<UIData>, range: RangeInclusive<i32>) -> Self {
        Self {
            value: value,
            ui_data: ui_data,
            range: range,
        }
    }

    fn get_range(&self) -> RangeInclusive<i32> {
        self.range.clone()
    }
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct UnsignedInteger {
    pub value: u32,
    pub ui_data: Option<UIData>,
    pub range: RangeInclusive<u32>,
}

impl UIInput<u32> for UnsignedInteger {
    fn get_value(&self) -> u32 {
        self.value
    }

    fn get_value_mut(&mut self) -> &mut u32 {
        &mut self.value
    }

    fn get_ui_data(&self) -> &Option<UIData> {
        &self.ui_data
    }

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData> {
        &mut self.ui_data
    }
}

impl RangedInput<u32> for UnsignedInteger {
    fn with_range(value: u32, ui_data: Option<UIData>, range: RangeInclusive<u32>) -> Self {
        Self {
            value: value,
            range: range,
            ui_data: ui_data,
        }
    }

    fn get_range(&self) -> RangeInclusive<u32> {
        self.range.clone()
    }
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Float {
    pub value: f32,
    pub ui_data: Option<UIData>,
    pub range: RangeInclusive<f32>,
}

impl UIInput<f32> for Float {
    fn get_value(&self) -> f32 {
        self.value
    }

    fn get_value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }

    fn get_ui_data(&self) -> &Option<UIData> {
        &self.ui_data
    }

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData> {
        &mut self.ui_data
    }
}

impl RangedInput<f32> for Float {
    fn with_range(value: f32, ui_data: Option<UIData>, range: RangeInclusive<f32>) -> Self {
        Self {
            value: value,
            range: range,
            ui_data: ui_data,
        }
    }

    fn get_range(&self) -> RangeInclusive<f32> {
        self.range.clone()
    }
}

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Vec3 {
    pub value: [f32; 3],
    pub is_colour: bool,
}

impl Vec3 {
    pub fn new(value: glam::Vec3, is_colour: bool) -> Self {
        return Self {
            value: value.to_array(),
            is_colour: is_colour,
        };
    }

    pub fn as_vec3(&self) -> glam::Vec3 {
        glam::Vec3::from_array(self.value)
    }
}

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Vec4 {
    pub value: [f32; 4],
    pub is_colour: bool,
}

impl Vec4 {
    pub fn new(value: glam::Vec4, is_colour: bool) -> Self {
        return Self {
            value: value.to_array(),
            is_colour: is_colour,
        };
    }

    pub fn as_vec4(&self) -> glam::Vec4 {
        glam::Vec4::from_array(self.value)
    }
}

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ComboBox {
    pub selected: String,
    pub options: Vec<String>,
}

impl ComboBox {
    pub fn new<E: IntoEnumIterator + Display>(enumeration: E) -> Self {
        let mut options = vec![];
        for enum_option in E::iter() {
            options.push(format!("{}", enum_option));
        }
        Self {
            selected: format!("{}", enumeration),
            options: options,
        }
    }
}
