// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashSet, str::FromStr};

use strum::{Display, EnumCount, EnumIter, EnumString};

use super::PreprocessorDirectives;

use crate::{
    Enumerator,
    geometry::{
        BlendType, Repetition,
        primitives::{Primitive, Shapes},
    },
    lights::{Light, LightType},
    materials::{Material, ProceduralTexture, ProceduralTextureType},
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
    EnableTrapColour,
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
            Self::EnableTrapColour,
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

    pub fn directives_for_primitive(primitive: &Primitive) -> HashSet<Self> {
        let mut preprocessor_directives = HashSet::<Self>::new();

        if primitive.num_descendants > 0
            && (primitive.blend_type > BlendType::Union || primitive.blend_strength > 0.)
        {
            preprocessor_directives.insert(Self::EnableChildInteractions);

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

    pub fn directives_for_procedural_texture(
        procedural_texture: &ProceduralTexture,
    ) -> HashSet<Self> {
        let mut preprocessor_directives = HashSet::<Self>::new();

        if procedural_texture.texture_type == ProceduralTextureType::Grade {
            preprocessor_directives.insert(Self::EnableGrade);
        } else if procedural_texture.texture_type == ProceduralTextureType::Checkerboard {
            preprocessor_directives.insert(Self::EnableCheckerboard);
        } else if procedural_texture.texture_type == ProceduralTextureType::FBMNoise
            || procedural_texture.texture_type == ProceduralTextureType::TurbulenceNoise
        {
            preprocessor_directives.insert(Self::EnableNoise);
        }

        preprocessor_directives
    }

    pub fn directives_for_material(material: &Material) -> HashSet<Self> {
        let mut preprocessor_directives = HashSet::<Self>::new();

        if material.diffuse_colour_texture.texture_type > ProceduralTextureType::None {
            preprocessor_directives.insert(Self::EnableDiffuseColourTexture);
            preprocessor_directives.extend(Self::directives_for_procedural_texture(
                &material.diffuse_colour_texture,
            ));
        }
        if material.specular_probability_texture.texture_type > ProceduralTextureType::None {
            preprocessor_directives.insert(Self::EnableSpecularProbabilityTexture);
            preprocessor_directives.extend(Self::directives_for_procedural_texture(
                &material.specular_probability_texture,
            ));
        }
        if material.specular_roughness_texture.texture_type > ProceduralTextureType::None {
            preprocessor_directives.insert(Self::EnableSpecularRoughnessTexture);
            preprocessor_directives.extend(Self::directives_for_procedural_texture(
                &material.specular_roughness_texture,
            ));
        }
        if material.specular_colour_texture.texture_type > ProceduralTextureType::None {
            preprocessor_directives.insert(Self::EnableSpecularColourTexture);
            preprocessor_directives.extend(Self::directives_for_procedural_texture(
                &material.specular_colour_texture,
            ));
        }
        if material.transmissive_probability_texture.texture_type > ProceduralTextureType::None {
            preprocessor_directives.insert(Self::EnableTransmissiveProbabilityTexture);
            preprocessor_directives.extend(Self::directives_for_procedural_texture(
                &material.transmissive_probability_texture,
            ));
        }
        if material.transmissive_roughness_texture.texture_type > ProceduralTextureType::None {
            preprocessor_directives.insert(Self::EnableTransmissiveRoughnessTexture);
            preprocessor_directives.extend(Self::directives_for_procedural_texture(
                &material.transmissive_roughness_texture,
            ));
        }
        if material.transmissive_colour_texture.texture_type > ProceduralTextureType::None {
            preprocessor_directives.insert(Self::EnableExtinctionColourTexture);
            preprocessor_directives.extend(Self::directives_for_procedural_texture(
                &material.transmissive_colour_texture,
            ));
        }
        if material.emissive_colour_texture.texture_type > ProceduralTextureType::None {
            preprocessor_directives.insert(Self::EnableEmissiveColourTexture);
            preprocessor_directives.extend(Self::directives_for_procedural_texture(
                &material.emissive_colour_texture,
            ));
        }
        if material.refractive_index_texture.texture_type > ProceduralTextureType::None {
            preprocessor_directives.insert(Self::EnableRefractiveIndexTexture);
            preprocessor_directives.extend(Self::directives_for_procedural_texture(
                &material.refractive_index_texture,
            ));
        }
        if material.scattering_colour_texture.texture_type > ProceduralTextureType::None {
            preprocessor_directives.insert(Self::EnableScatteringColourTexture);
            preprocessor_directives.extend(Self::directives_for_procedural_texture(
                &material.scattering_colour_texture,
            ));
        }

        if material.transmissive_probability > 0. {
            preprocessor_directives.insert(Self::EnableSpecularMaterials);
            preprocessor_directives.insert(Self::EnableTransmissiveMaterials);
        } else if material.specular_probability > 0. {
            preprocessor_directives.insert(Self::EnableSpecularMaterials);
        }

        if material.diffuse_colour_texture.use_trap_colour
            || material.specular_colour_texture.use_trap_colour
            || material.emissive_colour_texture.use_trap_colour
        {
            preprocessor_directives.insert(Self::EnableTrapColour);
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
