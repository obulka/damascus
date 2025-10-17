// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashSet;

use crevice::std430::AsStd430;

use crate::{
    DualDevice,
    camera::{Camera, GPUCamera},
    geometry::primitives::GPUPrimitive,
    lights::GPULight,
    materials::{GPUMaterial, Material},
    shaders::scene::ScenePreprocessorDirectives,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUSceneArrayLengths {
    num_primitives: u32,
    num_lights: u32,
    num_materials: u32,
    num_non_physical_lights: u32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUScene {
    pub cameras: Vec<GPUCamera>,
    pub primitives: Vec<GPUPrimitive>,
    pub lights: Vec<GPULight>,
    pub materials: Vec<GPUMaterial>,
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
            materials: vec![Material::default().to_gpu()],
            emissive_primitive_indices: vec![],
            render_camera: 0,
            atmosphere: 0,
            array_lengths: GPUSceneArrayLengths {
                num_primitives: 0,
                num_lights: 0,
                num_materials: 1,
                num_non_physical_lights: 0,
            },
            preprocessor_directives: HashSet::<ScenePreprocessorDirectives>::new(),
        }
    }
}
