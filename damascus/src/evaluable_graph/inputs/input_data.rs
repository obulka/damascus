// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{any::type_name, collections::HashMap};

use glam::{BVec3, Mat3, Mat4, UVec2, UVec3, Vec2, Vec3, Vec4};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    evaluable_graph::{
        nodes::{NodeErrors, NodeId, NodeResult},
        EvaluableGraph,
    },
    render_passes::RenderPasses,
    scene_graph::SceneGraphLocation,
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
    RenderPass(RenderPasses),
    SceneGraphLocation(SceneGraphLocation),
}

impl Enumerator for InputData {}

impl InputData {
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

    pub fn try_to_render_pass(self) -> NodeResult<RenderPasses> {
        match self {
            InputData::RenderPass(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<RenderPasses>().to_string(),
            }),
        }
    }

    pub fn try_to_scene_graph_location(self) -> NodeResult<SceneGraphLocation> {
        match self {
            InputData::SceneGraphLocation(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<SceneGraphLocation>().to_string(),
            }),
        }
    }

    pub fn try_to_material_id(self) -> NodeResult<MaterialId> {
        match self.try_to_scene_graph_location()? {
            SceneGraphLocation::MaterialId(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<MaterialId>().to_string(),
            }),
        }
    }

    pub fn try_to_primitive_id(self) -> NodeResult<PrimitiveId> {
        match self.try_to_scene_graph_location()? {
            SceneGraphLocation::PrimitiveId(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<PrimitiveId>().to_string(),
            }),
        }
    }

    pub fn try_to_light_id(self) -> NodeResult<LightId> {
        match self.try_to_scene_graph_location()? {
            SceneGraphLocation::LightId(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<LightId>().to_string(),
            }),
        }
    }

    pub fn try_to_camera_id(self) -> NodeResult<CameraId> {
        match self.try_to_scene_graph_location()? {
            SceneGraphLocation::CameraId(value) => Ok(value),
            _ => Err(NodeErrors::InputDowncastError {
                data: self,
                conversion_to: type_name::<CameraId>().to_string(),
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

    fn add_to_node(graph: &mut EvaluableGraph, node_id: NodeId) {
        Self::iter().for_each(|input| {
            graph.add_input(node_id, &input.name(), input.default_data());
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
                conversion_to: "damascus::node_graph::inputs::input_data::InputData".to_string(),
            })
        );
    }
}
