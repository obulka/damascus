// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::{Mat4, Vec4};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUCamera {
    aperture: f32,
    focal_distance: f32,
    world_matrix: Mat4,
    inverse_world_matrix: Mat4,
    inverse_projection_matrix: Mat4,
    flags: u32,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub focal_length: f32,
    pub horizontal_aperture: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub focal_distance: f32,
    pub f_stop: f32,
    pub world_matrix: Mat4,
    pub enable_depth_of_field: bool,
    pub latlong: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            1.,
            50.,
            24.576,
            0.1,
            10000.,
            2.,
            16.,
            Mat4::IDENTITY,
            false,
            false,
        )
    }
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        focal_length: f32,
        horizontal_aperture: f32,
        near_plane: f32,
        far_plane: f32,
        focal_distance: f32,
        f_stop: f32,
        world_matrix: Mat4,
        enable_depth_of_field: bool,
        latlong: bool,
    ) -> Self {
        Self {
            aspect_ratio: aspect_ratio,
            focal_length: focal_length,
            horizontal_aperture: horizontal_aperture,
            near_plane: near_plane,
            far_plane: far_plane,
            focal_distance: focal_distance,
            f_stop: f_stop,
            world_matrix: world_matrix,
            enable_depth_of_field: enable_depth_of_field,
            latlong: latlong,
        }
    }

    fn aperture_from_f_stop(f_stop: f32, focal_length: f32) -> f32 {
        focal_length / f_stop / 1000.0
    }

    fn projection_matrix(&self) -> Mat4 {
        let far_to_near_plane_distance = self.far_plane - self.near_plane;
        Mat4::from_cols(
            Vec4::new(
                2. * self.focal_length / self.horizontal_aperture,
                0.,
                0.,
                0.,
            ),
            Vec4::new(
                0.,
                2. * self.focal_length / self.horizontal_aperture * self.aspect_ratio,
                0.,
                0.,
            ),
            Vec4::new(
                0.,
                0.,
                -(self.far_plane + self.near_plane) / far_to_near_plane_distance,
                -1.,
            ),
            Vec4::new(
                0.,
                0.,
                -2. * (self.far_plane * self.near_plane) / far_to_near_plane_distance,
                0.,
            ),
        )
    }

    fn to_gpu(&self) -> GPUCamera {
        GPUCamera {
            aperture: Self::aperture_from_f_stop(self.f_stop, self.focal_length),
            focal_distance: self.focal_distance,
            world_matrix: self.world_matrix,
            inverse_world_matrix: self.world_matrix.inverse(),
            inverse_projection_matrix: self.projection_matrix().inverse(),
            flags: self.enable_depth_of_field as u32 | (self.latlong as u32) << 1,
        }
    }

    pub fn as_std_430(&self) -> Std430GPUCamera {
        self.to_gpu().as_std430()
    }
}
