// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{borrow::Cow, collections::HashSet, fmt::Debug, hash::Hash, str::FromStr};

use strum::{EnumCount, EnumString, IntoEnumIterator};
use wgpu;

pub mod ray_marcher;
pub mod texture_viewer;

#[derive(Debug, EnumString)]
pub enum Includes {
    AOVs,
    Camera,
    TextureViewerConstants,
    TextureViewerRenderParameters,
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
    RayMarcherConstants,
    RayMarcherRenderParameters,
    SceneSDFs,
    Texture,
}

impl Includes {
    fn source(&self) -> &str {
        match *self {
            Self::AOVs => include_str!("./wgsl/pipelines/ray_marcher/aovs.wgsl"),
            Self::Camera => include_str!("./wgsl/geometry/camera.wgsl"),
            Self::TextureViewerConstants => {
                include_str!("./wgsl/pipelines/texture_viewer/constants.wgsl")
            }
            Self::TextureViewerRenderParameters => {
                include_str!("./wgsl/pipelines/texture_viewer/render_parameters.wgsl")
            }
            Self::Lights => include_str!("./wgsl/lights/lights.wgsl"),
            Self::Material => include_str!("./wgsl/materials/material.wgsl"),
            Self::Math => include_str!("./wgsl/utils/math.wgsl"),
            Self::Normals => include_str!("./wgsl/geometry/normals.wgsl"),
            Self::Primitive => include_str!("./wgsl/geometry/primitive.wgsl"),
            Self::PrimitiveModifiers => include_str!("./wgsl/geometry/modifiers.wgsl"),
            Self::PrimitiveSDFs => include_str!("./wgsl/geometry/primitive_sdfs.wgsl"),
            Self::ProceduralTexture => include_str!("./wgsl/materials/procedural_texture.wgsl"),
            Self::Random => include_str!("./wgsl/utils/random.wgsl"),
            Self::Ray => include_str!("./wgsl/geometry/ray.wgsl"),
            Self::RayMarcherConstants => {
                include_str!("./wgsl/pipelines/ray_marcher/constants.wgsl")
            }
            Self::RayMarcherRenderParameters => {
                include_str!("./wgsl/pipelines/ray_marcher/render_parameters.wgsl")
            }
            Self::SceneSDFs => include_str!("./wgsl/geometry/scene_sdfs.wgsl"),
            Self::Texture => include_str!("./wgsl/textures/texture.wgsl"),
        }
    }
}

pub trait PreprocessorDirectives:
    Clone
    + Debug
    + Eq
    + PartialEq
    + Hash
    + FromStr
    + EnumCount
    + IntoEnumIterator
    + serde::Serialize
    + for<'a> serde::Deserialize<'a>
{
}

pub trait ShaderSource<Directives: PreprocessorDirectives> {
    fn vertex_shader_raw(&self) -> String;

    fn fragment_shader_raw(&self) -> String;

    fn current_directives(&self) -> &HashSet<Directives>;

    fn current_directives_mut(&mut self) -> &mut HashSet<Directives>;

    fn dynamic_directives(&self) -> HashSet<Directives> {
        HashSet::<Directives>::new()
    }

    fn update_directives(&mut self) -> bool {
        let new_directives = self.directives();
        let current_directives = self.current_directives_mut();

        // Check if the directives have changed and store them if they have
        if new_directives == *current_directives {
            return false;
        }
        *current_directives = new_directives;
        true
    }

    fn vertex_shader(&self) -> wgpu::ShaderSource<'_> {
        wgpu::ShaderSource::Wgsl(Cow::Borrowed(&process_shader_source(
            self.vertex_shader_raw(),
            self.current_directives(),
        )))
    }

    fn fragment_shader(&self) -> wgpu::ShaderSource<'_> {
        wgpu::ShaderSource::Wgsl(Cow::Borrowed(&process_shader_source(
            self.fragment_shader_raw(),
            self.current_directives(),
        )))
    }

    fn dynamic_recompilation_enabled(&self) -> bool {
        true
    }

    fn all_directives(&self) -> HashSet<Directives> {
        Directives::iter().collect()
    }

    fn directives(&self) -> HashSet<Directives> {
        if self.dynamic_recompilation_enabled() {
            return self.dynamic_directives();
        }
        self.all_directives()
    }
}

pub fn preprocess_directives<Directives: PreprocessorDirectives>(
    shader_source: Vec<String>,
    preprocessor_directives: &HashSet<Directives>,
) -> Vec<String>
where
    <Directives as FromStr>::Err: Debug,
{
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
                        let ifdef_directive =
                            Directives::from_str(line.trim().trim_start_matches("#ifdef").trim())
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
                        let else_ifdef_directive =
                            Directives::from_str(line.trim().trim_start_matches("#elifdef").trim())
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
                let ifdef_directive =
                    Directives::from_str(line.trim().trim_start_matches("#ifdef").trim()).unwrap();
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

pub fn process_shader_source<Directives: PreprocessorDirectives>(
    shader_source: &str,
    preprocessor_directives: &HashSet<Directives>,
) -> String
where
    <Directives as FromStr>::Err: Debug,
{
    let mut processed_source = Vec::<String>::new();

    // Read shader source and replace includes with shader source files.
    for line in shader_source.split("\n") {
        if line.trim().starts_with("#include") {
            for line in Includes::from_str(line.trim().trim_start_matches("#include").trim())
                .unwrap()
                .source()
                .split("\n")
            {
                processed_source.push(line.to_string());
            }
        } else if line.trim().starts_with("//") || line.trim() == "" {
            continue;
        } else {
            processed_source.push(line.to_string());
        }
    }

    preprocess_directives(processed_source, preprocessor_directives).join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ray_marcher::*;

    #[test]
    fn test_all_directives_covered() {
        let mut preprocessor_directives: HashSet<RayMarcherPreprocessorDirectives> =
            all_directives_for_material();
        preprocessor_directives.extend(all_directives_for_primitive());
        preprocessor_directives.extend(all_directives_for_ray_marcher());
        preprocessor_directives.extend(all_directives_for_light());
        assert_eq!(
            preprocessor_directives.len(),
            RayMarcherPreprocessorDirectives::COUNT
        );
    }

    #[test]
    fn test_ifdef_preprocessor_directives() {
        let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableDiffuseColourTexture);

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
        let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableDiffuseColourTexture);

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
        let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableDiffuseColourTexture);
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableSpecularColourTexture);

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
        preprocessor_directives
            .remove(&RayMarcherPreprocessorDirectives::EnableDiffuseColourTexture);
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
        preprocessor_directives
            .remove(&RayMarcherPreprocessorDirectives::EnableSpecularColourTexture);
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
        let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableDiffuseColourTexture);
        preprocessor_directives
            .insert(RayMarcherPreprocessorDirectives::EnableSpecularColourTexture);

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
        preprocessor_directives
            .remove(&RayMarcherPreprocessorDirectives::EnableDiffuseColourTexture);
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

        let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();

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

        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableFiniteRepetition);

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

        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableInfiniteRepetition);

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

        preprocessor_directives.insert(RayMarcherPreprocessorDirectives::EnableFiniteRepetition);

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
