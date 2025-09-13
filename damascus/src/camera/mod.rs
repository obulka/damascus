// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::{Mat4, UVec2, Vec4};

use crate::DualDevice;

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUCamera {
    flags: u32,
    sensor_resolution: UVec2,
    aperture: f32,
    focal_distance: f32,
    camera_to_world: Mat4,
    world_to_camera: Mat4,
    screen_to_camera: Mat4,
    camera_to_screen: Mat4,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Camera {
    pub focal_length: f32,
    pub horizontal_aperture: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub focal_distance: f32,
    pub f_stop: f32,
    pub sensor_resolution: UVec2,
    pub camera_to_world: Mat4,
    pub enable_depth_of_field: bool,
    pub latlong: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            50.,
            24.576,
            0.1,
            10000.,
            2.,
            16.,
            UVec2::new(1920, 1080),
            Mat4::IDENTITY,
            false,
            false,
        )
    }
}

impl Camera {
    pub fn new(
        focal_length: f32,
        horizontal_aperture: f32,
        near_plane: f32,
        far_plane: f32,
        focal_distance: f32,
        f_stop: f32,
        sensor_resolution: UVec2,
        camera_to_world: Mat4,
        enable_depth_of_field: bool,
        latlong: bool,
    ) -> Self {
        Self {
            focal_length: focal_length,
            horizontal_aperture: horizontal_aperture,
            near_plane: near_plane.max(1e-5),
            far_plane: far_plane,
            focal_distance: focal_distance,
            f_stop: f_stop,
            sensor_resolution: sensor_resolution,
            camera_to_world: camera_to_world,
            enable_depth_of_field: enable_depth_of_field,
            latlong: latlong,
        }
    }

    pub fn aperture_from_f_stop(f_stop: f32, focal_length: f32) -> f32 {
        focal_length / f_stop / 1000.0
    }

    pub fn camera_to_screen(&self) -> Mat4 {
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
                2. * self.focal_length / self.horizontal_aperture
                    * (self.sensor_resolution.x as f32)
                    / (self.sensor_resolution.y as f32),
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

    pub fn focal_length(mut self, focal_length: f32) -> Self {
        self.focal_length = focal_length;
        self
    }

    pub fn horizontal_aperture(mut self, horizontal_aperture: f32) -> Self {
        self.horizontal_aperture = horizontal_aperture;
        self
    }

    pub fn near_plane(mut self, near_plane: f32) -> Self {
        self.near_plane = near_plane;
        self
    }

    pub fn far_plane(mut self, far_plane: f32) -> Self {
        self.far_plane = far_plane;
        self
    }

    pub fn focal_distance(mut self, focal_distance: f32) -> Self {
        self.focal_distance = focal_distance;
        self
    }

    pub fn f_stop(mut self, f_stop: f32) -> Self {
        self.f_stop = f_stop;
        self
    }

    pub fn sensor_resolution(mut self, sensor_resolution: UVec2) -> Self {
        self.sensor_resolution = sensor_resolution;
        self
    }

    pub fn camera_to_world(mut self, camera_to_world: Mat4) -> Self {
        self.camera_to_world = camera_to_world;
        self
    }

    pub fn enable_depth_of_field(mut self, enable_depth_of_field: bool) -> Self {
        self.enable_depth_of_field = enable_depth_of_field;
        self
    }

    pub fn latlong(mut self, latlong: bool) -> Self {
        self.latlong = latlong;
        self
    }
}

impl DualDevice<GPUCamera, Std430GPUCamera> for Camera {
    fn to_gpu(&self) -> GPUCamera {
        let camera_to_screen: Mat4 = self.camera_to_screen();
        GPUCamera {
            flags: self.enable_depth_of_field as u32 | (self.latlong as u32) << 1,
            sensor_resolution: self.sensor_resolution,
            aperture: Self::aperture_from_f_stop(self.f_stop, self.focal_length),
            focal_distance: self.focal_distance,
            camera_to_world: self.camera_to_world,
            world_to_camera: self.camera_to_world.inverse(),
            screen_to_camera: camera_to_screen.inverse(),
            camera_to_screen: camera_to_screen,
        }
    }
}
