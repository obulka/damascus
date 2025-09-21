// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use glam::{BVec3, Mat3, Mat4, UVec2, UVec3, Vec2, Vec3, Vec4};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    node_graph::{nodes::NodeId, NodeGraph},
    render_passes::RenderPasses,
    scene::Scene,
    Enum, Enumerator,
};

use super::{InputErrors, InputResult};

mod axis;
mod camera;
mod grade;
mod light;
mod material;
mod primitive;
mod ray_marcher;
mod scene;
mod texture;

pub use axis::AxisInputData;
pub use camera::CameraInputData;
pub use grade::GradeInputData;
pub use light::LightInputData;
pub use material::MaterialInputData;
pub use primitive::PrimitiveInputData;
pub use ray_marcher::RayMarcherInputData;
pub use scene::SceneInputData;
pub use texture::TextureInputData;

#[derive(
    Debug,
    Display,
    Default,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    PartialEq,
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

pub trait NodeInputData: Enumerator + Eq {
    fn default_data(&self) -> InputData;

    fn name(&self) -> String {
        self.to_string()
    }

    fn label(&self) -> String {
        let mut words = Vec::<String>::new();
        let mut word = String::new();

        for character in self.to_string().chars() {
            if character.is_uppercase() && !word.is_empty() {
                words.push(word.clone());
                word.clear();
            }
            word.push_str(&character.to_lowercase().to_string());
        }

        if !word.is_empty() {
            words.push(word);
        }

        words.join(" ")
    }

    fn add_to_node(node_graph: &mut NodeGraph, node_id: NodeId) {
        Self::iter().for_each(|input| {
            node_graph.add_input(node_id, &input.name(), input.default_data());
        });
    }

    fn get_data(&self, data_map: &mut HashMap<String, InputData>) -> InputResult<InputData> {
        let name: String = self.name();
        data_map
            .remove(&name)
            .ok_or_else(|| InputErrors::InputDataDoesNotExistError(name))
    }

    fn compute_output(_data_map: &mut HashMap<String, InputData>) -> InputResult<InputData> {
        Err(InputErrors::UnknownError)
    }
}
