// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

const BEAUTY_AOV: u32 = 0u;
const WORLD_POSITION_AOV: u32 = 1u;
const LOCAL_POSITION_AOV: u32 = 2u;
const SURFACE_NORMAL_AOV: u32 = 3u;
const DEPTH_AOV: u32 = 4u;
const CRYPTOMATTE_AOV: u32 = 5u;
const STATS_AOV: u32 = 6u;


#ifdef EnableAOVs
fn early_exit_aovs(
    aov_type: u32,
    world_position: vec3f,
    local_position: vec3f,
    surface_normal: vec3f,
    primitive_id: u32,
    ray: ptr<function, Ray>,
) {
    switch aov_type {
        case WORLD_POSITION_AOV {
            (*ray).colour = world_position;
        }
        case LOCAL_POSITION_AOV {
            (*ray).colour = local_position;
        }
        case SURFACE_NORMAL_AOV {
            (*ray).colour = surface_normal;
        }
        case DEPTH_AOV {
            // Depth
            (*ray).colour = vec3(abs(world_to_camera_space(world_position).z));
        }
        case CRYPTOMATTE_AOV {
            // Cryptomatte
            (*ray).colour = random_vec3f(f32(primitive_id) * vec3(1., 2., 3.));
        }
        default {}
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
        case STATS_AOV {
            (*ray).colour = vec3(
                f32(bounces) / f32(_render_parameters.max_bounces),
                f32(iterations) / f32(_render_parameters.max_ray_steps),
                distance_travelled / _render_parameters.max_distance,
            );
        }
        default {}
    }
}
#endif


fn ray_miss_aovs(
    aov_type: u32,
    bounces: u32,
    iterations: u32,
    distance_travelled: f32,
    ray: ptr<function, Ray>,
    nested_dielectrics: ptr<function, NestedDielectrics>,
) {
    var world_position: vec3f = (*ray).origin + (*ray).direction * distance_travelled;

#ifdef EnableAOVs
    switch aov_type {
        case BEAUTY_AOV {
#endif
            sample_equiangular(
                distance_travelled,
                ray,
                nested_dielectrics,
            );
#ifdef EnableEmissiveColourTexture
            (*ray).colour += (*ray).throughput * procedurally_texture_vec3f(
                vec4((*ray).direction, 8.27447),
                _atmosphere.emissive_colour,
                _atmosphere.emissive_colour_texture,
            );
#else
            (*ray).colour += (*ray).throughput * _atmosphere.emissive_colour;
#endif
#ifdef EnableAOVs
        }
        case WORLD_POSITION_AOV, LOCAL_POSITION_AOV {
            (*ray).colour = world_position;
        }
        case DEPTH_AOV {
            // Depth
            (*ray).colour = vec3(abs(world_to_camera_space(world_position).z));
        }
        case STATS_AOV {
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
#endif
}
