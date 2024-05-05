// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.


const BEAUTY_AOV: u32 = 0u;
const STATS_AOV: u32 = 6u;


fn early_exit_aovs(
    aov_type: u32,
    world_position: vec3<f32>,
    local_position: vec3<f32>,
    surface_normal: vec3<f32>,
    primitive_id: u32,
    ray: ptr<function, Ray>,
) {
    switch aov_type {
        case 1u {
            (*ray).colour = world_position;
        }
        case 2u {
            (*ray).colour = local_position;
        }
        case 3u {
            (*ray).colour = surface_normal;
        }
        case 4u {
            // Depth
            (*ray).colour = vec3(abs(world_to_camera_space(world_position).z));
        }
        case 5u {
            // Cryptomatte
            (*ray).colour = random_vec3f(f32(primitive_id) * vec3(1., 2., 3.));
        }
        default {
            (*ray).colour = vec3(-1.); // Invalid!!
        }
    }
}


fn final_aovs(
    aov_type: u32,
    bounces: u32,
    iterations: u32,
    distance_travelled: f32,
    ray: ptr<function, Ray>,
) {
    switch aov_type {
        case 6u {
            (*ray).colour = vec3(
                f32(bounces) / f32(_render_parameters.max_bounces),
                f32(iterations) / f32(_render_parameters.max_ray_steps),
                distance_travelled / _render_parameters.max_distance,
            );
        }
        default {}
    }
}


fn ray_miss_aovs(
    aov_type: u32,
    bounces: u32,
    iterations: u32,
    distance_travelled: f32,
    ray: ptr<function, Ray>,
    nested_dielectrics: ptr<function, NestedDielectrics>,
) {
    var world_position: vec3<f32> = (*ray).origin + (*ray).direction * distance_travelled;

    switch aov_type {
        case 0u {
            sample_equiangular(
                distance_travelled,
                ray,
                nested_dielectrics,
            );
            var texture: ProceduralTexture = _atmosphere.diffuse_texture;
            (*ray).colour += (*ray).throughput * procedurally_texture(
                vec4((*ray).direction, 10.),
                _atmosphere.diffuse_colour,
                &texture,
            );
        }
        case 1u, 2u {
            (*ray).colour = world_position;
        }
        case 4u {
            // Depth
            (*ray).colour = vec3(abs(world_to_camera_space(world_position).z));
        }
        case 6u {
            (*ray).colour = vec3(
                f32(bounces) / f32(_render_parameters.max_bounces),
                f32(iterations) / f32(_render_parameters.max_ray_steps),
                distance_travelled / _render_parameters.max_distance,
            );
        }
        default {
            (*ray).colour = vec3(0.);
        }
    }
}
