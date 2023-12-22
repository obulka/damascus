use std::fmt::Display;

use glam;
use strum::IntoEnumIterator;

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
