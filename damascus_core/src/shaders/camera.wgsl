
struct Camera {
    enable_depth_of_field: u32, // bool isn't host-shareable?
    aperture: f32,
    focal_distance: f32,
    world_matrix: mat4x4<f32>,
    inverse_world_matrix: mat4x4<f32>,
    inverse_projection_matrix: mat4x4<f32>,
}


@group(1) @binding(0)
var<uniform> _render_camera: Camera;


fn world_to_camera_space(world_position: vec3<f32>) -> vec3<f32> {
    return (
        _render_camera.inverse_world_matrix
        * vec4(world_position, 1.)
    ).xyz;
}


/**
 * Generate a ray out of a camera.
 *
 * @arg uv_coordinate: The UV position in the resulting image.
 */
fn create_ray(uv_coordinate: vec4<f32>) -> Ray {
    return Ray(
        vec3(
            _render_camera.world_matrix[3][0],
            _render_camera.world_matrix[3][1],
            _render_camera.world_matrix[3][2],
        ),
        normalize((
            _render_camera.world_matrix
            * vec4(
                (_render_camera.inverse_projection_matrix * uv_coordinate).xyz,
                0.,
            )
        ).xyz),
        vec3(0.),
        vec3(1.),
    );
}


/**
 * Create a ray out of the camera. It will be either a standard ray,
 * a latlong ray, or a ray that will result in depth of field.
 *
 * @arg seed: The seed to use in randomization.
 * @arg uv_coordinate: The u, and v locations of the pixel.
 */
fn create_render_camera_ray(seed: vec3<f32>, uv_coordinate: vec4<f32>) -> Ray {
    // if (bool(_render_params.ray_marcher.latlong))
    // {
    //     // create_latlong_ray(
    //     //     uv_coordinate,
    //     //     ray_origin,
    //     //     ray_direction,
    //     // );
    // }
    // else if (bool(_render_camera.enable_depth_of_field))
    // {
    //     // create_ray_with_dof(
    //     //     uv_coordinate,
    //     //     seed,
    //     //     ray_origin,
    //     //     ray_direction,
    //     // );
    // }
    // else
    // {
    return create_ray(uv_coordinate);
    // }
}
