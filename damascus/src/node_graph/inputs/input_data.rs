// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt;

use glam::{BVec3, Mat3, Mat4, UVec2, UVec3, Vec2, Vec3, Vec4};
use strum::{Display, EnumIter, EnumString};

use crate::{
    camera::Camera,
    geometry::primitives::Primitive,
    lights::Light,
    materials::{Material, ProceduralTexture},
    render_passes::RenderPasses,
    scene::Scene,
    Enum, Enumerator, Error,
};

#[derive(Debug, Clone)]
pub struct InputDowncastError {
    data: InputData,
}

type Result<T> = std::result::Result<T, InputDowncastError>;

impl InputDowncastError {
    pub fn new(data: InputData) -> Self {
        Self { data: data }
    }
}

impl fmt::Display for InputDowncastError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid cast from input data of type: {:?} to: {:?}",
            self.data,
            match self.data {
                InputData::Bool(..) => "bool",
                InputData::BVec3(..) => "BVec3",
                InputData::Enum(..) => "Enum",
                InputData::Filepath(..) => "String",
                InputData::Int(..) => "i32",
                InputData::UInt(..) => "u32",
                InputData::UVec2(..) => "UVec2",
                InputData::UVec3(..) => "UVec3",
                InputData::Float(..) => "f32",
                InputData::Vec2(..) => "Vec2",
                InputData::Vec3(..) => "Vec3",
                InputData::Vec4(..) => "Vec4",
                InputData::Mat3(..) => "Mat3",
                InputData::Mat4(..) => "Mat4",
                InputData::Camera(..) => "Camera",
                InputData::Light(..) => "Lights",
                InputData::Material(..) => "Material",
                InputData::Primitive(..) => "Primitives",
                InputData::ProceduralTexture(..) => "ProceduralTexture",
                InputData::RenderPass(..) => "RenderPasses",
                InputData::Scene(..) => "Scene",
                _ => "None",
            }
        )
    }
}

impl Error for InputDowncastError {}

#[derive(
    Debug, Display, Default, Clone, EnumIter, EnumString, serde::Serialize, serde::Deserialize,
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
    Camera(Camera),
    Light(Vec<Light>),
    Material(Material),
    Primitive(Vec<Primitive>),
    ProceduralTexture(ProceduralTexture),
    RenderPass(RenderPasses),
    Scene(Scene),
}

impl InputData {
    pub fn try_to_bool(self) -> Result<bool> {
        match self {
            InputData::Bool(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_bvec3(self) -> Result<BVec3> {
        match self {
            InputData::BVec3(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_int(self) -> Result<i32> {
        match self {
            InputData::Int(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_uint(self) -> Result<u32> {
        match self {
            InputData::UInt(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_uvec2(self) -> Result<UVec2> {
        match self {
            InputData::UVec2(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_uvec3(self) -> Result<UVec3> {
        match self {
            InputData::UVec3(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_enum<E: Enumerator>(self) -> Result<E> {
        match self {
            InputData::Enum(value) => Ok(value.as_enumerator()),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_filepath(self) -> Result<String> {
        match self {
            InputData::Filepath(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_float(self) -> Result<f32> {
        match self {
            InputData::Float(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_vec2(self) -> Result<Vec2> {
        match self {
            InputData::Vec2(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_vec3(self) -> Result<Vec3> {
        match self {
            InputData::Vec3(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_vec4(self) -> Result<Vec4> {
        match self {
            InputData::Vec4(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_mat3(self) -> Result<Mat3> {
        match self {
            InputData::Mat3(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_mat4(self) -> Result<Mat4> {
        match self {
            InputData::Mat4(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_camera(self) -> Result<Camera> {
        match self {
            InputData::Camera(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_light(self) -> Result<Vec<Light>> {
        match self {
            InputData::Light(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_material(self) -> Result<Material> {
        match self {
            InputData::Material(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_primitive(self) -> Result<Vec<Primitive>> {
        match self {
            InputData::Primitive(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_procedural_texture(self) -> Result<ProceduralTexture> {
        match self {
            InputData::ProceduralTexture(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_render_pass(self) -> Result<RenderPasses> {
        match self {
            InputData::RenderPass(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }

    pub fn try_to_scene(self) -> Result<Scene> {
        match self {
            InputData::Scene(value) => Ok(value),
            _ => InputDowncastError::new(self).as_err(),
        }
    }
}

impl Enumerator for InputData {}
