use glam::Mat4;

#[derive(Default)]
pub struct Camera {
    pub focal_length: f32,
    pub horizontal_aperture: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub focal_distance: f32,
    pub f_stops: f32,
    pub world_matrix: Mat4,
}
