const UNIFORM_BIND_GROUP: u32 = 0u;
const TEXTURE_BIND_GROUP: u32 = 1u;

#include Texture
#include CompositorRenderParameters
#include VertexShader


@group(TEXTURE_BIND_GROUP) @binding(0)
var _texture: texture_2d<f32>;


@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    // Use the UV coordinates and resolution to get texture coordinates
    var texture_dimensions: vec2u = textureDimensions(_texture);
    var current_pixel_indices: vec2f = uv_to_screen(
        in.uv_coordinate.xy,
        vec2f(texture_dimensions),//_render_state.resolution,
    );

    var texture_coordinates = vec2u(current_pixel_indices);

    var pixel_colour: vec4f = textureLoad(_texture, texture_coordinates, 0);

    return pixel_colour;
}
