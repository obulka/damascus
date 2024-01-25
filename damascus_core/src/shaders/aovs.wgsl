
const BEAUTY_AOV: u32 = 0u;
const STATS_AOV: u32 = 5u;


fn early_exit_aovs(
    aov_type: u32,
    world_position: vec3<f32>,
    local_position: vec3<f32>,
    surface_normal: vec3<f32>,
) -> vec3<f32> {
    switch aov_type {
        case 1u {
            return world_position;
        }
        case 2u {
            return local_position;
        }
        case 3u {
            return surface_normal;
        }
        case 4u {
            // Depth
            return vec3(abs(world_to_camera_space(world_position).z));
        }
        default {
            return vec3(-1.); // Invalid!!
        }
    }
}


fn final_aovs(
    aov_type: u32,
    bounces: u32,
    iterations: u32,
    distance_travelled: f32,
) -> vec3<f32> {
    switch aov_type {
        case 5u {
            return vec3(
                f32(bounces) / f32(_render_params.ray_marcher.max_bounces),
                f32(iterations) / f32(_render_params.ray_marcher.max_ray_steps),
                distance_travelled / _render_params.ray_marcher.max_distance,
            );
        }
        default {
            return vec3(-1.); // Invalid!!
        }
    }
}
