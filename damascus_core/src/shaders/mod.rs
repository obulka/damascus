// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use std::{collections::HashSet, str::FromStr};

use strum::EnumString;

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

#[derive(Debug, EnumString, Eq, Hash, PartialEq)]
pub enum PreprocessorDirectives {
    EnableDiffuseTexture,
    EnableSpecularProbabilityTexture,
    EnableSpecularRoughnessTexture,
    EnableSpecularTexture,
    EnableTransmissiveProbabilityTexture,
    EnableTransmissiveRoughnessTexture,
    EnableEmissiveColourTexture,
    EnableRefractiveIndexTexture,
}

impl Includes {
    fn source(&self) -> &str {
        match *self {
            Self::AOVs => include_str!("./renderer/aovs.wgsl"),
            Self::Camera => include_str!("./camera.wgsl"),
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
    println!("Compiling with directives: {:?}", preprocessor_directives);
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
        } else {
            shader_source.push(line.to_string());
        }
    }

    preprocess_directives(shader_source, preprocessor_directives).join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ifdef_preprocessor_directives() {
        let mut preprocessor_directives = HashSet::<PreprocessorDirectives>::new();
        preprocessor_directives.insert(PreprocessorDirectives::EnableDiffuseTexture);

        let source = vec![
            "keep;",
            "#ifdef EnableDiffuseTexture",
            "keep;",
            "keep;",
            "#endif",
            "keep;",
            "#ifdef EnableSpecularTexture",
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
        preprocessor_directives.insert(PreprocessorDirectives::EnableDiffuseTexture);

        let source = vec![
            "keep;",
            "#ifdef EnableDiffuseTexture",
            "keep;",
            "keep;",
            "#else",
            "remove;",
            "#endif",
            "keep;",
            "#ifdef EnableSpecularTexture",
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
        preprocessor_directives.insert(PreprocessorDirectives::EnableDiffuseTexture);
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularTexture);

        let mut source = vec![
            "keep;",
            "#ifdef EnableDiffuseTexture",
            "keep;",
            "#elifdef EnableSpecularTexture",
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
            "#ifdef EnableDiffuseTexture",
            "remove;",
            "#elifdef EnableSpecularTexture",
            "keep;",
            "#elifdef EnableRefractiveIndexTexture",
            "remove;",
            "#else",
            "remove;",
            "#endif",
            "keep;",
        ];
        preprocessor_directives.remove(&PreprocessorDirectives::EnableDiffuseTexture);
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
            "#ifdef EnableDiffuseTexture",
            "remove;",
            "#elifdef EnableSpecularTexture",
            "remove;",
            "#elifdef EnableRefractiveIndexTexture",
            "remove;",
            "#else",
            "keep;",
            "#endif",
            "keep;",
        ];
        preprocessor_directives.remove(&PreprocessorDirectives::EnableSpecularTexture);
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
        preprocessor_directives.insert(PreprocessorDirectives::EnableDiffuseTexture);
        preprocessor_directives.insert(PreprocessorDirectives::EnableSpecularTexture);

        let mut source = vec![
            "keep;",
            "#ifdef EnableDiffuseTexture",
            "keep;",
            "#ifdef EnableRefractiveIndexTexture",
            "remove;",
            "#elifdef EnableSpecularTexture",
            "keep;",
            "#else",
            "remove;",
            "#endif",
            "#elifdef EnableSpecularTexture",
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
            "#ifdef EnableDiffuseTexture",
            "remove;",
            "#ifdef EnableRefractiveIndexTexture",
            "remove;",
            "#elifdef EnableSpecularTexture",
            "remove;",
            "#else",
            "remove;",
            "#endif",
            "#elifdef EnableSpecularTexture",
            "keep;",
            "#elifdef EnableRefractiveIndexTexture",
            "remove;",
            "#else",
            "remove;",
            "#endif",
            "keep;",
        ];
        preprocessor_directives.remove(&PreprocessorDirectives::EnableDiffuseTexture);
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
