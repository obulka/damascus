// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashSet, str::FromStr};

use strum::{EnumCount, EnumIter, EnumString};

use super::{
    geometry::{BlendType, Primitive, Repetition, Shapes},
    lights::{Light, Lights},
    materials::{Material, ProceduralTexture, ProceduralTextureType},
    renderers::ray_marcher::{AOVs, RayMarcher},
};

#[derive(Debug, EnumString)]
enum Includes {
    AOVs,
    Camera,
    Material,
    Lights,
    Math,
    Normals,
    Primitive,
    PrimitiveModifiers,
    PrimitiveSDFs,
    ProceduralTexture,
    Random,
    Ray,
    RenderParameters,
    SceneSDFs,
    VertexShader,
}

#[derive(Debug, EnumString, EnumCount, EnumIter, Eq, Hash, PartialEq)]
pub enum PreprocessorDirectives {
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

impl Includes {
    fn source(&self) -> &str {
        match *self {
            Self::AOVs => include_str!("./renderer/aovs.wgsl"),
            Self::Camera => include_str!("./geometry/camera.wgsl"),
            Self::Lights => include_str!("./lights.wgsl"),
            Self::Material => include_str!("./materials/material.wgsl"),
            Self::Math => include_str!("./utils/math.wgsl"),
            Self::Normals => include_str!("./geometry/normals.wgsl"),
            Self::Primitive => include_str!("./geometry/primitive.wgsl"),
            Self::PrimitiveModifiers => include_str!("./geometry/modifiers.wgsl"),
            Self::PrimitiveSDFs => include_str!("./geometry/primitive_sdfs.wgsl"),
            Self::ProceduralTexture => include_str!("./materials/procedural_texture.wgsl"),
            Self::Random => include_str!("./utils/random.wgsl"),
            Self::Ray => include_str!("./geometry/ray.wgsl"),
            Self::RenderParameters => include_str!("./renderer/render_parameters.wgsl"),
            Self::SceneSDFs => include_str!("./geometry/scene_sdfs.wgsl"),
            Self::VertexShader => include_str!("./renderer/vertex_shader.wgsl"),
        }
    }
}

fn preprocess_directives(
    shader_source: Vec<String>,
    preprocessor_directives: &HashSet<PreprocessorDirectives>,
) -> Vec<String> {
    // Handle ifdef preprocessor macro
    let mut branch_stack = Vec::<(bool, bool)>::new();
    shader_source
        .into_iter()
        .filter(|line| {
            let hit_ifdef: bool = line.trim().starts_with("#ifdef");
            let hit_else_ifdef: bool = line.trim().starts_with("#elifdef");
            let hit_else: bool = line.trim().starts_with("#else");
            let hit_endif: bool = line.trim().starts_with("#endif");

            if hit_endif {
                branch_stack.pop();
                return false;
            }

            if let Some((branch_taken, current_branch_taken)) = branch_stack.last_mut() {
                if hit_ifdef {
                    if *current_branch_taken {
                        // If we are currently in a branch and have taken it
                        // and we hit another branch decide whether or not to
                        // take it, push it to the stack and carry on
                        let ifdef_directive = PreprocessorDirectives::from_str(
                            line.trim().trim_start_matches("#ifdef").trim(),
                        )
                        .unwrap();
                        let take_branch: bool = preprocessor_directives.contains(&ifdef_directive);
                        branch_stack.push((take_branch, take_branch));
                    } else {
                        // If we are not in the current branch we want
                        // to skip further directives until the next endif
                        branch_stack.push((true, false));
                    }
                    return false;
                } else if hit_else_ifdef {
                    if !*branch_taken {
                        // If we are in a branch and hit an else ifdef, and
                        // we have not yet taken a branch of the current
                        // conditional, check if we want to take this branch
                        let else_ifdef_directive = PreprocessorDirectives::from_str(
                            line.trim().trim_start_matches("#elifdef").trim(),
                        )
                        .unwrap();
                        *current_branch_taken =
                            preprocessor_directives.contains(&else_ifdef_directive);
                        *branch_taken |= *current_branch_taken;
                    } else if *current_branch_taken {
                        *current_branch_taken = false;
                    }
                    return false;
                } else if hit_else {
                    // If we have not yet taken a branch and hit an else
                    // then we take it
                    *current_branch_taken = !*branch_taken;
                    *branch_taken |= *current_branch_taken;

                    return false;
                } else {
                    // If we have not hit a new directive keep the line
                    // if we are in the current branch
                    return *current_branch_taken;
                }
            } else if hit_ifdef {
                // If we are not currently in a branch and we hit a branch
                // decide whether or not to take the branch, push it to the
                // stack and carry on
                let ifdef_directive = PreprocessorDirectives::from_str(
                    line.trim().trim_start_matches("#ifdef").trim(),
                )
                .unwrap();
                let take_branch: bool = preprocessor_directives.contains(&ifdef_directive);
                branch_stack.push((take_branch, take_branch));

                return false;
            }

            // If we are not in a branch and have not hit a directive
            // then keep this line
            true
        })
        .collect::<Vec<String>>()
}

pub fn ray_march_shader(preprocessor_directives: &HashSet<PreprocessorDirectives>) -> String {
    let mut shader_source = Vec::<String>::new();

    // println!(
    //     "compiling with the following directives: {:?}",
    //     preprocessor_directives
    // );

    // Read shader source and replace includes with shader source files.
    for line in include_str!("./renderer/ray_march.wgsl").split("\n") {
        if line.trim().starts_with("#include") {
            for line in Includes::from_str(line.trim().trim_start_matches("#include").trim())
                .unwrap()
                .source()
                .split("\n")
            {
                shader_source.push(line.to_string());
            }
        } else if line.trim().starts_with("//") || line.trim() == "" {
            continue;
        } else {
            shader_source.push(line.to_string());
        }
    }

    preprocess_directives(shader_source, preprocessor_directives).join("\n")
}

pub fn all_directives_for_ray_marcher() -> HashSet<PreprocessorDirectives> {
    HashSet::<PreprocessorDirectives>::from([PreprocessorDirectives::EnableAOVs])
}

pub fn all_directives_for_material() -> HashSet<PreprocessorDirectives> {
    HashSet::<PreprocessorDirectives>::from([
        PreprocessorDirectives::EnableDiffuseColourTexture,
        PreprocessorDirectives::EnableScatteringColourTexture,
        PreprocessorDirectives::EnableSpecularProbabilityTexture,
        PreprocessorDirectives::EnableSpecularRoughnessTexture,
        PreprocessorDirectives::EnableSpecularColourTexture,
        PreprocessorDirectives::EnableTransmissiveProbabilityTexture,
        PreprocessorDirectives::EnableTransmissiveRoughnessTexture,
        PreprocessorDirectives::EnableEmissiveColourTexture,
        PreprocessorDirectives::EnableExtinctionColourTexture,
        PreprocessorDirectives::EnableRefractiveIndexTexture,
        PreprocessorDirectives::EnableTrapColour,
        PreprocessorDirectives::EnableGrade,
        PreprocessorDirectives::EnableCheckerboard,
        PreprocessorDirectives::EnableNoise,
        PreprocessorDirectives::EnableSpecularMaterials,
        PreprocessorDirectives::EnableTransmissiveMaterials,
    ])
}

pub fn all_directives_for_primitive() -> HashSet<PreprocessorDirectives> {
    HashSet::<PreprocessorDirectives>::from([
        PreprocessorDirectives::EnableCappedCone,
        PreprocessorDirectives::EnableCappedTorus,
        PreprocessorDirectives::EnableCapsule,
        PreprocessorDirectives::EnableCone,
        PreprocessorDirectives::EnableCutSphere,
        PreprocessorDirectives::EnableCylinder,
        PreprocessorDirectives::EnableDeathStar,
        PreprocessorDirectives::EnableEllipsoid,
        PreprocessorDirectives::EnableHexagonalPrism,
        PreprocessorDirectives::EnableHollowSphere,
        PreprocessorDirectives::EnableInfiniteCone,
        PreprocessorDirectives::EnableInfiniteCylinder,
        PreprocessorDirectives::EnableLink,
        PreprocessorDirectives::EnableMandelbox,
        PreprocessorDirectives::EnableMandelbulb,
        PreprocessorDirectives::EnableOctahedron,
        PreprocessorDirectives::EnablePlane,
        PreprocessorDirectives::EnableRectangularPrism,
        PreprocessorDirectives::EnableRectangularPrismFrame,
        PreprocessorDirectives::EnableRhombus,
        PreprocessorDirectives::EnableRoundedCone,
        PreprocessorDirectives::EnableSolidAngle,
        PreprocessorDirectives::EnableTorus,
        PreprocessorDirectives::EnableTriangularPrism,
        PreprocessorDirectives::EnableChildInteractions,
        PreprocessorDirectives::EnablePrimitiveBlendSubtraction,
        PreprocessorDirectives::EnablePrimitiveBlendIntersection,
        PreprocessorDirectives::EnableInfiniteRepetition,
        PreprocessorDirectives::EnableFiniteRepetition,
        PreprocessorDirectives::EnableElongation,
        PreprocessorDirectives::EnableMirroring,
        PreprocessorDirectives::EnableHollowing,
        PreprocessorDirectives::EnablePhysicalLights,
    ])
}

pub fn all_directives_for_light() -> HashSet<PreprocessorDirectives> {
    HashSet::<PreprocessorDirectives>::from([
        PreprocessorDirectives::EnableDirectionalLights,
        PreprocessorDirectives::EnablePointLights,
        PreprocessorDirectives::EnableAmbientOcclusion,
        PreprocessorDirectives::EnableSoftShadows,
    ])
}

pub fn directives_for_ray_marcher(ray_marcher: &RayMarcher) -> HashSet<PreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();

    if ray_marcher.output_aov > AOVs::Beauty {
        preprocessor_directives.insert(PreprocessorDirectives::EnableAOVs);
    }

    preprocessor_directives
}

pub fn directives_for_primitive(primitive: &Primitive) -> HashSet<PreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();

    if primitive.num_descendants > 0
        && (primitive.blend_type > BlendType::Union || primitive.blend_strength > 0.)
    {
        preprocessor_directives.insert(PreprocessorDirectives::EnableChildInteractions);

        match primitive.blend_type {
            BlendType::Subtraction => {
                preprocessor_directives
                    .insert(PreprocessorDirectives::EnablePrimitiveBlendSubtraction);
            }
            BlendType::Intersection => {
                preprocessor_directives
                    .insert(PreprocessorDirectives::EnablePrimitiveBlendIntersection);
            }
            _ => {}
        }
    }

    match primitive.repetition {
        Repetition::Finite => {
            preprocessor_directives.insert(PreprocessorDirectives::EnableFiniteRepetition);
        }
        Repetition::Infinite => {
            preprocessor_directives.insert(PreprocessorDirectives::EnableInfiniteRepetition);
        }
        _ => {}
    }

    if primitive.elongate {
        preprocessor_directives.insert(PreprocessorDirectives::EnableElongation);
    }

    if primitive.mirror.any() {
        preprocessor_directives.insert(PreprocessorDirectives::EnableMirroring);
    }

    if primitive.hollow {
        preprocessor_directives.insert(PreprocessorDirectives::EnableHollowing);
    }

    if primitive.shape == Shapes::Sphere {
        return preprocessor_directives;
    }

    preprocessor_directives.insert(
        PreprocessorDirectives::from_str(&("Enable".to_owned() + &primitive.shape.to_string()))
            .unwrap(),
    );

    preprocessor_directives
}

pub fn directives_for_procedural_texture(
    procedural_texture: &ProceduralTexture,
) -> HashSet<PreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();

    if procedural_texture.texture_type == ProceduralTextureType::Grade {
        preprocessor_directives.insert(PreprocessorDirectives::EnableGrade);
    } else if procedural_texture.texture_type == ProceduralTextureType::Checkerboard {
        preprocessor_directives.insert(PreprocessorDirectives::EnableCheckerboard);
    } else if procedural_texture.texture_type == ProceduralTextureType::FBMNoise
        || procedural_texture.texture_type == ProceduralTextureType::TurbulenceNoise
    {
        preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
    }

    preprocessor_directives
}

pub fn directives_for_material(material: &Material) -> HashSet<PreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();

    if material.diffuse_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableDiffuseColourTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.diffuse_colour_texture,
        ));
    }
    if material.specular_probability_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularProbabilityTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.specular_probability_texture,
        ));
    }
    if material.specular_roughness_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularRoughnessTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.specular_roughness_texture,
        ));
    }
    if material.specular_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularColourTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.specular_colour_texture,
        ));
    }
    if material.transmissive_probability_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives
            .insert(PreprocessorDirectives::EnableTransmissiveProbabilityTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.transmissive_probability_texture,
        ));
    }
    if material.transmissive_roughness_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableTransmissiveRoughnessTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.transmissive_roughness_texture,
        ));
    }
    if material.transmissive_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableExtinctionColourTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.transmissive_colour_texture,
        ));
    }
    if material.emissive_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableEmissiveColourTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.emissive_colour_texture,
        ));
    }
    if material.refractive_index_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableRefractiveIndexTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.refractive_index_texture,
        ));
    }
    if material.scattering_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableScatteringColourTexture);
        preprocessor_directives.extend(directives_for_procedural_texture(
            &material.scattering_colour_texture,
        ));
    }

    if material.transmissive_probability > 0. {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularMaterials);
        preprocessor_directives.insert(PreprocessorDirectives::EnableTransmissiveMaterials);
    } else if material.specular_probability > 0. {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularMaterials);
    }

    if material.diffuse_colour_texture.use_trap_colour
        || material.specular_colour_texture.use_trap_colour
        || material.emissive_colour_texture.use_trap_colour
    {
        preprocessor_directives.insert(PreprocessorDirectives::EnableTrapColour);
    }

    if material.is_emissive() {
        preprocessor_directives.insert(PreprocessorDirectives::EnablePhysicalLights);
    }

    preprocessor_directives
}

pub fn directives_for_light(light: &Light) -> HashSet<PreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();

    match light.light_type {
        Lights::Directional => {
            preprocessor_directives.insert(PreprocessorDirectives::EnableDirectionalLights);
        }
        Lights::Point => {
            preprocessor_directives.insert(PreprocessorDirectives::EnablePointLights);
        }
        Lights::AmbientOcclusion => {
            preprocessor_directives.insert(PreprocessorDirectives::EnableAmbientOcclusion);
        }
        _ => {}
    }

    if light.soften_shadows {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSoftShadows);
    }

    preprocessor_directives
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_directives_covered() {
        let mut preprocessor_directives: HashSet<PreprocessorDirectives> =
            all_directives_for_material();
        preprocessor_directives.extend(all_directives_for_primitive());
        preprocessor_directives.extend(all_directives_for_ray_marcher());
        preprocessor_directives.extend(all_directives_for_light());
        assert_eq!(preprocessor_directives.len(), PreprocessorDirectives::COUNT);
    }

    #[test]
    fn test_ifdef_preprocessor_directives() {
        let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();
        preprocessor_directives.insert(PreprocessorDirectives::EnableDiffuseColourTexture);

        let source = vec![
            "keep;",
            "#ifdef EnableDiffuseColourTexture",
            "keep;",
            "keep;",
            "#endif",
            "keep;",
            "#ifdef EnableSpecularColourTexture",
            "remove;",
            "#endif",
            "keep;",
        ];
        let result = preprocess_directives(
            source.into_iter().map(|line| line.to_string()).collect(),
            &preprocessor_directives,
        );
        assert_eq!(result.len(), 5);

        let result_string: String = result.join("\n");
        assert!(!result_string.contains("remove"));
        assert!(!result_string.contains("#"));
    }

    #[test]
    fn test_else_preprocessor_directives() {
        let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();
        preprocessor_directives.insert(PreprocessorDirectives::EnableDiffuseColourTexture);

        let source = vec![
            "keep;",
            "#ifdef EnableDiffuseColourTexture",
            "keep;",
            "keep;",
            "#else",
            "remove;",
            "#endif",
            "keep;",
            "#ifdef EnableSpecularColourTexture",
            "remove;",
            "#else",
            "keep;",
            "#endif",
            "keep;",
        ];
        let result = preprocess_directives(
            source.into_iter().map(|line| line.to_string()).collect(),
            &preprocessor_directives,
        );
        assert_eq!(result.len(), 6);

        let result_string: String = result.join("\n");
        assert!(!result_string.contains("remove"));
        assert!(!result_string.contains("#"));
    }

    #[test]
    fn test_elifdef_preprocessor_directives() {
        let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();
        preprocessor_directives.insert(PreprocessorDirectives::EnableDiffuseColourTexture);
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularColourTexture);

        let mut source = vec![
            "keep;",
            "#ifdef EnableDiffuseColourTexture",
            "keep;",
            "#elifdef EnableSpecularColourTexture",
            "remove;",
            "#elifdef EnableRefractiveIndexTexture",
            "remove;",
            "#else",
            "remove;",
            "#endif",
            "keep;",
        ];
        let result = preprocess_directives(
            source.into_iter().map(|line| line.to_string()).collect(),
            &preprocessor_directives,
        );
        assert_eq!(result.len(), 3);

        let result_string: String = result.join("\n");
        assert!(!result_string.contains("remove"));
        assert!(!result_string.contains("#"));

        source = vec![
            "keep;",
            "#ifdef EnableDiffuseColourTexture",
            "remove;",
            "#elifdef EnableSpecularColourTexture",
            "keep;",
            "#elifdef EnableRefractiveIndexTexture",
            "remove;",
            "#else",
            "remove;",
            "#endif",
            "keep;",
        ];
        preprocessor_directives.remove(&PreprocessorDirectives::EnableDiffuseColourTexture);
        let result = preprocess_directives(
            source.into_iter().map(|line| line.to_string()).collect(),
            &preprocessor_directives,
        );
        assert_eq!(result.len(), 3);

        let result_string: String = result.join("\n");
        assert!(!result_string.contains("remove"));
        assert!(!result_string.contains("#"));

        source = vec![
            "keep;",
            "#ifdef EnableDiffuseColourTexture",
            "remove;",
            "#elifdef EnableSpecularColourTexture",
            "remove;",
            "#elifdef EnableRefractiveIndexTexture",
            "remove;",
            "#else",
            "keep;",
            "#endif",
            "keep;",
        ];
        preprocessor_directives.remove(&PreprocessorDirectives::EnableSpecularColourTexture);
        let result = preprocess_directives(
            source.into_iter().map(|line| line.to_string()).collect(),
            &preprocessor_directives,
        );
        assert_eq!(result.len(), 3);

        let result_string: String = result.join("\n");
        assert!(!result_string.contains("remove"));
        assert!(!result_string.contains("#"));
    }

    #[test]
    fn test_nested_preprocessor_directives() {
        let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();
        preprocessor_directives.insert(PreprocessorDirectives::EnableDiffuseColourTexture);
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularColourTexture);

        let mut source = vec![
            "keep;",
            "#ifdef EnableDiffuseColourTexture",
            "keep;",
            "#ifdef EnableRefractiveIndexTexture",
            "remove;",
            "#elifdef EnableSpecularColourTexture",
            "keep;",
            "#else",
            "remove;",
            "#endif",
            "#elifdef EnableSpecularColourTexture",
            "remove;",
            "#elifdef EnableRefractiveIndexTexture",
            "remove;",
            "#else",
            "remove;",
            "#endif",
            "keep;",
        ];
        let result = preprocess_directives(
            source.into_iter().map(|line| line.to_string()).collect(),
            &preprocessor_directives,
        );
        assert_eq!(result.len(), 4);

        let result_string: String = result.join("\n");
        assert!(!result_string.contains("remove"));
        assert!(!result_string.contains("#"));

        source = vec![
            "keep;",
            "#ifdef EnableDiffuseColourTexture",
            "remove;",
            "#ifdef EnableRefractiveIndexTexture",
            "remove;",
            "#elifdef EnableSpecularColourTexture",
            "remove;",
            "#else",
            "remove;",
            "#endif",
            "#elifdef EnableSpecularColourTexture",
            "keep;",
            "#elifdef EnableRefractiveIndexTexture",
            "remove;",
            "#else",
            "remove;",
            "#endif",
            "keep;",
        ];
        preprocessor_directives.remove(&PreprocessorDirectives::EnableDiffuseColourTexture);
        let result = preprocess_directives(
            source.into_iter().map(|line| line.to_string()).collect(),
            &preprocessor_directives,
        );
        assert_eq!(result.len(), 3);

        let result_string: String = result.join("\n");
        assert!(!result_string.contains("remove"));
        assert!(!result_string.contains("#"));
    }

    #[test]
    fn test_transform_directives() {
        let source = vec![
            "fn transform_position(",
            "    position: vec3f,",
            "    primitive: ptr<function, Primitive>,",
            ") -> vec3f {",
            "    // Perform finite or infinite repetition if enabled",
            "#ifdef EnableFiniteRepetition",
            "    var transformed_position: vec3f = select(",
            "#else",
            "    var transformed_position: vec3f =",
            "#endif",
            "#ifdef EnableInfiniteRepetition",
            "        select(",
            "            position,",
            "            mirrored_infinite_repetition(",
            "                position,",
            "                primitive,",
            "            ),",
            "            bool((*primitive).modifiers & INFINITE_REPETITION),",
            "#ifdef EnableFiniteRepetition",
            "        ),",
            "#else",
            "        );",
            "#endif",
            "#elifdef EnableFiniteRepetition",
            "        position,",
            "#else",
            "        position;",
            "#endif",
            "#ifdef EnableFiniteRepetition",
            "        mirrored_finite_repetition(",
            "            position,",
            "            primitive,",
            "        ),",
            "        bool((*primitive).modifiers & FINITE_REPETITION),",
            "    );",
            "#endif",
            "    // Perform elongation if enabled",
            "    transformed_position -= select(",
            "        vec3(0.),",
            "        clamp(",
            "            transformed_position,",
            "            -(*primitive).elongation,",
            "            (*primitive).elongation,",
            "        ),",
            "        bool((*primitive).modifiers & ELONGATE),",
            "    );",
            "    // Perform mirroring if enabled",
            "    return select(",
            "        transformed_position,",
            "        abs(transformed_position),",
            "        vec3<bool>(",
            "            bool((*primitive).modifiers & MIRROR_X),",
            "            bool((*primitive).modifiers & MIRROR_Y),",
            "            bool((*primitive).modifiers & MIRROR_Z),",
            "        ),",
            "    );",
            "}",
        ];

        let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();

        let mut expected_result = vec![
            "fn transform_position(",
            "    position: vec3f,",
            "    primitive: ptr<function, Primitive>,",
            ") -> vec3f {",
            "    // Perform finite or infinite repetition if enabled",
            "    var transformed_position: vec3f =",
            "        position;",
            "    // Perform elongation if enabled",
            "    transformed_position -= select(",
            "        vec3(0.),",
            "        clamp(",
            "            transformed_position,",
            "            -(*primitive).elongation,",
            "            (*primitive).elongation,",
            "        ),",
            "        bool((*primitive).modifiers & ELONGATE),",
            "    );",
            "    // Perform mirroring if enabled",
            "    return select(",
            "        transformed_position,",
            "        abs(transformed_position),",
            "        vec3<bool>(",
            "            bool((*primitive).modifiers & MIRROR_X),",
            "            bool((*primitive).modifiers & MIRROR_Y),",
            "            bool((*primitive).modifiers & MIRROR_Z),",
            "        ),",
            "    );",
            "}",
        ];

        let result = preprocess_directives(
            source.iter().map(|line| line.to_string()).collect(),
            &preprocessor_directives,
        );

        let result_string: String = result.join("\n");
        let expected_result_string: String = expected_result.join("\n");

        assert_eq!(result_string, expected_result_string);

        preprocessor_directives.insert(PreprocessorDirectives::EnableFiniteRepetition);

        expected_result = vec![
            "fn transform_position(",
            "    position: vec3f,",
            "    primitive: ptr<function, Primitive>,",
            ") -> vec3f {",
            "    // Perform finite or infinite repetition if enabled",
            "    var transformed_position: vec3f = select(",
            "        position,",
            "        mirrored_finite_repetition(",
            "            position,",
            "            primitive,",
            "        ),",
            "        bool((*primitive).modifiers & FINITE_REPETITION),",
            "    );",
            "    // Perform elongation if enabled",
            "    transformed_position -= select(",
            "        vec3(0.),",
            "        clamp(",
            "            transformed_position,",
            "            -(*primitive).elongation,",
            "            (*primitive).elongation,",
            "        ),",
            "        bool((*primitive).modifiers & ELONGATE),",
            "    );",
            "    // Perform mirroring if enabled",
            "    return select(",
            "        transformed_position,",
            "        abs(transformed_position),",
            "        vec3<bool>(",
            "            bool((*primitive).modifiers & MIRROR_X),",
            "            bool((*primitive).modifiers & MIRROR_Y),",
            "            bool((*primitive).modifiers & MIRROR_Z),",
            "        ),",
            "    );",
            "}",
        ];

        let result = preprocess_directives(
            source.iter().map(|line| line.to_string()).collect(),
            &preprocessor_directives,
        );

        let result_string: String = result.join("\n");
        let expected_result_string: String = expected_result.join("\n");

        assert_eq!(result_string, expected_result_string);

        preprocessor_directives.clear();

        preprocessor_directives.insert(PreprocessorDirectives::EnableInfiniteRepetition);

        expected_result = vec![
            "fn transform_position(",
            "    position: vec3f,",
            "    primitive: ptr<function, Primitive>,",
            ") -> vec3f {",
            "    // Perform finite or infinite repetition if enabled",
            "    var transformed_position: vec3f =",
            "        select(",
            "            position,",
            "            mirrored_infinite_repetition(",
            "                position,",
            "                primitive,",
            "            ),",
            "            bool((*primitive).modifiers & INFINITE_REPETITION),",
            "        );",
            "    // Perform elongation if enabled",
            "    transformed_position -= select(",
            "        vec3(0.),",
            "        clamp(",
            "            transformed_position,",
            "            -(*primitive).elongation,",
            "            (*primitive).elongation,",
            "        ),",
            "        bool((*primitive).modifiers & ELONGATE),",
            "    );",
            "    // Perform mirroring if enabled",
            "    return select(",
            "        transformed_position,",
            "        abs(transformed_position),",
            "        vec3<bool>(",
            "            bool((*primitive).modifiers & MIRROR_X),",
            "            bool((*primitive).modifiers & MIRROR_Y),",
            "            bool((*primitive).modifiers & MIRROR_Z),",
            "        ),",
            "    );",
            "}",
        ];

        let result = preprocess_directives(
            source.iter().map(|line| line.to_string()).collect(),
            &preprocessor_directives,
        );

        let result_string: String = result.join("\n");
        let expected_result_string: String = expected_result.join("\n");

        assert_eq!(result_string, expected_result_string);

        preprocessor_directives.insert(PreprocessorDirectives::EnableFiniteRepetition);

        expected_result = vec![
            "fn transform_position(",
            "    position: vec3f,",
            "    primitive: ptr<function, Primitive>,",
            ") -> vec3f {",
            "    // Perform finite or infinite repetition if enabled",
            "    var transformed_position: vec3f = select(",
            "        select(",
            "            position,",
            "            mirrored_infinite_repetition(",
            "                position,",
            "                primitive,",
            "            ),",
            "            bool((*primitive).modifiers & INFINITE_REPETITION),",
            "        ),",
            "        mirrored_finite_repetition(",
            "            position,",
            "            primitive,",
            "        ),",
            "        bool((*primitive).modifiers & FINITE_REPETITION),",
            "    );",
            "    // Perform elongation if enabled",
            "    transformed_position -= select(",
            "        vec3(0.),",
            "        clamp(",
            "            transformed_position,",
            "            -(*primitive).elongation,",
            "            (*primitive).elongation,",
            "        ),",
            "        bool((*primitive).modifiers & ELONGATE),",
            "    );",
            "    // Perform mirroring if enabled",
            "    return select(",
            "        transformed_position,",
            "        abs(transformed_position),",
            "        vec3<bool>(",
            "            bool((*primitive).modifiers & MIRROR_X),",
            "            bool((*primitive).modifiers & MIRROR_Y),",
            "            bool((*primitive).modifiers & MIRROR_Z),",
            "        ),",
            "    );",
            "}",
        ];

        let result = preprocess_directives(
            source.iter().map(|line| line.to_string()).collect(),
            &preprocessor_directives,
        );

        let result_string: String = result.join("\n");
        let expected_result_string: String = expected_result.join("\n");

        assert_eq!(result_string, expected_result_string);
    }
}
