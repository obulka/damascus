// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

/**
 * Convert location in uv space to pixel space.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel in uv space.
 * @arg resolution: The image width, and height.
 *
 * @returns: The pixel indices.
 */
fn uv_to_pixels(pixel_coordinates: vec2f, resolution: vec2f) -> vec2f {
    return (pixel_coordinates + 1.) * (resolution - 1.) * 0.5;
}

/**
 * Convert location in pixel space to uv space.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel in uv space.
 * @arg resolution: The image width, and height.
 *
 * @returns: The pixel indices.
 */
fn pixels_to_uv(pixel_coordinates: vec2f, resolution: vec2f) -> vec2f {
    return pixel_coordinates * 2. / (resolution - 1.) - 1.;
}

fn scale_pixels_to_uv(pixel_coordinates: vec2f, resolution: vec2f) -> vec2f {
    return pixel_coordinates * 2. / resolution;
}
