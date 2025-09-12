// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

slotmap::new_key_type! { pub struct InputId; }

use crate::{
    camera, geometry::primitive, lights, materials, render_passes, scene, Enum, Enumerator,
};

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Input {
    pub id: InputId,
    pub node: NodeId,
    pub data: InputData,
}

pub type Inputs = SlotMap<InputId, Input>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum InputData {
    Bool(bool),
    BVec3(glam::BVec3),
    Enum(Enum),
    Filepath(String),
    Integer(i32),
    UnsignedInteger(u32),
    UVec2(glam::UVec2),
    UVec3(glam::UVec3),
    Float(f32),
    Vec2(glam::Vec2),
    Vec3(glam::Vec3),
    Vec4(glam::Vec4),
    Mat3(glam::Mat3),
    Mat4(glam::Mat4),
    Camera(Camera),
    Light(Lights),
    Material(Material),
    Primitive(Primitives),
    ProceduralTexture(ProceduralTexture),
    RenderPass(RenderPasses),
    Scene(Scene),
}

impl InputData {
    pub fn to_bool(self) -> Option<bool> {
        match self {
            InputData::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_bvec3(self) -> Option<glam::BVec3> {
        match self {
            InputData::BVec3(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_int(self) -> Option<i32> {
        match self {
            InputData::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_uint(self) -> Option<u32> {
        match self {
            InputData::UnsignedInteger(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_uvec2(self) -> Option<glam::UVec2> {
        match self {
            InputData::UVec2(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_uvec3(self) -> Option<glam::UVec3> {
        match self {
            InputData::UVec3(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_enum<E: Enumerator>(self) -> Option<E> {
        match self {
            InputData::Enum(value) => Some(value.value().as_enumerator()),
            _ => None,
        }
    }

    pub fn to_filepath(self) -> Option<String> {
        match self {
            InputData::Filepath(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_float(self) -> Option<f32> {
        match self {
            InputData::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_vec2(self) -> Option<glam::Vec2> {
        match self {
            InputData::Vec2(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_vec3(self) -> Option<glam::Vec3> {
        match self {
            InputData::Vec3(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_vec4(self) -> Option<glam::Vec4> {
        match self {
            InputData::Vec4(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_mat3(self) -> Option<glam::Mat3> {
        match self {
            InputData::Mat3(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_mat4(self) -> Option<glam::Mat4> {
        match self {
            InputData::Mat4(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_camera(self) -> Option<camera::Camera> {
        match self {
            InputData::Camera(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_light(self) -> Option<Vec<lights::Light>> {
        match self {
            InputData::Light(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_material(self) -> Option<materials::Material> {
        match self {
            InputData::Material(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_primitive(self) -> Option<Vec<primitive::Primitive>> {
        match self {
            InputData::Primitive(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_procedural_texture(self) -> Option<materials::ProceduralTexture> {
        match self {
            InputData::ProceduralTexture(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_render_pass(self) -> Option<render_passes::RenderPasses> {
        match self {
            InputData::RenderPass(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_scene(self) -> Option<scene::Scene> {
        match self {
            InputData::Scene(value) => Some(value),
            _ => None,
        }
    }
}
