
const BEAUTY_AOV: u32 = 0u;
const STATS_AOV: u32 = 5u;


fn early_exit_aovs(
    aov_type: u32,
    world_position: vec3<f32>,
    local_position: vec3<f32>,
    surface_normal: vec3<f32>,
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
        case 5u {
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
    world_position: vec3<f32>,
    ray: ptr<function, Ray>,
) {
    switch aov_type {
        case 0u {
            (*ray).colour += (*ray).throughput * procedurally_texture(
                world_position,
                _atmosphere.diffuse_colour,
                _atmosphere.diffuse_texture,
            );
        }
        case 1u, 2u {
            (*ray).colour = world_position;
        }
        case 3u {
            (*ray).colour = vec3(0.);
        }
        case 4u {
            // Depth
            (*ray).colour = vec3(abs(world_to_camera_space(world_position).z));
        }
        case 5u {
            (*ray).colour = vec3(
                f32(bounces) / f32(_render_parameters.max_bounces),
                f32(iterations) / f32(_render_parameters.max_ray_steps),
                distance_travelled / _render_parameters.max_distance,
            );
        }
        default {
            (*ray).colour = vec3(-1.); // Invalid!!
        }
    }
}
