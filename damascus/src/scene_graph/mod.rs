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
    camera::{Camera, CameraId, Cameras, GPUCamera},
    geometry::primitives::{GPUPrimitive, Primitive, PrimitiveId, Primitives},
    impl_slot_map_indexing,
    lights::{GPULight, Light, LightId, Lights},
    materials::{GPUMaterial, Material, MaterialId, Materials},
    shaders::scene::ScenePreprocessorDirectives,
    DualDevice, Enumerator, Transformable,
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
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUSceneArrayLengths {
    num_primitives: u32,
    num_lights: u32,
    num_materials: u32,
    num_non_physical_lights: u32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUScene {
    pub cameras: Vec<GPUCamera>,
    pub primitives: Vec<GPUPrimitive>,
    pub lights: Vec<GPULight>,
    pub materials: Vec<GPUMaterial>,
    pub emissive_primitive_indices: Vec<u32>,
    pub render_camera: usize,
    pub atmosphere: usize,
    pub array_lengths: GPUSceneArrayLengths,
    pub preprocessor_directives: HashSet<ScenePreprocessorDirectives>,
}

impl Default for GPUScene {
    fn default() -> Self {
        Self {
            cameras: vec![Camera::default().to_gpu()],
            primitives: vec![],
            lights: vec![],
            materials: vec![Material::default().to_gpu()],
            emissive_primitive_indices: vec![],
            render_camera: 0,
            atmosphere: 0,
            array_lengths: GPUSceneArrayLengths {
                num_primitives: 0,
                num_lights: 0,
                num_materials: 1,
                num_non_physical_lights: 0,
            },
            preprocessor_directives: HashSet::<ScenePreprocessorDirectives>::new(),
        }
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

    /// Add all descendants of `scene_graph_ids` to the gpu_scene in depth first order
    fn build_gpu_scene_from_locations(
        &self,
        scene_graph_ids: &Vec<SceneGraphId>,
        transform: &Mat4,
        material_ids: &mut HashMap<MaterialId, usize>,
        primitive_ids: &mut HashSet<PrimitiveId>,
        light_ids: &mut HashSet<LightId>,
        camera_ids: &mut HashMap<CameraId, usize>,
        gpu_scene: &mut GPUScene,
    ) {
        for scene_graph_id in scene_graph_ids.iter() {
            match scene_graph_id {
                SceneGraphId::Camera(camera_id) => {
                    if !camera_ids.contains_key(camera_id) {
                        camera_ids.insert(*camera_id, gpu_scene.cameras.len());
                    }

                    let mut camera: Camera = self[*camera_id];
                    camera.transform(transform);

                    gpu_scene.cameras.push(camera.to_gpu());
                }
                SceneGraphId::Light(light_id) => {
                    light_ids.insert(*light_id);

                    let mut light: Light = self[*light_id];
                    light.transform(transform);

                    gpu_scene.lights.push(light.to_gpu());
                }
                SceneGraphId::Primitive(primitive_id) => {
                    primitive_ids.insert(*primitive_id);

                    let mut primitive: Primitive = self[*primitive_id];
                    primitive.transform(transform);

                    let mut gpu_primitive: GPUPrimitive = primitive.to_gpu();

                    if let Some(material_id) = self.primitive_materials.get(*primitive_id) {
                        if !material_ids.contains_key(material_id) {
                            let material_index: usize = gpu_scene.materials.len();
                            gpu_scene.materials.push(self[*material_id].to_gpu());

                            material_ids.insert(*material_id, material_index);

                            gpu_primitive.material_id = material_index as u32;
                        } else if let Some(material_index) = material_ids.get(material_id) {
                            gpu_primitive.material_id = *material_index as u32;
                        }
                    }

                    let num_primitives: usize = gpu_scene.primitives.len();
                    let gpu_primitive_id: u32 = num_primitives as u32 + 1;
                    gpu_primitive.id = gpu_primitive_id;
                    gpu_scene.primitives.push(gpu_primitive);

                    if let Some(children) = self.children(scene_graph_id) {
                        self.build_gpu_scene_from_locations(
                            children,
                            &primitive.local_to_world,
                            material_ids,
                            primitive_ids,
                            light_ids,
                            camera_ids,
                            gpu_scene,
                        );
                    }

                    gpu_scene.primitives[num_primitives].num_descendants =
                        gpu_scene.primitives.len() as u32 - gpu_primitive_id;
                }
                _ => {}
            }
        }
    }

    pub fn create_gpu_scene(&self) -> GPUScene {
        let mut gpu_scene = GPUScene::default();
        let mut material_ids = HashMap::<MaterialId, usize>::new();
        let mut primitive_ids = HashSet::<PrimitiveId>::new();
        let mut light_ids = HashSet::<LightId>::new();
        let mut camera_ids = HashMap::<CameraId, usize>::new();

        self.build_gpu_scene_from_locations(
            &self.root_ids,
            &Mat4::IDENTITY,
            &mut material_ids,
            &mut primitive_ids,
            &mut light_ids,
            &mut camera_ids,
            &mut gpu_scene,
        );

        if let Some(atmosphere_id) = self.atmosphere_id {
            if !material_ids.contains_key(&atmosphere_id) {
                gpu_scene.atmosphere = gpu_scene.materials.len();
                gpu_scene.materials.push(self[atmosphere_id].to_gpu());
            } else if let Some(atmosphere_index) = material_ids.get(&atmosphere_id) {
                gpu_scene.atmosphere = *atmosphere_index;
            }
        }

        if let Some(render_camera_id) = self.render_camera_id {
            if !camera_ids.contains_key(&render_camera_id) {
                gpu_scene.render_camera = gpu_scene.cameras.len();
                gpu_scene.cameras.push(self[render_camera_id].to_gpu());
            } else if let Some(render_camera_index) = camera_ids.get(&render_camera_id) {
                gpu_scene.render_camera = *render_camera_index;
            }
        }

        if gpu_scene.primitives.is_empty() {
            gpu_scene.primitives.push(Primitive::default().to_gpu());
        }
        if gpu_scene.lights.is_empty() {
            gpu_scene.lights.push(Light::default().to_gpu());
        }
        if gpu_scene.cameras.is_empty() {
            gpu_scene.cameras.push(Camera::default().to_gpu());
        }

        gpu_scene.array_lengths = self.to_gpu();

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
