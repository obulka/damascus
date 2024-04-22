// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use std::str::FromStr;

use strum::EnumString;

#[derive(Debug, EnumString)]
enum Includes {
    RenderParameters,
    Ray,
    Math,
    Random,
    PrimitiveSDFs,
    Materials,
    Primitive,
    PrimitiveModifiers,
    SceneSDFs,
    Normals,
    Lights,
    Camera,
    AOVs,
    VertexShader,
}

impl Includes {
    fn source(&self) -> &str {
        match *self {
            Self::RenderParameters => include_str!("./render_parameters.wgsl"),
            Self::Ray => include_str!("./ray.wgsl"),
            Self::Math => include_str!("./math.wgsl"),
            Self::Random => include_str!("./random.wgsl"),
            Self::PrimitiveSDFs => include_str!("./geometry/primitive_sdfs.wgsl"),
            Self::Materials => include_str!("./materials.wgsl"),
            Self::Primitive => include_str!("./geometry/primitive.wgsl"),
            Self::PrimitiveModifiers => include_str!("./geometry/modifiers.wgsl"),
            Self::SceneSDFs => include_str!("./geometry/scene_sdfs.wgsl"),
            Self::Normals => include_str!("./geometry/normals.wgsl"),
            Self::Lights => include_str!("./lights.wgsl"),
            Self::Camera => include_str!("./camera.wgsl"),
            Self::AOVs => include_str!("./aovs.wgsl"),
            Self::VertexShader => include_str!("./vertex_shader.wgsl"),
        }
    }
}

pub fn ray_march_shader() -> String {
    let mut shader_source: String = "".to_string();
    for line in include_str!("./ray_march.wgsl").split("\n") {
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
