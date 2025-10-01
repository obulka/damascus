// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashSet;

use crevice::std430::AsStd430;
use slotmap::SparseSecondaryMap;

use super::{
    camera::{Camera, CameraId, Cameras, Std430GPUCamera},
    geometry::primitives::{Primitive, PrimitiveId, Primitives, Std430GPUPrimitive},
    lights::{Light, LightId, Lights, Std430GPULight},
    materials::{Material, MaterialId, Materials, Std430GPUMaterial},
};
use crate::{impl_slot_map_indexing, DualDevice};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUSceneGraphParameters {
    num_primitives: u32,
    num_lights: u32,
    num_materials: u32,
    num_non_physical_lights: u32,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SceneGraph {
    cameras: Cameras,
    primitives: Primitives,
    lights: Lights,
    materials: Materials,
    primitive_children: SparseSecondaryMap<PrimitiveId, HashSet<PrimitiveId>>,
    primitive_materials: SparseSecondaryMap<PrimitiveId, MaterialId>,
    root_primitive: Option<PrimitiveId>,
    render_camera: Option<CameraId>,
    atmosphere: Option<MaterialId>,
}

impl SceneGraph {
    pub fn render_camera(&self) -> Std430GPUCamera {
        if let Some(render_camera_id) = self.render_camera {
            return self[render_camera_id].as_std430();
        }
        Camera::default().as_std430()
    }

    pub fn atmosphere(&self) -> Std430GPUMaterial {
        if let Some(atmosphere_id) = self.atmosphere {
            return self[atmosphere_id].as_std430();
        }
        Material::default().as_std430()
    }

    pub fn set_render_camera(&mut self, render_camera: CameraId) {
        self.render_camera = Some(render_camera);
    }

    pub fn set_atmosphere(&mut self, atmosphere: MaterialId) {
        self.atmosphere = Some(atmosphere);
    }

    pub fn cameras(&self) -> &Cameras {
        &self.cameras
    }

    pub fn lights(&self) -> &Lights {
        &self.lights
    }

    pub fn primitives(&self) -> &Primitives {
        &self.primitives
    }

    pub fn materials(&self) -> &Materials {
        &self.materials
    }

    pub fn children_owned(&self, primitive_id: PrimitiveId) -> HashSet<PrimitiveId> {
        let mut child_ids = HashSet::<PrimitiveId>::new();
        if let Some(children) = self.children(primitive_id) {
            child_ids = children.clone();
        }
        child_ids
    }

    pub fn children(&self, primitive_id: PrimitiveId) -> Option<&HashSet<PrimitiveId>> {
        self.primitive_children.get(primitive_id)
    }

    pub fn num_emissive_primitives(&self) -> usize {
        let mut count = 0;
        for primitive_id in self.primitives.keys() {
            if let Some(material_id) = self.primitive_materials.get(primitive_id) {
                if self[*material_id].scaled_emissive_colour().length() > 0. {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn create_gpu_primitives(&self) -> Vec<Std430GPUPrimitive> {
        let mut gpu_primitives: Vec<Std430GPUPrimitive> = self
            .primitives
            .iter()
            .map(|(_primitive_id, primitive)| *primitive)
            .enumerate()
            .map(|(index, primitive)| {
                let mut gpu_primitive = primitive.to_gpu();
                gpu_primitive.id = (index + 1) as u32;
                gpu_primitive.as_std430()
            })
            .collect();
        if gpu_primitives.is_empty() {
            gpu_primitives.push(Primitive::default().as_std430());
        }
        gpu_primitives
    }

    pub fn create_gpu_lights(&self) -> Vec<Std430GPULight> {
        let mut gpu_lights: Vec<Std430GPULight> = self
            .lights
            .iter()
            .map(|(_light_id, light)| light.as_std430())
            .collect();
        if gpu_lights.is_empty() {
            gpu_lights.push(Light::default().as_std430());
        }
        gpu_lights
    }

    // pub fn emissive_primitive_indices(&self) -> Vec<u32> {
    //     let mut emissive_indices = vec![];
    //     for (index, primitive) in self.primitives.iter().enumerate() {
    //         if primitive.material.scaled_emissive_colour().length() == 0. {
    //             continue;
    //         }
    //         emissive_indices.push(index as u32);
    //     }
    //     if emissive_indices.is_empty() {
    //         emissive_indices.push(0);
    //     }
    //     emissive_indices
    // }

    pub fn clear_primitives(&mut self) {
        self.primitives = Primitives::default();
    }

    pub fn clear_lights(&mut self) {
        self.lights = Lights::default();
    }

    // pub fn merge(&mut self, mut other: Self) {
    //     self.primitives.append(&mut other.primitives);
    //     self.lights.append(&mut other.lights);
    // }
}

impl DualDevice<GPUSceneGraphParameters, Std430GPUSceneGraphParameters> for SceneGraph {
    fn to_gpu(&self) -> GPUSceneGraphParameters {
        GPUSceneGraphParameters {
            num_primitives: self.primitives.len() as u32,
            num_lights: (self.lights.len() + self.num_emissive_primitives()) as u32,
            num_materials: self.materials.len() as u32,
            num_non_physical_lights: self.lights.len() as u32,
        }
    }
}

impl_slot_map_indexing!(SceneGraph, CameraId, Camera, cameras);
impl_slot_map_indexing!(SceneGraph, PrimitiveId, Primitive, primitives);
impl_slot_map_indexing!(SceneGraph, LightId, Light, lights);
impl_slot_map_indexing!(SceneGraph, MaterialId, Material, materials);
