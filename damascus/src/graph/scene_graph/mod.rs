// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::{BTreeSet, HashMap, HashSet};

use glam::Mat4;
use macro_rules_attribute::derive;
use serde_hashkey::to_key_with_ordered_float;
use slotmap::SlotMap;

use crate::{
    DualDevice, EnumHashTraits, Transformable,
    camera::{Camera, CameraId, Cameras},
    geometry::primitives::{GPUPrimitive, Primitive, PrimitiveId, Primitives},
    gpu::scene::{GPUScene, GPUSceneArrayLengths, ScenePreprocessorDirectives},
    graph::{
        BidirectedGraph,
        edges::{BidirectedEdges, MultiParentBidirectedEdges, SingleParentBidirectedEdges},
    },
    impl_slot_map_indexing,
    lights::{Light, LightId, Lights},
    materials::{Material, MaterialId, Materials},
    textures::evaluators::{
        TextureEvaluatorId, TextureEvaluators, TextureEvaluatorsMap, checkerboard::Checkerboard,
        grade::Grade, noise::Noise,
    },
};

slotmap::new_key_type! { pub struct RootId; }

#[derive(Clone, Default)]
pub enum SceneGraphData {
    #[default]
    None,
    Camera(Camera),
    Light(Light),
    Material(Material),
    Primitive(Primitive),
    Root(Root),
    TextureEvaluator(TextureEvaluators),
}

impl From<Material> for SceneGraphData {
    fn from(material: Material) -> Self {
        Self::Material(material)
    }
}

impl From<Camera> for SceneGraphData {
    fn from(camera: Camera) -> Self {
        Self::Camera(camera)
    }
}

impl From<Light> for SceneGraphData {
    fn from(light: Light) -> Self {
        Self::Light(light)
    }
}

impl From<Primitive> for SceneGraphData {
    fn from(primitive: Primitive) -> Self {
        Self::Primitive(primitive)
    }
}

impl From<Root> for SceneGraphData {
    fn from(root: Root) -> Self {
        Self::Root(root)
    }
}

impl From<TextureEvaluators> for SceneGraphData {
    fn from(texture_evaluator: TextureEvaluators) -> Self {
        Self::TextureEvaluator(texture_evaluator)
    }
}

#[derive(Copy, Default, EnumHashTraits!)]
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

impl From<MaterialId> for SceneGraphId {
    fn from(material_id: MaterialId) -> Self {
        Self::Material(material_id)
    }
}

impl From<CameraId> for SceneGraphId {
    fn from(camera_id: CameraId) -> Self {
        Self::Camera(camera_id)
    }
}

impl From<LightId> for SceneGraphId {
    fn from(light_id: LightId) -> Self {
        Self::Light(light_id)
    }
}

impl From<PrimitiveId> for SceneGraphId {
    fn from(primitive_id: PrimitiveId) -> Self {
        Self::Primitive(primitive_id)
    }
}

impl From<RootId> for SceneGraphId {
    fn from(root_id: RootId) -> Self {
        Self::Root(root_id)
    }
}

impl From<TextureEvaluatorId> for SceneGraphId {
    fn from(texture_evaluator_id: TextureEvaluatorId) -> Self {
        Self::TextureEvaluator(texture_evaluator_id)
    }
}

#[derive(Copy, Default, EnumHashTraits!)]
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
    pub local_to_world: Mat4,
}

impl Transformable for Root {
    fn transform(&mut self, local_to_world: &Mat4) {
        self.local_to_world *= local_to_world;
    }
}

pub type Roots = SlotMap<RootId, Root>;

pub type TransformHierarchy = MultiParentBidirectedEdges<SceneGraphId, SceneGraphId>;
pub type MaterialPrimitives = SingleParentBidirectedEdges<MaterialId, PrimitiveId>;
pub type RenderCameras = SingleParentBidirectedEdges<CameraId, RootId>;
pub type Atmospheres = SingleParentBidirectedEdges<MaterialId, RootId>;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SceneGraph {
    cameras: Cameras,
    primitives: Primitives,
    lights: Lights,
    materials: Materials,
    roots: Roots,
    texture_evaluators: TextureEvaluatorsMap,
    material_primitives: MaterialPrimitives,
    transform_hierarchy: TransformHierarchy,
    render_cameras: RenderCameras,
    atmospheres: Atmospheres,
}

impl PartialEq for SceneGraph {
    fn eq(&self, other: &Self) -> bool {
        to_key_with_ordered_float(self) == to_key_with_ordered_float(other)
    }
}

impl BidirectedGraph<SceneGraphId> for SceneGraph {
    fn node_count(&self) -> usize {
        self.transform_hierarchy.parent_count()
    }

    fn clear(&mut self) {
        self.cameras.clear();
        self.primitives.clear();
        self.lights.clear();
        self.materials.clear();
        self.roots.clear();
        self.texture_evaluators.clear();
        self.material_primitives.clear();
        self.transform_hierarchy.clear();
        self.render_cameras.clear();
        self.atmospheres.clear();
    }

    fn iter_children<'a>(
        &'a self,
        scene_graph_id: &'a SceneGraphId,
    ) -> impl Iterator<Item = &'a SceneGraphId> + 'a
    where
        SceneGraphId: 'a,
    {
        self.transform_hierarchy.iter_children(scene_graph_id)
    }

    fn iter_parents<'a>(
        &'a self,
        scene_graph_id: &'a SceneGraphId,
    ) -> impl Iterator<Item = &'a SceneGraphId> + 'a
    where
        SceneGraphId: 'a,
    {
        self.transform_hierarchy.iter_parents(scene_graph_id)
    }
}

impl SceneGraph {
    pub fn remove(
        &mut self,
        scene_graph_id: SceneGraphId,
    ) -> BTreeSet<(SceneGraphId, SceneGraphId)> {
        let mut disconnected_edges: BTreeSet<(SceneGraphId, SceneGraphId)> = self
            .transform_hierarchy
            .disconnect_children_of_parent(&scene_graph_id)
            .map(|child_id| (scene_graph_id, child_id))
            .collect();
        disconnected_edges.extend(
            self.transform_hierarchy
                .disconnect_parents_of_child(&scene_graph_id)
                .map(|parent_id| (parent_id, scene_graph_id)),
        );
        match scene_graph_id {
            SceneGraphId::Camera(camera_id) => {
                disconnected_edges.extend(
                    self.render_cameras
                        .disconnect_children_of_parent(&camera_id)
                        .map(|root_id| (camera_id.into(), root_id.into())),
                );
                self.cameras.remove(camera_id);
            }
            SceneGraphId::Light(light_id) => {
                self.lights.remove(light_id);
            }
            SceneGraphId::Primitive(primitive_id) => {
                disconnected_edges.extend(
                    self.material_primitives
                        .disconnect_parents_of_child(&primitive_id)
                        .map(|material_id| (material_id.into(), primitive_id.into())),
                );
                self.primitives.remove(primitive_id);
            }
            SceneGraphId::Material(material_id) => {
                disconnected_edges.extend(
                    self.material_primitives
                        .disconnect_children_of_parent(&material_id)
                        .map(|primitive_id| (material_id.into(), primitive_id.into())),
                );
                disconnected_edges.extend(
                    self.atmospheres
                        .disconnect_children_of_parent(&material_id)
                        .map(|root_id| (material_id.into(), root_id.into())),
                );
                self.materials.remove(material_id);
            }
            SceneGraphId::Root(root_id) => {
                disconnected_edges.extend(
                    self.render_cameras
                        .disconnect_parents_of_child(&root_id)
                        .map(|camera_id| (camera_id.into(), root_id.into())),
                );
                disconnected_edges.extend(
                    self.atmospheres
                        .disconnect_parents_of_child(&root_id)
                        .map(|material_id| (material_id.into(), root_id.into())),
                );
                self.roots.remove(root_id);
            }
            SceneGraphId::TextureEvaluator(texture_evaluator_id) => {
                self.texture_evaluators.remove(texture_evaluator_id);
            }
            SceneGraphId::None => {}
        }
        disconnected_edges
    }

    pub fn add_data(&mut self, data: SceneGraphData) -> SceneGraphId {
        match data {
            SceneGraphData::Camera(camera) => self.add_camera(camera).into(),
            SceneGraphData::Light(light) => self.add_light(light).into(),
            SceneGraphData::Material(material) => self.add_material(material).into(),
            SceneGraphData::Primitive(primitive) => self.add_primitive(primitive).into(),
            SceneGraphData::Root(root) => self.add_root(root).into(),
            SceneGraphData::TextureEvaluator(texture_evaluator) => {
                self.add_texture_evaluator(texture_evaluator).into()
            }
            _ => SceneGraphId::None,
        }
    }

    pub fn add_camera(&mut self, camera: Camera) -> CameraId {
        self.cameras.insert(camera)
    }

    pub fn add_light(&mut self, light: Light) -> LightId {
        self.lights.insert(light)
    }

    pub fn add_material(&mut self, material: Material) -> MaterialId {
        self.materials.insert(material)
    }

    pub fn add_primitive(&mut self, primitive: Primitive) -> PrimitiveId {
        self.primitives.insert(primitive)
    }

    pub fn add_root(&mut self, root: Root) -> RootId {
        self.roots.insert(root)
    }

    pub fn add_texture_evaluator(&mut self, texture: TextureEvaluators) -> TextureEvaluatorId {
        self.texture_evaluators.insert(texture)
    }

    pub fn camera_count(&self) -> usize {
        self.cameras.len()
    }

    pub fn primitive_count(&self) -> usize {
        self.primitives.len()
    }

    pub fn light_count(&self) -> usize {
        self.lights.len()
    }

    pub fn material_count(&self) -> usize {
        self.materials.len()
    }

    pub fn root_count(&self) -> usize {
        self.roots.len()
    }

    pub fn texture_evaluator_count(&self) -> usize {
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

    pub fn iter_texture_evaluators(&self) -> impl Iterator<Item = &TextureEvaluators> + '_ {
        self.texture_evaluators
            .iter()
            .map(|(_texture_id, texture)| texture)
    }

    pub fn add_child(&mut self, parent_id: SceneGraphId, child_id: SceneGraphId) {
        self.transform_hierarchy.connect(parent_id, child_id);
    }

    pub fn set_material(&mut self, primitive_id: PrimitiveId, material_id: MaterialId) {
        self.material_primitives.connect(material_id, primitive_id);
    }

    pub fn set_atmosphere(&mut self, root_id: RootId, atmosphere_id: MaterialId) {
        self.atmospheres.connect(atmosphere_id, root_id);
    }

    pub fn set_render_camera(&mut self, root_id: RootId, render_camera_id: CameraId) {
        self.render_cameras.connect(render_camera_id, root_id);
    }

    pub fn emissive_primitive_count(&self) -> usize {
        let mut count = 0;
        for primitive_id in self.primitives.keys() {
            if let Some(material_id) = self.material_primitives.parent(&primitive_id) {
                if self[*material_id].is_emissive() {
                    count += 1;
                }
            }
        }
        count
    }

    /// Add all descendants of `scene_graph_ids` to the gpu_scene in depth first order
    fn build_gpu_scene_from_locations<'a>(
        &'a self,
        scene_graph_ids: impl Iterator<Item = &'a SceneGraphId>,
        transform: &'a Mat4,
        material_ids: &'a mut HashMap<MaterialId, usize>,
        primitive_ids: &'a mut HashSet<PrimitiveId>,
        light_ids: &'a mut HashSet<LightId>,
        camera_ids: &'a mut HashMap<CameraId, usize>,
        gpu_scene: &'a mut GPUScene,
    ) {
        for scene_graph_id in scene_graph_ids {
            match scene_graph_id {
                SceneGraphId::Camera(camera_id) => {
                    let mut camera: Camera = self[*camera_id];
                    camera.transform(transform);

                    camera_ids.insert(*camera_id, gpu_scene.cameras.len());

                    gpu_scene.cameras.push(camera.to_gpu());

                    self.build_gpu_scene_from_locations(
                        self.iter_children(scene_graph_id),
                        &camera.camera_to_world,
                        material_ids,
                        primitive_ids,
                        light_ids,
                        camera_ids,
                        gpu_scene,
                    );
                }
                SceneGraphId::Light(light_id) => {
                    let mut light: Light = self[*light_id];
                    light.transform(transform);
                    gpu_scene.lights.push(light.to_gpu());

                    if light_ids.insert(*light_id) {
                        gpu_scene
                            .preprocessor_directives
                            .extend(ScenePreprocessorDirectives::directives_for_light(&light));
                    }

                    self.build_gpu_scene_from_locations(
                        self.iter_children(scene_graph_id),
                        transform,
                        material_ids,
                        primitive_ids,
                        light_ids,
                        camera_ids,
                        gpu_scene,
                    );
                }
                SceneGraphId::Primitive(primitive_id) => {
                    let mut primitive: Primitive = self[*primitive_id];
                    primitive.transform(transform);

                    let mut gpu_primitive: GPUPrimitive = primitive.to_gpu();

                    if primitive_ids.insert(*primitive_id) {
                        gpu_scene.preprocessor_directives.extend(
                            ScenePreprocessorDirectives::directives_for_primitive(&primitive),
                        );
                    }

                    if let Some(material_id) = self.material_primitives.parent(primitive_id) {
                        if !material_ids.contains_key(material_id) {
                            let material: Material = self[*material_id];
                            gpu_scene.preprocessor_directives.extend(
                                ScenePreprocessorDirectives::directives_for_material(
                                    &material, &self,
                                ),
                            );

                            let material_index: usize = gpu_scene.materials.len();

                            gpu_scene.materials.push(material.to_gpu());

                            material_ids.insert(*material_id, material_index);

                            gpu_primitive.material_id = material_index as u32;
                        } else if let Some(material_index) = material_ids.get(material_id) {
                            gpu_primitive.material_id = *material_index as u32;
                        }
                    }

                    let primitive_count: usize = gpu_scene.primitives.len();
                    let gpu_primitive_id: u32 = primitive_count as u32 + 1;
                    gpu_primitive.id = gpu_primitive_id;
                    gpu_scene.primitives.push(gpu_primitive);

                    if self.has_child(scene_graph_id) {
                        gpu_scene
                            .preprocessor_directives
                            .insert(ScenePreprocessorDirectives::EnableChildInteractions);
                        self.build_gpu_scene_from_locations(
                            self.iter_children(scene_graph_id),
                            &primitive.local_to_world,
                            material_ids,
                            primitive_ids,
                            light_ids,
                            camera_ids,
                            gpu_scene,
                        );
                    }

                    gpu_scene.primitives[primitive_count].descendant_count =
                        gpu_scene.primitives.len() as u32 - gpu_primitive_id;
                }
                SceneGraphId::Root(root_id) => {
                    self.build_gpu_scene_from_locations(
                        self.iter_children(scene_graph_id),
                        &(self[*root_id].local_to_world * transform),
                        material_ids,
                        primitive_ids,
                        light_ids,
                        camera_ids,
                        gpu_scene,
                    );
                }
                _ => {}
            }
        }
    }

    pub fn as_gpu_scene(&self, root_id: RootId) -> GPUScene {
        let mut gpu_scene = GPUScene::default();
        let mut material_ids = HashMap::<MaterialId, usize>::new();
        let mut primitive_ids = HashSet::<PrimitiveId>::new();
        let mut light_ids = HashSet::<LightId>::new();
        let mut camera_ids = HashMap::<CameraId, usize>::new();

        self.build_gpu_scene_from_locations(
            self.iter_children(&SceneGraphId::Root(root_id)),
            &self[root_id].local_to_world,
            &mut material_ids,
            &mut primitive_ids,
            &mut light_ids,
            &mut camera_ids,
            &mut gpu_scene,
        );

        if let Some(atmosphere_id) = self.atmospheres.parent(&root_id) {
            if !material_ids.contains_key(atmosphere_id) {
                gpu_scene.atmosphere = gpu_scene.materials.len();
                gpu_scene.materials.push(self[*atmosphere_id].to_gpu());
            } else if let Some(atmosphere_index) = material_ids.get(atmosphere_id) {
                gpu_scene.atmosphere = *atmosphere_index;
            }
        }

        if let Some(render_camera_id) = self.render_cameras.parent(&root_id) {
            if !camera_ids.contains_key(render_camera_id) {
                gpu_scene.render_camera = gpu_scene.cameras.len();
                gpu_scene.cameras.push(self[*render_camera_id].to_gpu());
            } else if let Some(render_camera_index) = camera_ids.get(render_camera_id) {
                // TODO the same camera could be at multiple places in the hierarchy
                // this refers to the last added but should be specifiable in order
                // to pick up either transform
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
        if gpu_scene.checkerboards.is_empty() {
            gpu_scene
                .checkerboards
                .push(Checkerboard::default().to_gpu());
        }
        if gpu_scene.noises.is_empty() {
            gpu_scene.noises.push(Noise::default().to_gpu());
        }
        if gpu_scene.grades.is_empty() {
            gpu_scene.grades.push(Grade::default().to_gpu());
        }
        if gpu_scene.emissive_primitive_indices.is_empty() {
            gpu_scene.emissive_primitive_indices.push(0);
        }

        let light_count = gpu_scene.lights.len() as u32;
        gpu_scene.array_lengths = GPUSceneArrayLengths {
            primitive_count: gpu_scene.primitives.len() as u32,
            light_count: light_count + gpu_scene.emissive_primitive_count() as u32,
            material_count: gpu_scene.materials.len() as u32,
            non_physical_light_count: light_count,
        };

        gpu_scene
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
    TextureEvaluators,
    texture_evaluators
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let mut scene_graph = SceneGraph::default();

        scene_graph.add_camera(Camera::default());
        scene_graph.add_primitive(Primitive::default());
        scene_graph.add_light(Light::default());
        scene_graph.add_material(Material::default());
        scene_graph.add_root(Root::default());
        scene_graph.add_texture_evaluator(TextureEvaluators::default());

        assert_eq!(scene_graph.camera_count(), 1);
        assert_eq!(scene_graph.primitive_count(), 1);
        assert_eq!(scene_graph.light_count(), 1);
        assert_eq!(scene_graph.material_count(), 1);
        assert_eq!(scene_graph.root_count(), 1);
        assert_eq!(scene_graph.texture_evaluator_count(), 1);

        scene_graph.add_camera(Camera::default());
        assert_eq!(scene_graph.camera_count(), 2);
    }

    #[test]
    fn test_children() {
        let mut scene_graph = SceneGraph::default();

        // -------------------------------------------------------------
        // /root

        let root_id = scene_graph.add_root(Root::default());

        let mut gpu_scene: GPUScene = scene_graph.as_gpu_scene(root_id);

        assert_eq!(gpu_scene.materials.len(), 1);
        assert_eq!(gpu_scene.primitives.len(), 1);
        assert_eq!(gpu_scene.lights.len(), 1);
        assert_eq!(gpu_scene.emissive_primitive_indices.len(), 1);

        // -------------------------------------------------------------
        // /root

        let camera_id = scene_graph.add_camera(Camera::default());
        let primitive0_id = scene_graph.add_primitive(Primitive::default());
        let primitive1_id = scene_graph.add_primitive(Primitive::default());
        let light_id = scene_graph.add_light(Light::default());
        let material0_id = scene_graph.add_material(Material::default().specular_probability(1.));
        let material1_id = scene_graph.add_material(Material::default());

        assert_eq!(scene_graph[material0_id].specular_probability, 1.);
        assert_eq!(scene_graph[material1_id].specular_probability, 0.);

        gpu_scene = scene_graph.as_gpu_scene(root_id);

        assert_eq!(gpu_scene.materials.len(), 1);
        assert_eq!(gpu_scene.primitives.len(), 1);
        assert_eq!(gpu_scene.lights.len(), 1);
        assert_eq!(gpu_scene.emissive_primitive_indices.len(), 1);

        // -------------------------------------------------------------
        // /root/primitive0/primitive1/material1
        //      |          /light/camera
        //      |          /material0
        //      /primitive1/material1

        scene_graph.add_child(root_id.into(), primitive0_id.into());
        scene_graph.add_child(root_id.into(), primitive1_id.into());
        scene_graph.add_child(primitive0_id.into(), primitive1_id.into());
        scene_graph.add_child(primitive0_id.into(), light_id.into());
        scene_graph.add_child(light_id.into(), camera_id.into());

        scene_graph.set_material(primitive0_id, material0_id);
        scene_graph.set_material(primitive1_id, material1_id);

        assert_eq!(scene_graph.iter_children(&root_id.into()).count(), 2);

        gpu_scene = scene_graph.as_gpu_scene(root_id);

        assert_eq!(gpu_scene.primitives[0].descendant_count, 1);
        assert_eq!(gpu_scene.primitives[1].descendant_count, 0);
        assert_eq!(gpu_scene.materials.len(), 3);
        assert_eq!(gpu_scene.primitives.len(), 3);

        // -------------------------------------------------------------
        // /root/primitive0/primitive1/material1
        //      |          /light/camera
        //      |          /material0
        //      |          /primitive2
        //      /primitive1/material1

        let primitive2_id = scene_graph.add_primitive(Primitive::default());
        scene_graph.add_child(primitive0_id.into(), primitive2_id.into());

        gpu_scene = scene_graph.as_gpu_scene(root_id);

        assert_eq!(gpu_scene.primitives[0].descendant_count, 2);
        assert_eq!(gpu_scene.primitives[1].descendant_count, 0);

        // -------------------------------------------------------------
        // /root/primitive0/primitive1/material1
        //      |          |          /primitive3
        //      |          /light/camera
        //      |          /material0
        //      |          /primitive2
        //      /primitive1/material1
        //                 /primitive3

        let primitive3_id = scene_graph.add_primitive(Primitive::default());
        scene_graph.add_child(primitive1_id.into(), primitive3_id.into());

        gpu_scene = scene_graph.as_gpu_scene(root_id);

        assert_eq!(gpu_scene.primitives[0].descendant_count, 3);
        assert_eq!(gpu_scene.primitives[1].descendant_count, 1);

        // -------------------------------------------------------------

        assert_eq!(gpu_scene.atmosphere, 0);
        assert_eq!(gpu_scene.render_camera, 0);

        // -------------------------------------------------------------
        // /root/primitive0/primitive1/material1
        //      |          |          /primitive3
        //      |          /light/camera
        //      |          /material0
        //      |          /primitive2
        //      /primitive1/material1
        //      |          /primitive3
        //      /material0
        //      /camera

        scene_graph.set_atmosphere(root_id, material0_id);
        scene_graph.set_render_camera(root_id, camera_id);

        gpu_scene = scene_graph.as_gpu_scene(root_id);

        assert_eq!(
            gpu_scene.materials[gpu_scene.atmosphere].specular_probability,
            1.
        );
        assert_eq!(gpu_scene.render_camera, 1);
    }
}
