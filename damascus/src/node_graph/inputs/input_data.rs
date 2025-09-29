// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use glam::{BVec3, Mat3, Mat4, UVec2, UVec3, Vec2, Vec3, Vec4};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    node_graph::{
        nodes::{NodeErrors, NodeId, NodeResult},
        NodeGraph,
    },
    render_passes::RenderPasses,
    scene::Scene,
    Enum, Enumerator,
};

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
    SceneGraph(Scene),
}

impl Enumerator for InputData {}

impl InputData {
    pub fn try_to_bool(self) -> NodeResult<bool> {
        match self {
            InputData::Bool(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "bool".to_string(),
            }),
        }
    }

    pub fn try_to_bvec3(self) -> NodeResult<BVec3> {
        match self {
            InputData::BVec3(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "BVec3".to_string(),
            }),
        }
    }

    pub fn try_to_int(self) -> NodeResult<i32> {
        match self {
            InputData::Int(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "i32".to_string(),
            }),
        }
    }

    pub fn try_to_uint(self) -> NodeResult<u32> {
        match self {
            InputData::UInt(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "u32".to_string(),
            }),
        }
    }

    pub fn try_to_uvec2(self) -> NodeResult<UVec2> {
        match self {
            InputData::UVec2(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "UVec2".to_string(),
            }),
        }
    }

    pub fn try_to_uvec3(self) -> NodeResult<UVec3> {
        match self {
            InputData::UVec3(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "UVec3".to_string(),
            }),
        }
    }

    pub fn try_to_enum<E: Enumerator>(self) -> NodeResult<E> {
        match self {
            InputData::Enum(value) => Ok(value.as_enumerator()),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "Enum".to_string(),
            }),
        }
    }

    pub fn try_to_filepath(self) -> NodeResult<String> {
        match self {
            InputData::Filepath(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "String".to_string(),
            }),
        }
    }

    pub fn try_to_float(self) -> NodeResult<f32> {
        match self {
            InputData::Float(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "f32".to_string(),
            }),
        }
    }

    pub fn try_to_vec2(self) -> NodeResult<Vec2> {
        match self {
            InputData::Vec2(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "Vec2".to_string(),
            }),
        }
    }

    pub fn try_to_vec3(self) -> NodeResult<Vec3> {
        match self {
            InputData::Vec3(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "Vec3".to_string(),
            }),
        }
    }

    pub fn try_to_vec4(self) -> NodeResult<Vec4> {
        match self {
            InputData::Vec4(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "Vec4".to_string(),
            }),
        }
    }

    pub fn try_to_mat3(self) -> NodeResult<Mat3> {
        match self {
            InputData::Mat3(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "Mat3".to_string(),
            }),
        }
    }

    pub fn try_to_mat4(self) -> NodeResult<Mat4> {
        match self {
            InputData::Mat4(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "Mat4".to_string(),
            }),
        }
    }

    pub fn try_to_render_pass(self) -> NodeResult<RenderPasses> {
        match self {
            InputData::RenderPass(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: "RenderPasses".to_string(),
            }),
        }
    }

    pub fn try_to_scene(self) -> NodeResult<Scene> {
        match self {
            InputData::SceneGraph(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
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
        self.variant_snake_case()
    }

    fn add_to_node(node_graph: &mut NodeGraph, node_id: NodeId) {
        Self::iter().for_each(|input| {
            node_graph.add_input(node_id, &input.name(), input.default_data());
        });
    }

    fn get_data(&self, data_map: &mut HashMap<String, InputData>) -> NodeResult<InputData> {
        let name: String = self.name();
        data_map
            .remove(&name)
            .ok_or_else(|| NodeErrors::InputDataDoesNotExistError(name))
    }

    fn compute_output(_data_map: &mut HashMap<String, InputData>) -> NodeResult<InputData> {
        Err(NodeErrors::UnknownError)
    }
}
