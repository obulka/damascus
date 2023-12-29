use core::ops::RangeInclusive;
use std::fmt::Display;

use glam;
use strum::IntoEnumIterator;

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Integer {
    pub value: i32,
    pub range: RangeInclusive<i32>,
}

impl Integer {
    pub fn new(value: i32, range: RangeInclusive<i32>) -> Self {
        Self {
            value: value,
            range: range,
        }
    }
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct UnsignedInteger {
    pub value: u32,
    pub range: RangeInclusive<u32>,
}

impl UnsignedInteger {
    pub fn new(value: u32, range: RangeInclusive<u32>) -> Self {
        Self {
            value: value,
            range: range,
        }
    }
}

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Float {
    pub value: f32,
    pub range: RangeInclusive<f32>,
}

impl Float {
    pub fn new(value: f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value: value,
            range: range,
        }
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
