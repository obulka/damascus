// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use std::str::FromStr;

use strum::EnumString;

#[derive(Debug, EnumString)]
enum Includes {
    AOVs,
    Camera,
    Material,
    Lights,
    Math,
    Noise,
    Normals,
    Primitive,
    PrimitiveModifiers,
    PrimitiveSDFs,
    Random,
    Ray,
    RenderParameters,
    SceneSDFs,
    VertexShader,
}

impl Includes {
    fn source(&self) -> &str {
        match *self {
            Self::AOVs => include_str!("./renderer/aovs.wgsl"),
            Self::Camera => include_str!("./camera.wgsl"),
            Self::Lights => include_str!("./lights.wgsl"),
            Self::Material => include_str!("./materials/material.wgsl"),
            Self::Math => include_str!("./utils/math.wgsl"),
            Self::Noise => include_str!("./utils/noise.wgsl"),
            Self::Normals => include_str!("./geometry/normals.wgsl"),
            Self::Primitive => include_str!("./geometry/primitive.wgsl"),
            Self::PrimitiveModifiers => include_str!("./geometry/modifiers.wgsl"),
            Self::PrimitiveSDFs => include_str!("./geometry/primitive_sdfs.wgsl"),
            Self::Random => include_str!("./utils/random.wgsl"),
            Self::Ray => include_str!("./geometry/ray.wgsl"),
            Self::RenderParameters => include_str!("./renderer/render_parameters.wgsl"),
            Self::SceneSDFs => include_str!("./geometry/scene_sdfs.wgsl"),
            Self::VertexShader => include_str!("./renderer/vertex_shader.wgsl"),
        }
    }
}

pub fn ray_march_shader() -> String {
    let mut shader_source: String = "".to_string();
    for line in include_str!("./renderer/ray_march.wgsl").split("\n") {
        if line.trim().starts_with("#include") {
            shader_source += Includes::from_str(line.trim().trim_start_matches("#include").trim())
                .unwrap()
                .source();
        } else {
            shader_source += &(line.to_owned() + "\n");
        }
    }
    return shader_source;
}
