// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


/**
 * Convert location of a pixel in screen/image space from uvs.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel in uv space.
 * @arg resolution: The image width, and height.
 *
 * @returns: The pixel indices.
 */
fn uv_to_screen(pixel_coordinates: vec2f, resolution: vec2f) -> vec2f {
    return (pixel_coordinates + 1.) * resolution / 2.;
}
