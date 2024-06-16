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
    EnableSpecularTexture,
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

pub fn ray_march_shader(preprocessor_directives: HashSet<PreprocessorDirectives>) -> String {
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

    // Handle ifdef preprocessor macro
    let mut skipping_to_endif: bool = false;
    shader_source
        .into_iter()
        .filter(|line| {
            let hit_endif: bool = line.trim().starts_with("#endif");
            if skipping_to_endif {
                skipping_to_endif = !hit_endif;
                false
            } else if line.trim().starts_with("#ifdef") {
                let ifdef_directive = PreprocessorDirectives::from_str(
                    line.trim().trim_start_matches("#ifdef").trim(),
                )
                .unwrap();
                if !preprocessor_directives.contains(&ifdef_directive) {
                    skipping_to_endif = true;
                }
                false
            } else if hit_endif {
                false
            } else {
                true
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
