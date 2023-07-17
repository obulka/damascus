use bytemuck;
use glam::{Mat4, Vec4};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUCamera {
    // enable_depth_of_field: bool,
    // aperture: f32,
    world_matrix: [[f32; 4]; 4],
    inverse_projection_matrix: [[f32; 4]; 4],
}

#[derive(Clone, Copy, Debug)]
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
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(1., 50., 24.576, 0.1, 10000., 2., 16., Mat4::IDENTITY, false)
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
        }
    }

    fn aperture_from_f_stop(f_stop: f32, focal_length: f32) -> f32 {
        focal_length / f_stop / 1000.0
    }

    fn projection_matrix(
        focal_length: f32,
        horizontal_aperture: f32,
        aspect_ratio: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Mat4 {
        let far_to_near_plane_distance = far_plane - near_plane;
        Mat4::from_cols(
            Vec4::new(2. * focal_length / horizontal_aperture, 0., 0., 0.),
            Vec4::new(
                0.,
                2. * focal_length / horizontal_aperture / aspect_ratio,
                0.,
                0.,
            ),
            Vec4::new(
                0.,
                0.,
                -(far_plane + near_plane) / far_to_near_plane_distance,
                -1.,
            ),
            Vec4::new(
                0.,
                0.,
                -2. * (far_plane * near_plane) / far_to_near_plane_distance,
                0.,
            ),
        )
    }

    pub fn to_gpu(&self) -> GPUCamera {
        GPUCamera {
            // enable_depth_of_field: self.enable_depth_of_field,
            // aperture: Self::aperture_from_f_stop(self.f_stop, self.focal_length),
            world_matrix: self.world_matrix.to_cols_array_2d(),
            inverse_projection_matrix: Self::projection_matrix(
                self.focal_length,
                self.horizontal_aperture,
                self.aspect_ratio,
                self.near_plane,
                self.far_plane,
            )
            .inverse()
            .to_cols_array_2d(),
        }
    }
}
