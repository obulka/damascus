// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{any::type_name, collections::HashMap};

use glam::{BVec3, Mat3, Mat4, UVec2, UVec3, Vec2, Vec3, Vec4};
use macro_rules_attribute::derive;

use crate::{
    Enum, EnumTraits, Enumerator,
    camera::CameraId,
    geometry::primitives::PrimitiveId,
    graph::{
        node_graph::{
            NodeGraph,
            nodes::{NodeErrors, NodeId, NodeResult},
        },
        scene_graph::{RootId, SceneGraphId},
    },
    lights::LightId,
    materials::MaterialId,
    textures::evaluators::TextureEvaluatorId,
};

#[derive(Default, EnumTraits!)]
pub enum InputData {
    #[default]
    None,
    Bool(bool),
    BVec3(BVec3),
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
    Enum(Enum),
    Filepath(String),
    SceneGraphId(SceneGraphId),
}

impl From<bool> for InputData {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<BVec3> for InputData {
    fn from(value: BVec3) -> Self {
        Self::BVec3(value)
    }
}

impl From<i32> for InputData {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<u32> for InputData {
    fn from(value: u32) -> Self {
        Self::UInt(value)
    }
}

impl From<UVec2> for InputData {
    fn from(value: UVec2) -> Self {
        Self::UVec2(value)
    }
}

impl From<UVec3> for InputData {
    fn from(value: UVec3) -> Self {
        Self::UVec3(value)
    }
}

impl From<f32> for InputData {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<Vec2> for InputData {
    fn from(value: Vec2) -> Self {
        Self::Vec2(value)
    }
}

impl From<Vec3> for InputData {
    fn from(value: Vec3) -> Self {
        Self::Vec3(value)
    }
}

impl From<Vec4> for InputData {
    fn from(value: Vec4) -> Self {
        Self::Vec4(value)
    }
}

impl From<Mat3> for InputData {
    fn from(value: Mat3) -> Self {
        Self::Mat3(value)
    }
}

impl From<Mat4> for InputData {
    fn from(value: Mat4) -> Self {
        Self::Mat4(value)
    }
}

impl From<Enum> for InputData {
    fn from(value: Enum) -> Self {
        Self::Enum(value)
    }
}

impl From<SceneGraphId> for InputData {
    fn from(value: SceneGraphId) -> Self {
        Self::SceneGraphId(value)
    }
}

impl InputData {
    pub fn is_evaluable(&self) -> bool {
        match self {
            InputData::SceneGraphId(scene_graph_id) => match scene_graph_id {
                SceneGraphId::TextureEvaluator(..) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn as_bool(&self) -> NodeResult<&bool> {
        match self {
            InputData::Bool(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self.clone(),
                conversion_to: type_name::<bool>().to_string(),
            }),
        }
    }

    pub fn try_to_bool(self) -> NodeResult<bool> {
        match self {
            InputData::Bool(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<bool>().to_string(),
            }),
        }
    }

    pub fn try_to_bvec3(self) -> NodeResult<BVec3> {
        match self {
            InputData::BVec3(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<BVec3>().to_string(),
            }),
        }
    }

    pub fn try_to_int(self) -> NodeResult<i32> {
        match self {
            InputData::Int(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<i32>().to_string(),
            }),
        }
    }

    pub fn try_to_uint(self) -> NodeResult<u32> {
        match self {
            InputData::UInt(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<u32>().to_string(),
            }),
        }
    }

    pub fn try_to_uvec2(self) -> NodeResult<UVec2> {
        match self {
            InputData::UVec2(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<UVec2>().to_string(),
            }),
        }
    }

    pub fn try_to_uvec3(self) -> NodeResult<UVec3> {
        match self {
            InputData::UVec3(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<UVec3>().to_string(),
            }),
        }
    }

    pub fn try_to_enum<E: Enumerator>(self) -> NodeResult<E> {
        match self {
            InputData::Enum(value) => Ok(value.as_enumerator()),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<E>().to_string(),
            }),
        }
    }

    pub fn try_to_filepath(self) -> NodeResult<String> {
        match self {
            InputData::Filepath(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<String>().to_string(),
            }),
        }
    }

    pub fn try_to_float(self) -> NodeResult<f32> {
        match self {
            InputData::Float(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<f32>().to_string(),
            }),
        }
    }

    pub fn try_to_vec2(self) -> NodeResult<Vec2> {
        match self {
            InputData::Vec2(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<Vec2>().to_string(),
            }),
        }
    }

    pub fn try_to_vec3(self) -> NodeResult<Vec3> {
        match self {
            InputData::Vec3(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<Vec3>().to_string(),
            }),
        }
    }

    pub fn try_to_vec4(self) -> NodeResult<Vec4> {
        match self {
            InputData::Vec4(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<Vec4>().to_string(),
            }),
        }
    }

    pub fn try_to_mat3(self) -> NodeResult<Mat3> {
        match self {
            InputData::Mat3(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<Mat3>().to_string(),
            }),
        }
    }

    pub fn try_to_mat4(self) -> NodeResult<Mat4> {
        match self {
            InputData::Mat4(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<Mat4>().to_string(),
            }),
        }
    }

    pub fn as_scene_graph_id(&self) -> NodeResult<&SceneGraphId> {
        match self {
            InputData::SceneGraphId(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self.clone(),
                conversion_to: type_name::<SceneGraphId>().to_string(),
            }),
        }
    }

    pub fn try_to_scene_graph_id(self) -> NodeResult<SceneGraphId> {
        match self {
            InputData::SceneGraphId(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<SceneGraphId>().to_string(),
            }),
        }
    }

    pub fn as_material_id(&self) -> NodeResult<&MaterialId> {
        match self.as_scene_graph_id()? {
            SceneGraphId::Material(value) => Ok(value),
            value => Err(NodeErrors::InputDowncastError {
                data: InputData::SceneGraphId(*value),
                conversion_to: type_name::<MaterialId>().to_string(),
            }),
        }
    }

    pub fn try_to_material_id(self) -> NodeResult<MaterialId> {
        match self.try_to_scene_graph_id()? {
            SceneGraphId::Material(value) => Ok(value),
            value => Err(NodeErrors::InputDowncastError {
                data: InputData::SceneGraphId(value),
                conversion_to: type_name::<MaterialId>().to_string(),
            }),
        }
    }

    pub fn try_to_primitive_id(self) -> NodeResult<PrimitiveId> {
        match self.try_to_scene_graph_id()? {
            SceneGraphId::Primitive(value) => Ok(value),
            value => Err(NodeErrors::InputDowncastError {
                data: InputData::SceneGraphId(value),
                conversion_to: type_name::<PrimitiveId>().to_string(),
            }),
        }
    }

    pub fn try_to_light_id(self) -> NodeResult<LightId> {
        match self.try_to_scene_graph_id()? {
            SceneGraphId::Light(value) => Ok(value),
            value => Err(NodeErrors::InputDowncastError {
                data: InputData::SceneGraphId(value),
                conversion_to: type_name::<LightId>().to_string(),
            }),
        }
    }

    pub fn as_camera_id(&self) -> NodeResult<&CameraId> {
        match self.as_scene_graph_id()? {
            SceneGraphId::Camera(value) => Ok(value),
            value => Err(NodeErrors::InputDowncastError {
                data: InputData::SceneGraphId(*value),
                conversion_to: type_name::<CameraId>().to_string(),
            }),
        }
    }

    pub fn try_to_camera_id(self) -> NodeResult<CameraId> {
        match self.try_to_scene_graph_id()? {
            SceneGraphId::Camera(value) => Ok(value),
            value => Err(NodeErrors::InputDowncastError {
                data: InputData::SceneGraphId(value),
                conversion_to: type_name::<CameraId>().to_string(),
            }),
        }
    }

    pub fn try_to_root_id(self) -> NodeResult<RootId> {
        match self.try_to_scene_graph_id()? {
            SceneGraphId::Root(value) => Ok(value),
            value => Err(NodeErrors::InputDowncastError {
                data: InputData::SceneGraphId(value),
                conversion_to: type_name::<RootId>().to_string(),
            }),
        }
    }

    pub fn as_texture_evaluator_id(&self) -> NodeResult<&TextureEvaluatorId> {
        match self.as_scene_graph_id()? {
            SceneGraphId::TextureEvaluator(value) => Ok(value),
            value => Err(NodeErrors::InputDowncastError {
                data: InputData::SceneGraphId(*value),
                conversion_to: type_name::<TextureEvaluatorId>().to_string(),
            }),
        }
    }

    pub fn try_to_texture_evaluator_id(self) -> NodeResult<TextureEvaluatorId> {
        match self.try_to_scene_graph_id()? {
            SceneGraphId::TextureEvaluator(value) => Ok(value),
            value => Err(NodeErrors::InputDowncastError {
                data: InputData::SceneGraphId(value),
                conversion_to: type_name::<TextureEvaluatorId>().to_string(),
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
        self.variant_label()
    }

    fn add_to_node(graph: &mut NodeGraph, node_id: NodeId) {
        Self::iter().for_each(|input| {
            graph.add_input(node_id, &input.name(), input.default_data());
        });
    }

    fn from_data_map(&self, data_map: &mut HashMap<String, InputData>) -> NodeResult<InputData> {
        let name: String = self.name();
        data_map
            .remove(&name)
            .ok_or_else(|| NodeErrors::InputDataDoesNotExistError(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_errors() {
        assert_eq!(
            InputData::None.try_to_vec3(),
            Err(NodeErrors::InputDowncastError {
                data: InputData::None,
                conversion_to: "glam::f32::vec3::Vec3".to_string(),
            })
        );
        assert_eq!(
            InputData::Vec3(Vec3::ONE).try_to_enum::<InputData>(),
            Err(NodeErrors::InputDowncastError {
                data: InputData::Vec3(Vec3::ONE),
                conversion_to: "damascus::graph::node_graph::inputs::input_data::InputData"
                    .to_string(),
            })
        );
    }
}
