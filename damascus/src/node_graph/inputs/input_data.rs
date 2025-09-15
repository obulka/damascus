// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::{BVec3, Mat3, Mat4, UVec2, UVec3, Vec2, Vec3, Vec4};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{render_passes::RenderPasses, scene::Scene, Enum, Enumerator};

use super::{InputErrors, InputResult};

#[derive(
    Debug,
    Display,
    Default,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum InputData {
    #[default]
    None,
    Bool(bool),
    BVec3(BVec3),
    Enum(Enum),
    Filepath(String),
    Int(i32),
    UInt(u32),
    UVec2(UVec2),
    UVec3(UVec3),
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Mat3(Mat3),
    Mat4(Mat4),
    RenderPass(RenderPasses),
    Scene(Scene),
}

impl Enumerator for InputData {}

impl InputData {
    pub fn try_to_bool(self) -> InputResult<bool> {
        match self {
            InputData::Bool(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "bool".to_string(),
            }),
        }
    }

    pub fn try_to_bvec3(self) -> InputResult<BVec3> {
        match self {
            InputData::BVec3(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "BVec3".to_string(),
            }),
        }
    }

    pub fn try_to_int(self) -> InputResult<i32> {
        match self {
            InputData::Int(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "i32".to_string(),
            }),
        }
    }

    pub fn try_to_uint(self) -> InputResult<u32> {
        match self {
            InputData::UInt(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "u32".to_string(),
            }),
        }
    }

    pub fn try_to_uvec2(self) -> InputResult<UVec2> {
        match self {
            InputData::UVec2(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "UVec2".to_string(),
            }),
        }
    }

    pub fn try_to_uvec3(self) -> InputResult<UVec3> {
        match self {
            InputData::UVec3(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "UVec3".to_string(),
            }),
        }
    }

    pub fn try_to_enum<E: Enumerator>(self) -> InputResult<E> {
        match self {
            InputData::Enum(value) => Ok(value.as_enumerator()),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "Enum".to_string(),
            }),
        }
    }

    pub fn try_to_filepath(self) -> InputResult<String> {
        match self {
            InputData::Filepath(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "String".to_string(),
            }),
        }
    }

    pub fn try_to_float(self) -> InputResult<f32> {
        match self {
            InputData::Float(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "f32".to_string(),
            }),
        }
    }

    pub fn try_to_vec2(self) -> InputResult<Vec2> {
        match self {
            InputData::Vec2(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "Vec2".to_string(),
            }),
        }
    }

    pub fn try_to_vec3(self) -> InputResult<Vec3> {
        match self {
            InputData::Vec3(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "Vec3".to_string(),
            }),
        }
    }

    pub fn try_to_vec4(self) -> InputResult<Vec4> {
        match self {
            InputData::Vec4(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "Vec4".to_string(),
            }),
        }
    }

    pub fn try_to_mat3(self) -> InputResult<Mat3> {
        match self {
            InputData::Mat3(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "Mat3".to_string(),
            }),
        }
    }

    pub fn try_to_mat4(self) -> InputResult<Mat4> {
        match self {
            InputData::Mat4(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "Mat4".to_string(),
            }),
        }
    }

    pub fn try_to_render_pass(self) -> InputResult<RenderPasses> {
        match self {
            InputData::RenderPass(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "RenderPasses".to_string(),
            }),
        }
    }

    pub fn try_to_scene(self) -> InputResult<Scene> {
        match self {
            InputData::Scene(value) => Ok(value),
            _ => Err(InputErrors::InputDowncastError {
                data: self,
                conversion_to: "Scene".to_string(),
            }),
        }
    }
}
