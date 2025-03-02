// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashSet, str::FromStr};

use strum::{EnumCount, EnumIter, EnumString};

use super::{process_shader_source, CompilerSettings, PreprocessorDirectives};

use crate::{
    geometry::{
        primitive::{Primitive, Shapes},
        BlendType, Repetition,
    },
    lights::{Light, Lights},
    materials::{Material, ProceduralTexture, ProceduralTextureType},
    renderers::ray_marcher::{AOVs, GPURayMarcher, RayMarcher, Std430GPURayMarcher},
    Settings,
};

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Hash,
    EnumString,
    EnumCount,
    EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum RayMarcherPreprocessorDirectives {
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
    EnableAOVs,
    EnableDirectionalLights,
    EnablePointLights,
    EnableAmbientOcclusion,
    EnableSoftShadows,
}

impl PreprocessorDirectives for RayMarcherPreprocessorDirectives {}

pub fn ray_march_shader(
    preprocessor_directives: &HashSet<RayMarcherPreprocessorDirectives>,
) -> String {
    process_shader_source(
        include_str!("./renderer/ray_march.wgsl"),
        preprocessor_directives,
    )
}

pub fn all_directives_for_ray_marcher() -> HashSet<RayMarcherPreprocessorDirectives> {
    HashSet::<RayMarcherPreprocessorDirectives>::from([
        RayMarcherPreprocessorDirectives::EnableAOVs,
    ])
}

pub fn all_directives_for_material() -> HashSet<RayMarcherPreprocessorDirectives> {
    HashSet::<RayMarcherPreprocessorDirectives>::from([
        RayMarcherPreprocessorDirectives::EnableDiffuseColourTexture,
        RayMarcherPreprocessorDirectives::EnableScatteringColourTexture,
        RayMarcherPreprocessorDirectives::EnableSpecularProbabilityTexture,
        RayMarcherPreprocessorDirectives::EnableSpecularRoughnessTexture,
        RayMarcherPreprocessorDirectives::EnableSpecularColourTexture,
        RayMarcherPreprocessorDirectives::EnableTransmissiveProbabilityTexture,
        RayMarcherPreprocessorDirectives::EnableTransmissiveRoughnessTexture,
        RayMarcherPreprocessorDirectives::EnableEmissiveColourTexture,
        RayMarcherPreprocessorDirectives::EnableExtinctionColourTexture,
        RayMarcherPreprocessorDirectives::EnableRefractiveIndexTexture,
        RayMarcherPreprocessorDirectives::EnableTrapColour,
        RayMarcherPreprocessorDirectives::EnableGrade,
        RayMarcherPreprocessorDirectives::EnableCheckerboard,
        RayMarcherPreprocessorDirectives::EnableNoise,
        RayMarcherPreprocessorDirectives::EnableSpecularMaterials,
        RayMarcherPreprocessorDirectives::EnableTransmissiveMaterials,
    ])
}

pub fn all_directives_for_primitive() -> HashSet<RayMarcherPreprocessorDirectives> {
    HashSet::<RayMarcherPreprocessorDirectives>::from([
        RayMarcherPreprocessorDirectives::EnableCappedCone,
        RayMarcherPreprocessorDirectives::EnableCappedTorus,
        RayMarcherPreprocessorDirectives::EnableCapsule,
        RayMarcherPreprocessorDirectives::EnableCone,
        RayMarcherPreprocessorDirectives::EnableCutSphere,
        RayMarcherPreprocessorDirectives::EnableCylinder,
        RayMarcherPreprocessorDirectives::EnableDeathStar,
        RayMarcherPreprocessorDirectives::EnableEllipsoid,
        RayMarcherPreprocessorDirectives::EnableHexagonalPrism,
        RayMarcherPreprocessorDirectives::EnableHollowSphere,
        RayMarcherPreprocessorDirectives::EnableInfiniteCone,
        RayMarcherPreprocessorDirectives::EnableInfiniteCylinder,
        RayMarcherPreprocessorDirectives::EnableLink,
        RayMarcherPreprocessorDirectives::EnableMandelbox,
        RayMarcherPreprocessorDirectives::EnableMandelbulb,
        RayMarcherPreprocessorDirectives::EnableOctahedron,
        RayMarcherPreprocessorDirectives::EnablePlane,
        RayMarcherPreprocessorDirectives::EnableRectangularPrism,
        RayMarcherPreprocessorDirectives::EnableRectangularPrismFrame,
        RayMarcherPreprocessorDirectives::EnableRhombus,
        RayMarcherPreprocessorDirectives::EnableRoundedCone,
        RayMarcherPreprocessorDirectives::EnableSolidAngle,
        RayMarcherPreprocessorDirectives::EnableTorus,
        RayMarcherPreprocessorDirectives::EnableTriangularPrism,
        RayMarcherPreprocessorDirectives::EnableChildInteractions,
        RayMarcherPreprocessorDirectives::EnablePrimitiveBlendSubtraction,
        RayMarcherPreprocessorDirectives::EnablePrimitiveBlendIntersection,
        RayMarcherPreprocessorDirectives::EnableInfiniteRepetition,
        RayMarcherPreprocessorDirectives::EnableFiniteRepetition,
        RayMarcherPreprocessorDirectives::EnableElongation,
        RayMarcherPreprocessorDirectives::EnableMirroring,
        RayMarcherPreprocessorDirectives::EnableHollowing,
        RayMarcherPreprocessorDirectives::EnablePhysicalLights,
    ])
}

pub fn all_directives_for_light() -> HashSet<RayMarcherPreprocessorDirectives> {
    HashSet::<RayMarcherPreprocessorDirectives>::from([
        RayMarcherPreprocessorDirectives::EnableDirectionalLights,
        RayMarcherPreprocessorDirectives::EnablePointLights,
        RayMarcherPreprocessorDirectives::EnableAmbientOcclusion,
        RayMarcherPreprocessorDirectives::EnableSoftShadows,
    ])
}

pub fn directives_for_ray_marcher(
    ray_marcher: &RayMarcher,
) -> HashSet<RayMarcherPreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();

    if ray_marcher.output_aov > AOVs::Beauty {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableAOVs);
    }

    preprocessor_directives
}

pub fn directives_for_primitive(
    primitive: &Primitive,
) -> HashSet<RayMarcherPreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();

    if primitive.num_descendants > 0
        && (primitive.blend_type > BlendType::Union || primitive.blend_strength > 0.)
    {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableChildInteractions);

        match primitive.blend_type {
            BlendType::Subtraction => {
                preprocessor_directives
                    .insert(RayMarcherPreprocessorDirectives::EnablePrimitiveBlendSubtraction);
            }
            BlendType::Intersection => {
                preprocessor_directives
                    .insert(RayMarcherPreprocessorDirectives::EnablePrimitiveBlendIntersection);
            }
            _ => {}
        }
    }

    match primitive.repetition {
        Repetition::Finite => {
            preprocessor_directives
                .insert(RayMarcherPreprocessorDirectives::EnableFiniteRepetition);
        }
        Repetition::Infinite => {
            preprocessor_directives
                .insert(RayMarcherPreprocessorDirectives::EnableInfiniteRepetition);
        }
        _ => {}
    }

    if primitive.elongate {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableElongation);
    }

    if primitive.mirror.any() {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableMirroring);
    }

    if primitive.hollow {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableHollowing);
    }

    if primitive.shape == Shapes::Sphere {
        return preprocessor_directives;
    }

    preprocessor_directives.insert(
        RayMarcherPreprocessorDirectives::from_str(
            &("Enable".to_owned() + &primitive.shape.to_string()),
        )
        .unwrap(),
    );

    preprocessor_directives
}

pub fn directives_for_procedural_texture(
    procedural_texture: &ProceduralTexture,
) -> HashSet<RayMarcherPreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();

    if procedural_texture.texture_type == ProceduralTextureType::Grade {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableGrade);
    } else if procedural_texture.texture_type == ProceduralTextureType::Checkerboard {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableCheckerboard);
    } else if procedural_texture.texture_type == ProceduralTextureType::FBMNoise
        || procedural_texture.texture_type == ProceduralTextureType::TurbulenceNoise
    {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableNoise);
    }

    preprocessor_directives
}

pub fn directives_for_material(material: &Material) -> HashSet<RayMarcherPreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();

    if material.diffuse_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableDiffuseColourTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.diffuse_colour_texture,
        ));
    }
    if material.specular_probability_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableSpecularProbabilityTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.specular_probability_texture,
        ));
    }
    if material.specular_roughness_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableSpecularRoughnessTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.specular_roughness_texture,
        ));
    }
    if material.specular_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableSpecularColourTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.specular_colour_texture,
        ));
    }
    if material.transmissive_probability_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableTransmissiveProbabilityTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.transmissive_probability_texture,
        ));
    }
    if material.transmissive_roughness_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableTransmissiveRoughnessTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.transmissive_roughness_texture,
        ));
    }
    if material.transmissive_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableExtinctionColourTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.transmissive_colour_texture,
        ));
    }
    if material.emissive_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableEmissiveColourTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.emissive_colour_texture,
        ));
    }
    if material.refractive_index_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableRefractiveIndexTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.refractive_index_texture,
        ));
    }
    if material.scattering_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableScatteringColourTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.scattering_colour_texture,
        ));
    }

    if material.transmissive_probability > 0. {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableSpecularMaterials);
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableTransmissiveMaterials);
    } else if material.specular_probability > 0. {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableSpecularMaterials);
    }

    if material.diffuse_colour_texture.use_trap_colour
        || material.specular_colour_texture.use_trap_colour
        || material.emissive_colour_texture.use_trap_colour
    {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableTrapColour);
    }

    if material.is_emissive() {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnablePhysicalLights);
    }

    preprocessor_directives
}

pub fn directives_for_light(light: &Light) -> HashSet<RayMarcherPreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();

    match light.light_type {
        Lights::Directional => {
            preprocessor_directives
                .insert(RayMarcherPreprocessorDirectives::EnableDirectionalLights);
        }
        Lights::Point => {
            preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnablePointLights);
        }
        Lights::AmbientOcclusion => {
            preprocessor_directives
                .insert(RayMarcherPreprocessorDirectives::EnableAmbientOcclusion);
        }
        _ => {}
    }

    if light.soften_shadows {
        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableSoftShadows);
    }

    preprocessor_directives
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RayMarcherCompilerSettings {
    pub enable_dynamic_recompilation_for_materials: bool,
    pub enable_dynamic_recompilation_for_primitives: bool,
    pub enable_dynamic_recompilation_for_ray_marcher: bool,
    pub enable_dynamic_recompilation_for_lights: bool,
}

impl Default for RayMarcherCompilerSettings {
    fn default() -> Self {
        Self {
            enable_dynamic_recompilation_for_materials: true,
            enable_dynamic_recompilation_for_primitives: true,
            enable_dynamic_recompilation_for_ray_marcher: true,
            enable_dynamic_recompilation_for_lights: true,
        }
    }
}

impl RayMarcherCompilerSettings {
    pub fn dynamic_recompilation_enabled(&self) -> bool {
        self.enable_dynamic_recompilation_for_primitives
            || self.enable_dynamic_recompilation_for_materials
            || self.enable_dynamic_recompilation_for_ray_marcher
            || self.enable_dynamic_recompilation_for_lights
    }
}

impl Settings for RayMarcherCompilerSettings {}

impl
    CompilerSettings<
        RayMarcherPreprocessorDirectives,
        RayMarcher,
        GPURayMarcher,
        Std430GPURayMarcher,
    > for RayMarcherCompilerSettings
{
    fn directives(&self, renderer: &RayMarcher) -> HashSet<RayMarcherPreprocessorDirectives> {
        let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();

        if !self.enable_dynamic_recompilation_for_ray_marcher {
            preprocessor_directives.extend(all_directives_for_ray_marcher());
        } else {
            preprocessor_directives.extend(directives_for_ray_marcher(renderer));
        }

        if !self.enable_dynamic_recompilation_for_primitives {
            preprocessor_directives.extend(all_directives_for_primitive());
        }

        if !self.enable_dynamic_recompilation_for_materials {
            preprocessor_directives.extend(all_directives_for_material());
        } else {
            preprocessor_directives.extend(directives_for_material(&renderer.scene.atmosphere));
        }

        if !self.enable_dynamic_recompilation_for_lights {
            preprocessor_directives.extend(all_directives_for_light());
        } else {
            for light in &renderer.scene.lights {
                preprocessor_directives.extend(directives_for_light(&light));
            }
        }

        if self.enable_dynamic_recompilation_for_primitives
            || self.enable_dynamic_recompilation_for_materials
        {
            for primitive in &renderer.scene.primitives {
                if self.enable_dynamic_recompilation_for_materials {
                    preprocessor_directives.extend(directives_for_material(&primitive.material));
                }
                if self.enable_dynamic_recompilation_for_primitives {
                    preprocessor_directives.extend(directives_for_primitive(&primitive));
                }
            }
        }

        preprocessor_directives
    }
}
