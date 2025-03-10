const UNIFORM_BIND_GROUP: u32 = 0u;
const TEXTURE_BIND_GROUP: u32 = 1u;

#include CompositorRenderParameters
#include VertexShader

// @group(TEXTURE_BIND_GROUP) @binding(0)
// var _texture: texture_2d<rgba32float, read>;


@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    // Use the UV coordinates and resolution to get texture coordinates
    var current_pixel_indices: vec2f = uv_to_pixels(
        in.uv_coordinate.xy,
        _render_state.resolution,
    );
    var texture_coordinates = vec2u(current_pixel_indices);

    var pixel_colour: vec4f = in.uv_coordinate;//textureLoad(_texture, texture_coordinates);

    return pixel_colour;
}
