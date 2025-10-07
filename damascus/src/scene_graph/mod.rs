// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::{HashMap, HashSet};

use crevice::std430::AsStd430;
use glam::Mat4;
use serde_hashkey::to_key_with_ordered_float;
use slotmap::SparseSecondaryMap;
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    camera::{Camera, CameraId, Cameras, Std430GPUCamera},
    geometry::primitives::{GPUPrimitive, Primitive, PrimitiveId, Primitives, Std430GPUPrimitive},
    impl_slot_map_indexing,
    lights::{Light, LightId, Lights, Std430GPULight},
    materials::{Material, MaterialId, Materials, Std430GPUMaterial},
    shaders::scene::ScenePreprocessorDirectives,
    DualDevice, Enumerator,
};

#[derive(
    Debug,
    Display,
    Default,
    Clone,
    Copy,
    EnumCount,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum SceneGraphId {
    #[default]
    None,
    Camera(CameraId),
    Light(LightId),
    Material(MaterialId),
    Primitive(PrimitiveId),
}

impl Enumerator for SceneGraphId {}

#[derive(
    Debug,
    Display,
    Default,
    Clone,
    Copy,
    EnumCount,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum SceneGraphIdType {
    #[default]
    None,
    Camera,
    Light,
    Material,
    Primitive,
}

impl Enumerator for SceneGraphIdType {}

impl From<SceneGraphId> for SceneGraphIdType {
    fn from(scene_graph_location: SceneGraphId) -> Self {
        match scene_graph_location {
            SceneGraphId::None => Self::None,
            SceneGraphId::Camera(..) => Self::Camera,
            SceneGraphId::Light(..) => Self::Light,
            SceneGraphId::Material(..) => Self::Material,
            SceneGraphId::Primitive(..) => Self::Primitive,
        }
    }
}

impl SceneGraphIdType {
    pub fn has_transform(&self) -> bool {
        match self {
            Self::None => false,
            Self::Camera => true,
            Self::Light => true,
            Self::Material => false,
            Self::Primitive => true,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUSceneArrayLengths {
    num_primitives: u32,
    num_lights: u32,
    num_materials: u32,
    num_non_physical_lights: u32,
}

#[derive(Debug, Clone)]
pub struct GPUScene {
    pub primitives: Vec<Std430GPUPrimitive>,
    pub lights: Vec<Std430GPULight>,
    pub materials: Vec<Std430GPUMaterial>,
    pub emissive_primitive_indices: Vec<u32>,
    pub atmosphere: Std430GPUMaterial,
    pub render_camera: Std430GPUCamera,
    pub array_lengths: Std430GPUSceneArrayLengths,
    pub preprocessor_directives: HashSet<ScenePreprocessorDirectives>,
}

impl Default for GPUScene {
    fn default() -> Self {
        Self {
            primitives: vec![],
            lights: vec![],
            materials: vec![Material::default().as_std430()],
            emissive_primitive_indices: vec![],
            atmosphere: Material::default().as_std430(),
            render_camera: Camera::default().as_std430(),
            array_lengths: GPUSceneArrayLengths {
                num_primitives: 0,
                num_lights: 0,
                num_materials: 1,
                num_non_physical_lights: 0,
            }
            .as_std430(),
            preprocessor_directives: HashSet::<ScenePreprocessorDirectives>::new(),
        }
    }
}

impl PartialEq for GPUScene {
    fn eq(&self, other: &Self) -> bool {
        self.atmosphere.as_bytes() == other.atmosphere.as_bytes()
            && self.render_camera.as_bytes() == other.render_camera.as_bytes()
            && self.array_lengths.as_bytes() == other.array_lengths.as_bytes()
            && self.preprocessor_directives == other.preprocessor_directives
            && self.emissive_primitive_indices == other.emissive_primitive_indices
            && self
                .primitives
                .iter()
                .map(|primitive| primitive.as_bytes())
                .collect::<Vec<_>>()
                == other
                    .primitives
                    .iter()
                    .map(|primitive| primitive.as_bytes())
                    .collect::<Vec<_>>()
            && self
                .lights
                .iter()
                .map(|light| light.as_bytes())
                .collect::<Vec<_>>()
                == other
                    .lights
                    .iter()
                    .map(|light| light.as_bytes())
                    .collect::<Vec<_>>()
            && self
                .materials
                .iter()
                .map(|material| material.as_bytes())
                .collect::<Vec<_>>()
                == other
                    .materials
                    .iter()
                    .map(|material| material.as_bytes())
                    .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SceneGraph {
    cameras: Cameras,
    primitives: Primitives,
    lights: Lights,
    materials: Materials,
    primitive_materials: SparseSecondaryMap<PrimitiveId, MaterialId>,
    children: HashMap<SceneGraphId, Vec<SceneGraphId>>,
    root_ids: Vec<SceneGraphId>,
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

    pub fn render_camera_id(&self) -> Option<CameraId> {
        self.render_camera_id
    }

    pub fn atmosphere_id(&self) -> Option<MaterialId> {
        self.atmosphere_id
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

    pub fn set_render_camera(&mut self, render_camera: Camera) {
        let render_camera_id: CameraId = self.add_camera(render_camera);
        self.set_render_camera_id(render_camera_id);
    }

    pub fn set_atmosphere(&mut self, atmosphere: Material) {
        let atmosphere_id: MaterialId = self.add_material(atmosphere);
        self.set_atmosphere_id(atmosphere_id);
    }

    pub fn add_primitive_to_root(&mut self, primitive: Primitive) -> PrimitiveId {
        let root_primitive_id: PrimitiveId = self.add_primitive(primitive);
        self.root_ids
            .push(SceneGraphId::Primitive(root_primitive_id));
        root_primitive_id
    }

    pub fn add_light_to_root(&mut self, light: Light) -> LightId {
        let root_light_id: LightId = self.add_light(light);
        self.root_ids.push(SceneGraphId::Light(root_light_id));
        root_light_id
    }

    pub fn add_camera_to_root(&mut self, camera: Camera) -> CameraId {
        let root_camera_id: CameraId = self.add_camera(camera);
        self.root_ids.push(SceneGraphId::Camera(root_camera_id));
        root_camera_id
    }

    pub fn cloned_children(&self, id: &SceneGraphId) -> Vec<SceneGraphId> {
        let mut child_ids = Vec::<SceneGraphId>::new();
        if let Some(children) = self.children(id) {
            child_ids = children.clone();
        }
        child_ids
    }

    pub fn children(&self, id: &SceneGraphId) -> Option<&Vec<SceneGraphId>> {
        self.children.get(id)
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

    /// Apply a closure to all descendants of `node_id` in depth first order
    fn build_gpu_scene_from_location(
        &self,
        scene_graph_id: &SceneGraphId,
        transform: &Mat4,
        material_ids: &mut HashMap<MaterialId, u32>,
        primitive_ids: &mut HashSet<PrimitiveId>,
        light_ids: &mut HashSet<LightId>,
        camera_ids: &mut HashSet<CameraId>,
        gpu_scene: &mut GPUScene,
    ) {
        if let Some(children) = self.children(scene_graph_id) {
            for child_id in children.iter() {
                match child_id {
                    SceneGraphId::Camera(camera_id) => {}
                    SceneGraphId::Light(light_id) => {}
                    SceneGraphId::Primitive(primitive_id) => {
                        primitive_ids.insert(*primitive_id);

                        let mut primitive: Primitive = self[*primitive_id];

                        primitive.local_to_world *= transform;

                        let mut gpu_primitive: GPUPrimitive = primitive.to_gpu();

                        if let Some(material_id) = self.primitive_materials.get(*primitive_id) {
                            if !material_ids.contains_key(material_id) {
                                gpu_scene.materials.push(self[*material_id].as_std430());

                                let material_index = gpu_scene.materials.len() as u32;
                                material_ids.insert(*material_id, material_index);

                                gpu_primitive.material_id = material_index;
                            } else if let Some(material_index) = material_ids.get(material_id) {
                                gpu_primitive.material_id = *material_index;
                            }
                        }

                        let num_primitives: usize = gpu_scene.primitives.len();
                        gpu_primitive.id = num_primitives as u32 + 1;
                        gpu_scene.primitives.push(gpu_primitive.as_std430());

                        self.build_gpu_scene_from_location(
                            child_id,
                            &primitive.local_to_world,
                            material_ids,
                            primitive_ids,
                            light_ids,
                            camera_ids,
                            gpu_scene,
                        );

                        gpu_primitive.num_descendants =
                            gpu_scene.primitives.len() as u32 - gpu_primitive.id;

                        gpu_scene.primitives[num_primitives] = gpu_primitive.as_std430();
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn create_gpu_scene(&self) -> GPUScene {
        let mut gpu_scene = GPUScene::default();
        let mut material_ids = HashMap::<MaterialId, u32>::new();
        let mut primitive_ids = HashSet::<PrimitiveId>::new();
        let mut light_ids = HashSet::<LightId>::new();
        let mut camera_ids = HashSet::<CameraId>::new();

        for root_id in self.root_ids.iter() {
            // TODO root_id itself is not getting added
            self.build_gpu_scene_from_location(
                root_id,
                &Mat4::IDENTITY,
                &mut material_ids,
                &mut primitive_ids,
                &mut light_ids,
                &mut camera_ids,
                &mut gpu_scene,
            );
        }

        if gpu_scene.primitives.is_empty() {
            gpu_scene.primitives.push(Primitive::default().as_std430());
        }
        if gpu_scene.lights.is_empty() {
            gpu_scene.lights.push(Light::default().as_std430());
        }

        gpu_scene.atmosphere = self.gpu_atmosphere();
        gpu_scene.render_camera = self.gpu_render_camera();
        gpu_scene.array_lengths = self.as_std430();

        gpu_scene.preprocessor_directives = self
            .directives_for_primitives(&primitive_ids)
            .into_iter()
            .chain(self.directives_for_materials(&material_ids.keys().copied().collect()))
            .chain(self.directives_for_lights(&light_ids))
            .collect();

        gpu_scene
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

    pub fn directives_for_primitives(
        &self,
        primitive_ids: &HashSet<PrimitiveId>,
    ) -> HashSet<ScenePreprocessorDirectives> {
        let mut directives = HashSet::<ScenePreprocessorDirectives>::new();
        for primitive_id in primitive_ids.iter() {
            directives.extend(ScenePreprocessorDirectives::directives_for_primitive(
                &self[*primitive_id],
            ));
        }
        directives
    }

    pub fn directives_for_materials(
        &self,
        material_ids: &HashSet<MaterialId>,
    ) -> HashSet<ScenePreprocessorDirectives> {
        let mut directives = HashSet::<ScenePreprocessorDirectives>::new();
        for material_id in material_ids.iter() {
            directives.extend(ScenePreprocessorDirectives::directives_for_material(
                &self[*material_id],
            ));
        }
        directives
    }

    pub fn directives_for_lights(
        &self,
        light_ids: &HashSet<LightId>,
    ) -> HashSet<ScenePreprocessorDirectives> {
        let mut directives = HashSet::<ScenePreprocessorDirectives>::new();
        for light_id in light_ids.iter() {
            directives.extend(ScenePreprocessorDirectives::directives_for_light(
                &self[*light_id],
            ));
        }
        directives
    }
}

impl DualDevice<GPUSceneArrayLengths, Std430GPUSceneArrayLengths> for SceneGraph {
    fn to_gpu(&self) -> GPUSceneArrayLengths {
        GPUSceneArrayLengths {
            num_primitives: self.primitives.len() as u32,
            num_lights: (self.lights.len() + self.num_emissive_primitives()) as u32,
            num_materials: self.materials.len() as u32 + 1,
            num_non_physical_lights: self.lights.len() as u32,
        }
    }
}

impl_slot_map_indexing!(SceneGraph, CameraId, Camera, cameras);
impl_slot_map_indexing!(SceneGraph, PrimitiveId, Primitive, primitives);
impl_slot_map_indexing!(SceneGraph, LightId, Light, lights);
impl_slot_map_indexing!(SceneGraph, MaterialId, Material, materials);
