// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


const ENABLE_DEPTH_OF_FIELD: u32 = 1u;
const LATLONG: u32 = 2u;


struct Camera {
    aperture: f32,
    focal_distance: f32,
    camera_to_world: mat4x4f,
    world_to_camera: mat4x4f,
    screen_to_camera: mat4x4f,
    flags: u32,
}


@group(UNIFORM_BIND_GROUP) @binding(3)
var<uniform> _render_camera: Camera;


/**
 * Convert location of a pixel in an image into uv.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel.
 * @arg resolution: The image width, and height.
 *
 * @returns: The uv position.
 */
fn pixels_to_uv(pixel_coordinates: vec2f, resolution: vec2f) -> vec2f {
    return 2. * pixel_coordinates / resolution - 1.;
}


fn world_to_camera_space(world_position: vec3f) -> vec3f {
    return (
        _render_camera.world_to_camera
        * vec4(world_position, 1.)
    ).xyz;
}


/**
 * Get the position of the render camera.
 *
 * @returns: The position of the render camera.
 */
fn render_camera_position() -> vec3f {
    return vec3(
        _render_camera.camera_to_world[3][0],
        _render_camera.camera_to_world[3][1],
        _render_camera.camera_to_world[3][2],
    );
}


/**
 * Get the rotation of the render camera.
 *
 * @returns: The rotation of the render camera.
 */
fn render_camera_rotation() -> mat3x3f {
    var rotation_matrix = mat3x3f();
    rotation_matrix[0][0] = _render_camera.camera_to_world[0][0];
    rotation_matrix[0][1] = _render_camera.camera_to_world[0][1];
    rotation_matrix[0][2] = _render_camera.camera_to_world[0][2];
    rotation_matrix[1][0] = _render_camera.camera_to_world[1][0];
    rotation_matrix[1][1] = _render_camera.camera_to_world[1][1];
    rotation_matrix[1][2] = _render_camera.camera_to_world[1][2];
    rotation_matrix[2][0] = _render_camera.camera_to_world[2][0];
    rotation_matrix[2][1] = _render_camera.camera_to_world[2][1];
    rotation_matrix[2][2] = _render_camera.camera_to_world[2][2];
    return rotation_matrix;
}


/**
 * Create a ray out of the camera. It will be either a standard ray,
 * a latlong ray, or a ray that will result in depth of field.
 *
 * @arg seed: The seed to use in randomization.
 * @arg uv_coordinate: The u, and v locations of the pixel.
 */
fn create_render_camera_ray(seed: vec2f, uv_coordinate: vec2f) -> Ray {
    if (bool(_render_camera.flags & LATLONG)) {
        return Ray(
            render_camera_position(),
            render_camera_rotation() * spherical_unit_vector_to_cartesion(
                uv_coordinate_to_angles(uv_coordinate.xy),
            ),
            vec3(0.),
            vec3(1.),
        );
    }
    var ray = Ray(
        render_camera_position(),
        normalize((
            _render_camera.camera_to_world
            * vec4(
                (_render_camera.screen_to_camera * vec4(uv_coordinate, 0., 1.)).xyz,
                0.,
            )
        ).xyz),
        vec3(0.),
        vec3(1.),
    );

    if (!bool(_render_camera.flags & ENABLE_DEPTH_OF_FIELD)) {
        return ray;
    }

    // Depth of field
    var camera_forward: vec3f = (
        _render_camera.camera_to_world * vec4(0., 0., -1., 0.)
    ).xyz;
    var camera_right: vec3f = (
        _render_camera.camera_to_world * vec4(1., 0., 0., 0.)
    ).xyz;
    var camera_up: vec3f = (
        _render_camera.camera_to_world * vec4(0., 1., 0., 0.)
    ).xyz;

    var focal_plane_point: vec3f = (
        ray.origin + camera_forward * _render_camera.focal_distance
    );
    var focal_plane_normal: vec3f = -camera_forward;

    var focal_point_distance: f32 = (
        (dot(focal_plane_normal, focal_plane_point) - dot(ray.origin, focal_plane_normal))
        / dot(ray.direction, focal_plane_normal)
    );
    var focal_point: vec3f = ray.origin + focal_point_distance * ray.direction;

    var point_in_unit_circle: vec2f = uniform_point_in_unit_circle(seed);
    var offset: vec2f = point_in_unit_circle.x * _render_camera.aperture * vec2(
        cos(point_in_unit_circle.y),
        sin(point_in_unit_circle.y),
    );

    ray.origin += camera_right * offset.x + camera_up * offset.y;
    ray.direction = normalize(focal_point - ray.origin);

    return ray;
}
