// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

const INVERT: u32 = 1u;


/**
 * Convert location of a pixel in screen/image space from uvs.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel in uv space.
 * @arg resolution: The image width, and height.
 *
 * @returns: The pixel indices.
 */
fn uv_to_screen(pixel_coordinates: vec2f, resolution: vec2f) -> vec2f {
    return (pixel_coordinates + 1.) * (resolution - 1.) * 0.5;
}


/**
 * Convert location of a pixel in screen/image space from uvs.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel in uv space.
 * @arg resolution: The image width, and height.
 *
 * @returns: The pixel indices.
 */
fn screen_to_uv(pixel_coordinates: vec2f, resolution: vec2f) -> vec2f {
    return pixel_coordinates * 2. / (resolution - 1.) - 1.;
}


fn scale_screen_to_uv(pixel_coordinates: vec2f, resolution: vec2f) -> vec2f {
    return pixel_coordinates * 2. / resolution;
}


fn grade_f32(colour: f32, grade: Grade) -> f32 {
    return select(
        pow(
            grade.gain * (
                (1. - grade.lift)
                * saturate_f32(
                    select(
                        colour,
                        1. - colour,
                        bool(grade.flags & INVERT),
                    ) - grade.black_point,
                ) / (grade.white_point - grade.black_point)
                + grade.lift
            ),
            1. / grade.gamma,
        ),
        0.,
        grade.white_point == grade.black_point,
    );
}


fn grade_vec3(colour: vec3f, grade: Grade) -> vec3f {
    return select(
        pow(
            grade.gain * (
                (1. - grade.lift)
                * saturate_vec3f(
                    select(
                        colour,
                        1. - colour,
                        bool(grade.flags & INVERT),
                    ) - grade.black_point,
                ) / (grade.white_point - grade.black_point)
                + grade.lift
            ),
            vec3(1. / grade.gamma),
        ),
        vec3f(),
        grade.white_point == grade.black_point,
    );
}

fn grade_vec4(colour: vec4f, grade: Grade) -> vec4f {
    return select(
        pow(
            grade.gain * (
                (1. - grade.lift)
                * saturate_vec4f(
                    select(
                        colour,
                        1. - colour,
                        bool(grade.flags & INVERT),
                    ) - grade.black_point,
                ) / (grade.white_point - grade.black_point)
                + grade.lift
            ),
            vec4(1. / grade.gamma),
        ),
        vec4f(),
        grade.white_point == grade.black_point,
    );
}

struct Grade {
    black_point: f32,
    white_point: f32,
    lift: f32,
    gain: f32,
    gamma: f32,
    flags: u32,
}

