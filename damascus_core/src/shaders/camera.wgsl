
struct Camera {
    enable_depth_of_field: u32, // bool isn't host-shareable?
    aperture: f32,
    focal_distance: f32,
    world_matrix: mat4x4<f32>,
    inverse_world_matrix: mat4x4<f32>,
    inverse_projection_matrix: mat4x4<f32>,
}


@group(0) @binding(3)
var<uniform> _render_camera: Camera;


/**
 * Convert location of a pixel in an image into uv.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel.
 * @arg resolution: The image width, and height.
 *
 * @returns: The uv position.
 */
fn pixels_to_uv(pixel_coordinates: vec2<f32>, resolution: vec2<f32>) -> vec2<f32> {
    return 2. * pixel_coordinates / resolution - 1.;
}


/**
 * Convert location of a pixel in an image from uv.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel in uv space.
 * @arg resolution: The image width, and height.
 *
 * @returns: The pixel indices.
 */
fn uv_to_pixels(pixel_coordinates: vec2<f32>, resolution: vec2<f32>) -> vec2<f32> {
    return (pixel_coordinates + 1.) * resolution / 2.;
}


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
fn create_ray(uv_coordinate: vec2<f32>) -> Ray {
    return Ray(
        vec3(
            _render_camera.world_matrix[3][0],
            _render_camera.world_matrix[3][1],
            _render_camera.world_matrix[3][2],
        ),
        normalize((
            _render_camera.world_matrix
            * vec4(
                (_render_camera.inverse_projection_matrix * vec4(uv_coordinate, 0., 1.)).xyz,
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
fn create_render_camera_ray(seed: vec2<f32>, uv_coordinate: vec2<f32>) -> Ray {
    // if (bool(_render_params.ray_marcher.latlong))
    // {
    //     // create_latlong_ray(
    //     //     uv_coordinate,
    //     //     ray_origin,
    //     //     ray_direction,
    //     // );
    // }
    var ray: Ray = create_ray(uv_coordinate);

    if (!bool(_render_camera.enable_depth_of_field)) {
        return ray;
    }

    // Depth of field
    var camera_forward: vec3<f32> = (
        _render_camera.world_matrix * vec4(0., 0., -1., 0.)
    ).xyz;
    var camera_right: vec3<f32> = (
        _render_camera.world_matrix * vec4(1., 0., 0., 0.)
    ).xyz;
    var camera_up: vec3<f32> = (
        _render_camera.world_matrix * vec4(0., 1., 0., 0.)
    ).xyz;

    var focal_plane_point: vec3<f32> = (
        ray.origin + camera_forward * _render_camera.focal_distance
    );
    var focal_plane_normal: vec3<f32> = -camera_forward;

    var focal_point_distance: f32 = (
        (dot(focal_plane_normal, focal_plane_point) - dot(ray.origin, focal_plane_normal))
        / dot(ray.direction, focal_plane_normal)
    );
    var focal_point: vec3<f32> = ray.origin + focal_point_distance * ray.direction;

    var point_in_unit_circle: vec2<f32> = uniform_point_in_unit_circle(seed);
    var offset: vec2<f32> = point_in_unit_circle.x * _render_camera.aperture * vec2(
        cos(point_in_unit_circle.y),
        sin(point_in_unit_circle.y),
    );

    ray.origin += camera_right * offset.x + camera_up * offset.y;
    ray.direction = normalize(focal_point - ray.origin);

    return ray;
}
