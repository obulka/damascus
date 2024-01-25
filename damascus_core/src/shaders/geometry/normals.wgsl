
/**
 * Estimate the surface normal at the closest point on the closest
 * object to a point.
 *
 * @arg position: The point near which to get the surface normal
 * @arg pixel_footprint: A value proportional to the amount of world
 *     space that fills a pixel, like the distance from camera.
 *
 * @returns: The normalized surface normal.
 */
fn estimate_surface_normal(position: vec3<f32>, pixel_footprint: f32) -> vec3<f32> {
    var normal_offset = vec2(0.5773, -0.5773);
    return normalize(
        normal_offset.xyy * signed_distance_to_scene(
            position + normal_offset.xyy * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
        )
        + normal_offset.yyx * signed_distance_to_scene(
            position + normal_offset.yyx * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
        )
        + normal_offset.yxy * signed_distance_to_scene(
            position + normal_offset.yxy * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
        )
        + normal_offset.xxx * signed_distance_to_scene(
            position + normal_offset.xxx * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
        )
    );
}
