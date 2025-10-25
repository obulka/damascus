// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::{BTreeSet, HashMap, HashSet};

use glam::Mat4;
use serde_hashkey::to_key_with_ordered_float;
use slotmap::SlotMap;
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    DualDevice, Enumerator, Transformable,
    camera::{Camera, CameraId, Cameras},
    geometry::primitives::{GPUPrimitive, Primitive, PrimitiveId, Primitives},
    gpu::scene::{
        GPUScene, GPUSceneArrayLengths, ScenePreprocessorDirectives, Std430GPUSceneArrayLengths,
    },
    graph::edges::BidirectionalSingleParentEdges,
    impl_slot_map_indexing,
    lights::{Light, LightId, Lights},
    materials::{Material, MaterialId, Materials},
    textures::evaluators::{TextureEvaluator, TextureEvaluatorId, TextureEvaluators},
};

slotmap::new_key_type! { pub struct RootId; }

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
    Ord,
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
    Root(RootId),
    TextureEvaluator(TextureEvaluatorId),
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
    Root,
    TextureEvaluator,
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
            SceneGraphId::Root(..) => Self::Root,
            SceneGraphId::TextureEvaluator(..) => Self::TextureEvaluator,
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
            Self::Root => true,
            Self::TextureEvaluator => false,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Root {
    pub atmosphere_id: Option<MaterialId>,
    pub render_camera_id: Option<CameraId>,
    pub local_to_world: Mat4,
}

impl Transformable for Root {
    fn transform(&mut self, local_to_world: &Mat4) {
        self.local_to_world *= local_to_world;
    }
}

pub type Roots = SlotMap<RootId, Root>;

pub type TransformHierarchy = BidirectionalSingleParentEdges<SceneGraphId, SceneGraphId>;
pub type MaterialPrimitives = BidirectionalSingleParentEdges<MaterialId, PrimitiveId>;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SceneGraph {
    cameras: Cameras,
    primitives: Primitives,
    lights: Lights,
    materials: Materials,
    roots: Roots,
    texture_evaluators: TextureEvaluators,
    material_primitives: MaterialPrimitives,
    edges: TransformHierarchy,
}

impl PartialEq for SceneGraph {
    fn eq(&self, other: &Self) -> bool {
        to_key_with_ordered_float(self) == to_key_with_ordered_float(other)
    }
}

impl SceneGraph {
    pub fn clear(&mut self) {
        self.cameras.clear();
        self.primitives.clear();
        self.lights.clear();
        self.materials.clear();
        self.roots.clear();
        self.texture_evaluators.clear();
        self.material_primitives.clear();
        self.edges.clear();
    }

    pub fn remove(&mut self, scene_graph_id: SceneGraphId) {
        self.edges.disconnect_parent(scene_graph_id);
        match scene_graph_id {
            SceneGraphId::Camera(camera_id) => {
                self.cameras.remove(camera_id);
            }
            SceneGraphId::Light(light_id) => {
                self.lights.remove(light_id);
            }
            SceneGraphId::Primitive(primitive_id) => {
                self.material_primitives.disconnect_child(primitive_id);
                self.primitives.remove(primitive_id);
            }
            SceneGraphId::Material(material_id) => {
                self.material_primitives.disconnect_parent(material_id);
                self.materials.remove(material_id);
            }
            SceneGraphId::Root(root_id) => {
                self.roots.remove(root_id);
            }
            SceneGraphId::TextureEvaluator(texture_evaluator_id) => {
                self.texture_evaluators.remove(texture_evaluator_id);
            }
            SceneGraphId::None => {}
        }
    }

    pub fn add_camera(&mut self, camera: Camera) -> CameraId {
        self.cameras.insert(camera)
    }

    pub fn add_primitive(&mut self, primitive: Primitive) -> PrimitiveId {
        self.primitives.insert(primitive)
    }

    pub fn add_light(&mut self, light: Light) -> LightId {
        self.lights.insert(light)
    }

    pub fn add_material(&mut self, material: Material) -> MaterialId {
        self.materials.insert(material)
    }

    pub fn add_root(&mut self, root: Root) -> RootId {
        self.roots.insert(root)
    }

    pub fn add_texture(&mut self, texture: TextureEvaluator) -> TextureEvaluatorId {
        self.texture_evaluators.insert(texture)
    }

    pub fn num_cameras(&self) -> usize {
        self.cameras.len()
    }

    pub fn num_primitives(&self) -> usize {
        self.primitives.len()
    }

    pub fn num_lights(&self) -> usize {
        self.lights.len()
    }

    pub fn num_materials(&self) -> usize {
        self.materials.len()
    }

    pub fn num_roots(&self) -> usize {
        self.roots.len()
    }

    pub fn num_texture_evaluators(&self) -> usize {
        self.texture_evaluators.len()
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

    pub fn iter_materials(&self) -> impl Iterator<Item = &Material> + '_ {
        self.materials
            .iter()
            .map(|(_material_id, material)| material)
    }

    pub fn iter_roots(&self) -> impl Iterator<Item = &Root> + '_ {
        self.roots.iter().map(|(_root_id, root)| root)
    }

    pub fn iter_texture_evaluators(&self) -> impl Iterator<Item = &TextureEvaluator> + '_ {
        self.texture_evaluators
            .iter()
            .map(|(_texture_id, texture)| texture)
    }

    pub fn has_children(&self, parent_id: SceneGraphId) -> bool {
        self.edges.has_children(parent_id)
    }

    pub fn children(&self, parent_id: SceneGraphId) -> Option<&BTreeSet<SceneGraphId>> {
        self.edges.children(parent_id)
    }

    pub fn add_child(&mut self, parent_id: SceneGraphId, child_id: SceneGraphId) {
        self.edges.connect(parent_id, child_id);
    }

    pub fn set_material(&mut self, primitive_id: PrimitiveId, material_id: MaterialId) {
        self.material_primitives.connect(material_id, primitive_id);
    }

    pub fn num_emissive_primitives(&self) -> usize {
        let mut count = 0;
        for primitive_id in self.primitives.keys() {
            if let Some(material_id) = self.material_primitives.parent(primitive_id) {
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
        scene_graph_ids: &BTreeSet<SceneGraphId>,
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

                    if let Some(children) = self.children(*scene_graph_id) {
                        self.build_gpu_scene_from_locations(
                            children,
                            &camera.camera_to_world,
                            material_ids,
                            primitive_ids,
                            light_ids,
                            camera_ids,
                            gpu_scene,
                        );
                    }
                }
                SceneGraphId::Light(light_id) => {
                    light_ids.insert(*light_id);

                    let mut light: Light = self[*light_id];
                    light.transform(transform);

                    gpu_scene
                        .preprocessor_directives
                        .extend(ScenePreprocessorDirectives::directives_for_light(&light));

                    gpu_scene.lights.push(light.to_gpu());

                    if let Some(children) = self.children(*scene_graph_id) {
                        self.build_gpu_scene_from_locations(
                            children,
                            transform,
                            material_ids,
                            primitive_ids,
                            light_ids,
                            camera_ids,
                            gpu_scene,
                        );
                    }
                }
                SceneGraphId::Primitive(primitive_id) => {
                    primitive_ids.insert(*primitive_id);

                    let mut primitive: Primitive = self[*primitive_id];
                    primitive.transform(transform);

                    gpu_scene.preprocessor_directives.extend(
                        ScenePreprocessorDirectives::directives_for_primitive(&primitive),
                    );

                    let mut gpu_primitive: GPUPrimitive = primitive.to_gpu();

                    if let Some(material_id) = self.material_primitives.parent(*primitive_id) {
                        if !material_ids.contains_key(material_id) {
                            let material: Material = self[*material_id];
                            gpu_scene.preprocessor_directives.extend(
                                ScenePreprocessorDirectives::directives_for_material(
                                    &material, &self,
                                ),
                            );
                            gpu_scene.materials.push(material.to_gpu());

                            let material_index: usize = gpu_scene.materials.len();
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

                    if let Some(children) = self.children(*scene_graph_id) {
                        gpu_scene
                            .preprocessor_directives
                            .insert(ScenePreprocessorDirectives::EnableChildInteractions);
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
                SceneGraphId::Root(root_id) => {
                    if let Some(children) = self.children(*scene_graph_id) {
                        self.build_gpu_scene_from_locations(
                            children,
                            &(self[*root_id].local_to_world * transform),
                            material_ids,
                            primitive_ids,
                            light_ids,
                            camera_ids,
                            gpu_scene,
                        );
                    }
                }
                _ => {}
            }
        }
    }

    pub fn create_gpu_scene(&self, root_id: RootId) -> GPUScene {
        let mut gpu_scene = GPUScene::default();
        let mut material_ids = HashMap::<MaterialId, usize>::new();
        let mut primitive_ids = HashSet::<PrimitiveId>::new();
        let mut light_ids = HashSet::<LightId>::new();
        let mut camera_ids = HashMap::<CameraId, usize>::new();

        if let Some(children) = self.children(SceneGraphId::Root(root_id)) {
            self.build_gpu_scene_from_locations(
                children,
                &self[root_id].local_to_world,
                &mut material_ids,
                &mut primitive_ids,
                &mut light_ids,
                &mut camera_ids,
                &mut gpu_scene,
            );
        }

        if let Some(atmosphere_id) = self[root_id].atmosphere_id {
            if !material_ids.contains_key(&atmosphere_id) {
                gpu_scene.atmosphere = gpu_scene.materials.len();
                gpu_scene.materials.push(self[atmosphere_id].to_gpu());
            } else if let Some(atmosphere_index) = material_ids.get(&atmosphere_id) {
                gpu_scene.atmosphere = *atmosphere_index;
            }
        }

        if let Some(render_camera_id) = self[root_id].render_camera_id {
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

        gpu_scene
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
impl_slot_map_indexing!(SceneGraph, RootId, Root, roots);
impl_slot_map_indexing!(
    SceneGraph,
    TextureEvaluatorId,
    TextureEvaluator,
    texture_evaluators
);
