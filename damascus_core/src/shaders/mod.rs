// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use std::{collections::HashSet, str::FromStr};

use strum::{EnumCount, EnumString};

use super::{
    geometry::{BlendType, Primitive, Shapes},
    materials::{Material, ProceduralTextureType},
    renderers::{AOVs, RayMarcher},
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

#[derive(Debug, EnumString, EnumCount, Eq, Hash, PartialEq)]
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
    EnableSpecularMaterials,
    EnableTransmissiveMaterials,
    EnableAOVs,
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
    let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();
    preprocessor_directives.insert(PreprocessorDirectives::EnableAOVs);

    preprocessor_directives
}

pub fn all_directives_for_material() -> HashSet<PreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();
    preprocessor_directives.insert(PreprocessorDirectives::EnableDiffuseColourTexture);
    preprocessor_directives.insert(PreprocessorDirectives::EnableScatteringColourTexture);
    preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularProbabilityTexture);
    preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularRoughnessTexture);
    preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularColourTexture);
    preprocessor_directives.insert(PreprocessorDirectives::EnableTransmissiveProbabilityTexture);
    preprocessor_directives.insert(PreprocessorDirectives::EnableTransmissiveRoughnessTexture);
    preprocessor_directives.insert(PreprocessorDirectives::EnableEmissiveColourTexture);
    preprocessor_directives.insert(PreprocessorDirectives::EnableExtinctionColourTexture);
    preprocessor_directives.insert(PreprocessorDirectives::EnableRefractiveIndexTexture);
    preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
    preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularMaterials);
    preprocessor_directives.insert(PreprocessorDirectives::EnableTransmissiveMaterials);

    preprocessor_directives
}

pub fn all_directives_for_primitive() -> HashSet<PreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();
    preprocessor_directives.insert(PreprocessorDirectives::EnableCappedCone);
    preprocessor_directives.insert(PreprocessorDirectives::EnableCappedTorus);
    preprocessor_directives.insert(PreprocessorDirectives::EnableCapsule);
    preprocessor_directives.insert(PreprocessorDirectives::EnableCone);
    preprocessor_directives.insert(PreprocessorDirectives::EnableCutSphere);
    preprocessor_directives.insert(PreprocessorDirectives::EnableCylinder);
    preprocessor_directives.insert(PreprocessorDirectives::EnableDeathStar);
    preprocessor_directives.insert(PreprocessorDirectives::EnableEllipsoid);
    preprocessor_directives.insert(PreprocessorDirectives::EnableHexagonalPrism);
    preprocessor_directives.insert(PreprocessorDirectives::EnableHollowSphere);
    preprocessor_directives.insert(PreprocessorDirectives::EnableInfiniteCone);
    preprocessor_directives.insert(PreprocessorDirectives::EnableInfiniteCylinder);
    preprocessor_directives.insert(PreprocessorDirectives::EnableLink);
    preprocessor_directives.insert(PreprocessorDirectives::EnableMandelbox);
    preprocessor_directives.insert(PreprocessorDirectives::EnableMandelbulb);
    preprocessor_directives.insert(PreprocessorDirectives::EnableOctahedron);
    preprocessor_directives.insert(PreprocessorDirectives::EnablePlane);
    preprocessor_directives.insert(PreprocessorDirectives::EnableRectangularPrism);
    preprocessor_directives.insert(PreprocessorDirectives::EnableRectangularPrismFrame);
    preprocessor_directives.insert(PreprocessorDirectives::EnableRhombus);
    preprocessor_directives.insert(PreprocessorDirectives::EnableRoundedCone);
    preprocessor_directives.insert(PreprocessorDirectives::EnableSolidAngle);
    preprocessor_directives.insert(PreprocessorDirectives::EnableTorus);
    preprocessor_directives.insert(PreprocessorDirectives::EnableTriangularPrism);
    preprocessor_directives.insert(PreprocessorDirectives::EnableChildInteractions);
    preprocessor_directives.insert(PreprocessorDirectives::EnablePrimitiveBlendSubtraction);
    preprocessor_directives.insert(PreprocessorDirectives::EnablePrimitiveBlendIntersection);

    preprocessor_directives
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

    if primitive.shape == Shapes::Sphere {
        return preprocessor_directives;
    }

    preprocessor_directives.insert(
        PreprocessorDirectives::from_str(&("Enable".to_owned() + &primitive.shape.to_string()))
            .unwrap(),
    );

    preprocessor_directives
}

pub fn directives_for_material(material: &Material) -> HashSet<PreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();
    if material.diffuse_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableDiffuseColourTexture);
        if material.diffuse_colour_texture.texture_type >= ProceduralTextureType::FBMNoise {
            preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
        }
    }
    if material.specular_probability_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularMaterials);
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularProbabilityTexture);
        if material.specular_probability_texture.texture_type >= ProceduralTextureType::FBMNoise {
            preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
        }
    }
    if material.specular_roughness_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularRoughnessTexture);
        if material.specular_roughness_texture.texture_type >= ProceduralTextureType::FBMNoise {
            preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
        }
    }
    if material.specular_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularColourTexture);
        if material.specular_colour_texture.texture_type >= ProceduralTextureType::FBMNoise {
            preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
        }
    }
    if material.transmissive_probability_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularMaterials);
        preprocessor_directives.insert(PreprocessorDirectives::EnableTransmissiveMaterials);
        preprocessor_directives
            .insert(PreprocessorDirectives::EnableTransmissiveProbabilityTexture);
        if material.transmissive_probability_texture.texture_type >= ProceduralTextureType::FBMNoise
        {
            preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
        }
    }
    if material.transmissive_roughness_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableTransmissiveRoughnessTexture);
        if material.transmissive_roughness_texture.texture_type >= ProceduralTextureType::FBMNoise {
            preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
        }
    }
    if material.transmissive_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableExtinctionColourTexture);
        if material.transmissive_colour_texture.texture_type >= ProceduralTextureType::FBMNoise {
            preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
        }
    }
    if material.emissive_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableEmissiveColourTexture);
        if material.emissive_colour_texture.texture_type >= ProceduralTextureType::FBMNoise {
            preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
        }
    }
    if material.refractive_index_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableRefractiveIndexTexture);
        if material.refractive_index_texture.texture_type >= ProceduralTextureType::FBMNoise {
            preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
        }
    }
    if material.scattering_colour_texture.texture_type > ProceduralTextureType::None {
        preprocessor_directives.insert(PreprocessorDirectives::EnableScatteringColourTexture);
        if material.scattering_colour_texture.texture_type >= ProceduralTextureType::FBMNoise {
            preprocessor_directives.insert(PreprocessorDirectives::EnableNoise);
        }
    }

    if material.transmissive_probability > 0. {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularMaterials);
        preprocessor_directives.insert(PreprocessorDirectives::EnableTransmissiveMaterials);
    } else if material.specular_probability > 0. {
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularMaterials);
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
        println!("{:?}", result);
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
        println!("{:?}", result);
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
}
