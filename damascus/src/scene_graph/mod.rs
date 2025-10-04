// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashSet;

use crevice::std430::AsStd430;
use serde_hashkey::to_key_with_ordered_float;
use slotmap::SparseSecondaryMap;
use strum::{Display, EnumCount, EnumIter, EnumString};

use super::{
    camera::{Camera, CameraId, Cameras, Std430GPUCamera},
    geometry::primitives::{Primitive, PrimitiveId, Primitives, Std430GPUPrimitive},
    impl_slot_map_indexing,
    lights::{Light, LightId, Lights, Std430GPULight},
    materials::{Material, MaterialId, Materials, Std430GPUMaterial},
    DualDevice, Enumerator,
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
pub enum SceneGraphLocation {
    #[default]
    None,
    MaterialId(MaterialId),
    PrimitiveId(PrimitiveId),
    LightId(LightId),
    CameraId(CameraId),
}

impl Enumerator for SceneGraphLocation {}

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
    primitive_children: SparseSecondaryMap<PrimitiveId, Vec<PrimitiveId>>,
    primitive_materials: SparseSecondaryMap<PrimitiveId, MaterialId>,
    root_primitive_id: Option<PrimitiveId>,
    render_camera_id: Option<CameraId>,
    atmosphere_id: Option<MaterialId>,
}

impl PartialEq for SceneGraph {
    fn eq(&self, other: &Self) -> bool {
        to_key_with_ordered_float(self) == to_key_with_ordered_float(other)
    }
}

impl SceneGraph {
    pub fn gpu_render_camera(&self) -> Std430GPUCamera {
        if let Some(render_camera_id) = self.render_camera_id {
            return self[render_camera_id].as_std430();
        }
        Camera::default().as_std430()
    }

    // TODO Could make this return an Option instead of a default
    // and use that to trigger a preprocessor directive
    pub fn gpu_atmosphere(&self) -> Std430GPUMaterial {
        if let Some(atmosphere_id) = self.atmosphere_id {
            return self[atmosphere_id].as_std430();
        }
        Material::default().as_std430()
    }

    pub fn render_camera(&self) -> Option<&Camera> {
        if let Some(render_camera_id) = self.render_camera_id {
            return Some(&self[render_camera_id]);
        }
        None
    }

    pub fn atmosphere(&self) -> Option<&Material> {
        if let Some(atmosphere_id) = self.atmosphere_id {
            return Some(&self[atmosphere_id]);
        }
        None
    }

    pub fn root_primitive(&self) -> Option<&Primitive> {
        if let Some(root_primitive_id) = self.root_primitive_id {
            return Some(&self[root_primitive_id]);
        }
        None
    }

    pub fn render_camera_id(&self) -> Option<CameraId> {
        self.render_camera_id
    }

    pub fn atmosphere_id(&self) -> Option<MaterialId> {
        self.atmosphere_id
    }

    pub fn root_primitive_id(&self) -> Option<PrimitiveId> {
        self.root_primitive_id
    }

    pub fn with_render_camera(mut self, render_camera: Camera) -> Self {
        self.set_render_camera(render_camera);
        self
    }

    pub fn with_atmosphere(mut self, atmosphere: Material) -> Self {
        self.set_atmosphere(atmosphere);
        self
    }

    pub fn set_render_camera_id(&mut self, render_camera_id: CameraId) {
        self.render_camera_id = Some(render_camera_id);
    }

    pub fn set_atmosphere_id(&mut self, atmosphere_id: MaterialId) {
        self.atmosphere_id = Some(atmosphere_id);
    }

    pub fn set_root_primitive_id(&mut self, root_primitive_id: PrimitiveId) {
        self.root_primitive_id = Some(root_primitive_id);
    }

    pub fn set_render_camera(&mut self, render_camera: Camera) {
        let render_camera_id: CameraId = self.add_camera(render_camera);
        self.set_render_camera_id(render_camera_id);
    }

    pub fn set_atmosphere(&mut self, atmosphere: Material) {
        let atmosphere_id: MaterialId = self.add_material(atmosphere);
        self.set_atmosphere_id(atmosphere_id);
    }

    pub fn set_root_primitive(&mut self, root_primitive: Primitive) {
        let root_primitive_id: PrimitiveId = self.add_primitive(root_primitive);
        self.set_root_primitive_id(root_primitive_id);
    }

    pub fn cloned_children(&self, primitive_id: PrimitiveId) -> Vec<PrimitiveId> {
        let mut child_ids = Vec::<PrimitiveId>::new();
        if let Some(children) = self.children(primitive_id) {
            child_ids = children.clone();
        }
        child_ids
    }

    pub fn children(&self, primitive_id: PrimitiveId) -> Option<&Vec<PrimitiveId>> {
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

    // TODO wrong order here
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

    pub fn add_primitive(&mut self, primitive: Primitive) -> PrimitiveId {
        self.primitives.insert(primitive)
    }

    pub fn add_light(&mut self, light: Light) -> LightId {
        self.lights.insert(light)
    }

    pub fn add_camera(&mut self, camera: Camera) -> CameraId {
        self.cameras.insert(camera)
    }

    pub fn add_material(&mut self, material: Material) -> MaterialId {
        self.materials.insert(material)
    }

    pub fn num_primitives(&self) -> usize {
        self.primitives.len()
    }

    pub fn num_lights(&self) -> usize {
        self.lights.len()
    }

    pub fn num_cameras(&self) -> usize {
        self.cameras.len()
    }

    pub fn num_materials(&self) -> usize {
        self.materials.len()
    }

    pub fn iter_materials(&self) -> impl Iterator<Item = &Material> + '_ {
        self.materials
            .iter()
            .map(|(_material_id, material)| material)
    }

    pub fn iter_cameras(&self) -> impl Iterator<Item = &Camera> + '_ {
        self.cameras.iter().map(|(_camera_id, camera)| camera)
    }

    pub fn iter_primitives(&self) -> impl Iterator<Item = &Primitive> + '_ {
        self.primitives
            .iter()
            .map(|(_primitive_id, primitive)| primitive)
    }

    pub fn iter_lights(&self) -> impl Iterator<Item = &Light> + '_ {
        self.lights.iter().map(|(_light_id, light)| light)
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
