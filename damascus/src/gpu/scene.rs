// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashSet, str::FromStr};

use crevice::std430::AsStd430;
use strum::{Display, EnumCount, EnumIter, EnumString};

use super::PreprocessorDirectives;

use crate::{
    DualDevice, Enumerator,
    camera::{Camera, GPUCamera},
    geometry::{
        BlendType, Repetition,
        primitives::{GPUPrimitive, Primitive, Shapes},
    },
    graph::scene_graph::SceneGraph,
    lights::{GPULight, Light, LightType},
    materials::{GPUMaterial, Material},
    textures::evaluators::{
        TextureEvaluator, checkerboard::GPUCheckerboard, grade::GPUGrade, noise::GPUNoise,
    },
};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUSceneArrayLengths {
    pub primitive_count: u32,
    pub light_count: u32,
    pub material_count: u32,
    pub non_physical_light_count: u32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUScene {
    pub cameras: Vec<GPUCamera>,
    pub primitives: Vec<GPUPrimitive>,
    pub lights: Vec<GPULight>,
    pub materials: Vec<GPUMaterial>,
    pub checkerboards: Vec<GPUCheckerboard>,
    pub noises: Vec<GPUNoise>,
    pub grades: Vec<GPUGrade>,
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
            materials: vec![Material::default().diffuse_colour(glam::Vec3::X).to_gpu()],
            checkerboards: vec![],
            noises: vec![],
            grades: vec![],
            emissive_primitive_indices: vec![],
            render_camera: 0,
            atmosphere: 0,
            array_lengths: GPUSceneArrayLengths {
                primitive_count: 0,
                light_count: 0,
                material_count: 1,
                non_physical_light_count: 0,
            },
            preprocessor_directives: HashSet::<ScenePreprocessorDirectives>::new(),
        }
    }
}

impl GPUScene {
    pub fn emissive_primitive_count(&self) -> usize {
        let mut count = 0;
        for primitive in self.primitives.iter() {
            if self.materials[primitive.material_id as usize].is_emissive() {
                count += 1;
            }
        }
        count
    }
}

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
pub enum ScenePreprocessorDirectives {
    #[default]
    EnableDiffuseColourTexture,
    EnableScatteringColourTexture,
    EnableSpecularProbabilityTexture,
    EnableSpecularRoughnessTexture,
    EnableSpecularColourTexture,
    EnableTransmissiveProbabilityTexture,
    EnableTransmissiveRoughnessTexture,
    EnableEmissiveColourTexture,
    EnableExtinctionColourTexture,
    EnableRefractiveIndexTexture,
    EnableOrbitTrapColour,
    EnableGrade,
    EnableCheckerboard,
    EnableNoise,
    EnableCappedCone,
    EnableCappedTorus,
    EnableCapsule,
    EnableCone,
    EnableCutSphere,
    EnableCylinder,
    EnableDeathStar,
    EnableEllipsoid,
    EnableHexagonalPrism,
    EnableHollowSphere,
    EnableInfiniteCone,
    EnableInfiniteCylinder,
    EnableLink,
    EnableMandelbox,
    EnableMandelbulb,
    EnableOctahedron,
    EnablePlane,
    EnableRectangularPrism,
    EnableRectangularPrismFrame,
    EnableRhombus,
    EnableRoundedCone,
    EnableSolidAngle,
    EnableTorus,
    EnableTriangularPrism,
    EnableChildInteractions,
    EnablePrimitiveBlendSubtraction,
    EnablePrimitiveBlendIntersection,
    EnableInfiniteRepetition,
    EnableFiniteRepetition,
    EnableElongation,
    EnableMirroring,
    EnableHollowing,
    EnableSpecularMaterials,
    EnableTransmissiveMaterials,
    EnablePhysicalLights,
    EnableDirectionalLights,
    EnablePointLights,
    EnableAmbientOcclusion,
    EnableSoftShadows,
}

impl Enumerator for ScenePreprocessorDirectives {}

impl PreprocessorDirectives for ScenePreprocessorDirectives {}

impl ScenePreprocessorDirectives {
    pub fn all_directives_for_material() -> HashSet<Self> {
        HashSet::<Self>::from([
            Self::EnableDiffuseColourTexture,
            Self::EnableScatteringColourTexture,
            Self::EnableSpecularProbabilityTexture,
            Self::EnableSpecularRoughnessTexture,
            Self::EnableSpecularColourTexture,
            Self::EnableTransmissiveProbabilityTexture,
            Self::EnableTransmissiveRoughnessTexture,
            Self::EnableEmissiveColourTexture,
            Self::EnableExtinctionColourTexture,
            Self::EnableRefractiveIndexTexture,
            Self::EnableGrade,
            Self::EnableCheckerboard,
            Self::EnableNoise,
            Self::EnableSpecularMaterials,
            Self::EnableTransmissiveMaterials,
        ])
    }

    pub fn all_directives_for_primitive() -> HashSet<Self> {
        HashSet::<Self>::from([
            Self::EnableCappedCone,
            Self::EnableCappedTorus,
            Self::EnableCapsule,
            Self::EnableCone,
            Self::EnableCutSphere,
            Self::EnableCylinder,
            Self::EnableDeathStar,
            Self::EnableEllipsoid,
            Self::EnableHexagonalPrism,
            Self::EnableHollowSphere,
            Self::EnableInfiniteCone,
            Self::EnableInfiniteCylinder,
            Self::EnableLink,
            Self::EnableMandelbox,
            Self::EnableMandelbulb,
            Self::EnableOctahedron,
            Self::EnablePlane,
            Self::EnableRectangularPrism,
            Self::EnableRectangularPrismFrame,
            Self::EnableRhombus,
            Self::EnableRoundedCone,
            Self::EnableSolidAngle,
            Self::EnableTorus,
            Self::EnableTriangularPrism,
            Self::EnableChildInteractions,
            Self::EnablePrimitiveBlendSubtraction,
            Self::EnablePrimitiveBlendIntersection,
            Self::EnableInfiniteRepetition,
            Self::EnableFiniteRepetition,
            Self::EnableElongation,
            Self::EnableMirroring,
            Self::EnableHollowing,
            Self::EnablePhysicalLights,
            Self::EnableOrbitTrapColour,
        ])
    }

    pub fn all_directives_for_light() -> HashSet<Self> {
        HashSet::<Self>::from([
            Self::EnableDirectionalLights,
            Self::EnablePointLights,
            Self::EnableAmbientOcclusion,
            Self::EnableSoftShadows,
        ])
    }

    pub fn all_directives_for_texture_evaluator() -> HashSet<Self> {
        HashSet::<Self>::from([
            Self::EnableGrade,
            Self::EnableCheckerboard,
            Self::EnableNoise,
        ])
    }

    pub fn directives_for_primitive(primitive: &Primitive) -> HashSet<Self> {
        let mut preprocessor_directives = HashSet::<Self>::new();

        if primitive.blend_type > BlendType::Union || primitive.blend_strength > 0. {
            match primitive.blend_type {
                BlendType::Subtraction => {
                    preprocessor_directives.insert(Self::EnablePrimitiveBlendSubtraction);
                }
                BlendType::Intersection => {
                    preprocessor_directives.insert(Self::EnablePrimitiveBlendIntersection);
                }
                _ => {}
            }
        }

        if primitive.enable_orbit_trap_colour
            && (primitive.shape == Shapes::Mandelbox || primitive.shape == Shapes::Mandelbulb)
        {
            preprocessor_directives.insert(Self::EnableOrbitTrapColour);
        }

        match primitive.repetition {
            Repetition::Finite => {
                preprocessor_directives.insert(Self::EnableFiniteRepetition);
            }
            Repetition::Infinite => {
                preprocessor_directives.insert(Self::EnableInfiniteRepetition);
            }
            _ => {}
        }

        if primitive.elongate {
            preprocessor_directives.insert(Self::EnableElongation);
        }

        if primitive.mirror.any() {
            preprocessor_directives.insert(Self::EnableMirroring);
        }

        if primitive.hollow {
            preprocessor_directives.insert(Self::EnableHollowing);
        }

        if primitive.shape == Shapes::Sphere {
            return preprocessor_directives;
        }

        preprocessor_directives
            .insert(Self::from_str(&("Enable".to_owned() + &primitive.shape.to_string())).unwrap());

        preprocessor_directives
    }

    pub fn directives_for_texture_evaluator(texture_evaluator: &TextureEvaluator) -> HashSet<Self> {
        let mut preprocessor_directives = HashSet::<Self>::new();

        match texture_evaluator {
            TextureEvaluator::Grade(_) => {
                preprocessor_directives.insert(Self::EnableGrade);
            }
            TextureEvaluator::Checkerboard(_) => {
                preprocessor_directives.insert(Self::EnableCheckerboard);
            }
            TextureEvaluator::Noise(_) => {
                preprocessor_directives.insert(Self::EnableNoise);
            }
            _ => {}
        }

        preprocessor_directives
    }

    pub fn directives_for_material(material: &Material, scene_graph: &SceneGraph) -> HashSet<Self> {
        let mut preprocessor_directives = HashSet::<Self>::new();

        if let Some(texture_evaluator_id) = material.diffuse_colour_texture_id
            && scene_graph[texture_evaluator_id] != TextureEvaluator::White
        {
            preprocessor_directives.insert(Self::EnableDiffuseColourTexture);
            preprocessor_directives.extend(Self::directives_for_texture_evaluator(
                &scene_graph[texture_evaluator_id],
            ));
        }
        if let Some(texture_evaluator_id) = material.specular_probability_texture_id
            && scene_graph[texture_evaluator_id] != TextureEvaluator::White
        {
            preprocessor_directives.insert(Self::EnableSpecularProbabilityTexture);
            preprocessor_directives.extend(Self::directives_for_texture_evaluator(
                &scene_graph[texture_evaluator_id],
            ));
        }
        if let Some(texture_evaluator_id) = material.specular_roughness_texture_id
            && scene_graph[texture_evaluator_id] != TextureEvaluator::White
        {
            preprocessor_directives.insert(Self::EnableSpecularRoughnessTexture);
            preprocessor_directives.extend(Self::directives_for_texture_evaluator(
                &scene_graph[texture_evaluator_id],
            ));
        }
        if let Some(texture_evaluator_id) = material.specular_colour_texture_id
            && scene_graph[texture_evaluator_id] != TextureEvaluator::White
        {
            preprocessor_directives.insert(Self::EnableSpecularColourTexture);
            preprocessor_directives.extend(Self::directives_for_texture_evaluator(
                &scene_graph[texture_evaluator_id],
            ));
        }
        if let Some(texture_evaluator_id) = material.transmissive_probability_texture_id
            && scene_graph[texture_evaluator_id] != TextureEvaluator::White
        {
            preprocessor_directives.insert(Self::EnableTransmissiveProbabilityTexture);
            preprocessor_directives.extend(Self::directives_for_texture_evaluator(
                &scene_graph[texture_evaluator_id],
            ));
        }
        if let Some(texture_evaluator_id) = material.transmissive_roughness_texture_id
            && scene_graph[texture_evaluator_id] != TextureEvaluator::White
        {
            preprocessor_directives.insert(Self::EnableTransmissiveRoughnessTexture);
            preprocessor_directives.extend(Self::directives_for_texture_evaluator(
                &scene_graph[texture_evaluator_id],
            ));
        }
        if let Some(texture_evaluator_id) = material.transmissive_colour_texture_id
            && scene_graph[texture_evaluator_id] != TextureEvaluator::White
        {
            preprocessor_directives.insert(Self::EnableExtinctionColourTexture);
            preprocessor_directives.extend(Self::directives_for_texture_evaluator(
                &scene_graph[texture_evaluator_id],
            ));
        }
        if let Some(texture_evaluator_id) = material.emissive_colour_texture_id
            && scene_graph[texture_evaluator_id] != TextureEvaluator::White
        {
            preprocessor_directives.insert(Self::EnableEmissiveColourTexture);
            preprocessor_directives.extend(Self::directives_for_texture_evaluator(
                &scene_graph[texture_evaluator_id],
            ));
        }
        if let Some(texture_evaluator_id) = material.refractive_index_texture_id
            && scene_graph[texture_evaluator_id] != TextureEvaluator::White
        {
            preprocessor_directives.insert(Self::EnableRefractiveIndexTexture);
            preprocessor_directives.extend(Self::directives_for_texture_evaluator(
                &scene_graph[texture_evaluator_id],
            ));
        }
        if let Some(texture_evaluator_id) = material.scattering_colour_texture_id
            && scene_graph[texture_evaluator_id] != TextureEvaluator::White
        {
            preprocessor_directives.insert(Self::EnableScatteringColourTexture);
            preprocessor_directives.extend(Self::directives_for_texture_evaluator(
                &scene_graph[texture_evaluator_id],
            ));
        }

        if material.transmissive_probability > 0. {
            preprocessor_directives.insert(Self::EnableSpecularMaterials);
            preprocessor_directives.insert(Self::EnableTransmissiveMaterials);
        } else if material.specular_probability > 0. {
            preprocessor_directives.insert(Self::EnableSpecularMaterials);
        }

        if material.is_emissive() {
            preprocessor_directives.insert(Self::EnablePhysicalLights);
        }

        preprocessor_directives
    }

    pub fn directives_for_light(light: &Light) -> HashSet<Self> {
        let mut preprocessor_directives = HashSet::<Self>::new();

        match light.light_type {
            LightType::Directional => {
                preprocessor_directives.insert(Self::EnableDirectionalLights);
            }
            LightType::Point => {
                preprocessor_directives.insert(Self::EnablePointLights);
            }
            LightType::AmbientOcclusion => {
                preprocessor_directives.insert(Self::EnableAmbientOcclusion);
            }
            _ => {}
        }

        if light.soften_shadows {
            preprocessor_directives.insert(Self::EnableSoftShadows);
        }

        preprocessor_directives
    }
}
