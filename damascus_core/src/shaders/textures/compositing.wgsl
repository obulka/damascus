

@group(0) @binding(0)
var _progressive_rendering_texture: texture_storage_2d<rgba32float, read_write>;


@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    // Use the UV coordinates and resolution to get texture coordinates
    var current_pixel_indices: vec2f = uv_to_pixels(
        in.uv_coordinate.xy,
        _render_state.resolution,
    );
    var texture_coordinates = vec2u(current_pixel_indices);

    // Load the current state of the progressive render, unless this is
    // the first path, in which case initialise as black
    var pixel_colour: vec4f = textureLoad(
        _progressive_rendering_texture,
        texture_coordinates,
    );

    // If the render is paused just return the current texture value
    if bool(_render_state.flags & PAUSED) {
        return pixel_colour;
    }

    return pixel_colour;
}
