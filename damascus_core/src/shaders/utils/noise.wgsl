// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.


var<private> PERM: array<array<i32, 4>, 32> = array<u32, 256>(
    151u, 160u, 137u, 91u, 90u, 15u, 131u, 13u, 201u, 95u, 96u, 53u, 194u, 233u,
    7u, 225u, 140u, 36u, 103u, 30u, 69u, 142u, 8u, 99u, 37u, 240u, 21u, 10u, 23u,
    190u, 6u, 148u, 247u, 120u, 234u, 75u, 0u, 26u, 197u, 62u, 94u, 252u, 219u,
    203u, 117u, 35u, 11u, 32u, 57u, 177u, 33u, 88u, 237u, 149u, 56u, 87u, 174u,
    20u, 125u, 136u, 171u, 168u, 68u, 175u, 74u, 165u, 71u, 134u, 139u, 48u,
    27u, 166u, 77u, 146u, 158u, 231u, 83u, 111u, 229u, 122u, 60u, 211u, 133u,
    230u, 220u, 105u, 92u, 41u, 55u, 46u, 245u, 40u, 244u, 102u, 143u, 54u, 65u,
    25u, 63u, 161u, 1u, 216u, 80u, 73u, 209u, 76u, 132u, 187u, 208u, 89u, 18u,
    169u, 200u, 196u, 135u, 130u, 116u, 188u, 159u, 86u, 164u, 100u, 109u, 198u,
    173u, 186u, 3u, 64u, 52u, 217u, 226u, 250u, 124u, 123u, 5u, 202u, 38u, 147u,
    118u, 126u, 255u, 82u, 85u, 212u, 207u, 206u, 59u, 227u, 47u, 16u, 58u, 17u,
    182u, 189u, 28u, 42u, 223u, 183u, 170u, 213u, 119u, 248u, 152u, 2u, 44u,
    154u, 163u, 70u, 221u, 153u, 101u, 155u, 167u, 43u, 172u, 9u, 129u, 22u,
    39u, 253u, 19u, 98u, 108u, 110u, 79u, 113u, 224u, 232u, 178u, 185u, 112u,
    104u, 218u, 246u, 97u, 228u, 251u, 34u, 242u, 193u, 238u, 210u, 144u, 12u,
    191u, 179u, 162u, 241u, 81u, 51u, 145u, 235u, 249u, 14u, 239u, 107u, 49u,
    192u, 214u, 31u, 181u, 199u, 106u, 157u, 184u, 84u, 204u, 176u, 115u, 121u,
    50u, 45u, 127u, 4u, 150u, 254u, 138u, 236u, 205u, 93u, 222u, 114u, 67u, 29u,
    24u, 72u, 243u, 141u, 128u, 195u, 78u, 66u, 215u, 61u, 156u, 180u,
);


var<private> GRAD4: array<array<i32, 4>, 32> = array<array<i32, 4>, 32>(
    array<i32, 4>(0, 1, 1, 1),
    array<i32, 4>(0, 1, 1, -1),
    array<i32, 4>(0, 1, -1, 1),
    array<i32, 4>(0, 1, -1, -1),
    array<i32, 4>(0, -1, 1, 1),
    array<i32, 4>(0, -1, 1, -1),
    array<i32, 4>(0, -1, -1, 1),
    array<i32, 4>(0, -1, -1, -1),
    array<i32, 4>(1, 0, 1, 1),
    array<i32, 4>(1, 0, 1, -1),
    array<i32, 4>(1, 0, -1, 1),
    array<i32, 4>(1, 0, -1, -1),
    array<i32, 4>(-1, 0, 1, 1),
    array<i32, 4>(-1, 0, 1, -1),
    array<i32, 4>(-1, 0, -1, 1),
    array<i32, 4>(-1, 0, -1, -1),
    array<i32, 4>(1, 1, 0, 1),
    array<i32, 4>(1, 1, 0, -1),
    array<i32, 4>(1, -1, 0, 1),
    array<i32, 4>(1, -1, 0, -1),
    array<i32, 4>(-1, 1, 0, 1),
    array<i32, 4>(-1, 1, 0, -1),
    array<i32, 4>(-1, -1, 0, 1),
    array<i32, 4>(-1, -1, 0, -1),
    array<i32, 4>(1, 1, 1, 0),
    array<i32, 4>(1, 1, -1, 0),
    array<i32, 4>(1, -1, 1, 0),
    array<i32, 4>(1, -1, -1, 0),
    array<i32, 4>(-1, 1, 1, 0),
    array<i32, 4>(-1, 1, -1, 0),
    array<i32, 4>(-1, -1, 1, 0),
    array<i32, 4>(-1, -1, -1, 0),
);


var<private> SIMPLEX: array<array<u32, 4>, 64> = array<array<u32, 4>, 64>(
    array<u32, 4>(0u, 1u, 2u, 3u),
    array<u32, 4>(0u, 1u, 3u, 2u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 2u, 3u, 1u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(1u, 2u, 3u, 0u),
    array<u32, 4>(0u, 2u, 1u, 3u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 3u, 1u, 2u),
    array<u32, 4>(0u, 3u, 2u, 1u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(1u, 3u, 2u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(1u, 2u, 0u, 3u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(1u, 3u, 0u, 2u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(2u, 3u, 0u, 1u),
    array<u32, 4>(2u, 3u, 1u, 0u),
    array<u32, 4>(1u, 0u, 2u, 3u),
    array<u32, 4>(1u, 0u, 3u, 2u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(2u, 0u, 3u, 1u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(2u, 1u, 3u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(2u, 0u, 1u, 3u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(3u, 0u, 1u, 2u),
    array<u32, 4>(3u, 0u, 2u, 1u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(3u, 1u, 2u, 0u),
    array<u32, 4>(2u, 1u, 0u, 3u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(3u, 1u, 0u, 2u),
    array<u32, 4>(0u, 0u, 0u, 0u),
    array<u32, 4>(3u, 2u, 0u, 1u),
    array<u32, 4>(3u, 2u, 1u, 0u),
);


const G4: f32 = 0.138196601;


/**
 * 4D Perlin simplex noise
 *
 * Copyright (c) 2007-2012 Eliot Eshelman
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 * @arg seed: The seed for the noise.
 *
 * @returns: Noise value in the range [-1, 1], value of 0 on all integer
 *     coordinates.
 */
fn perlin_simplex_noise(seed: vec4<f32>) -> f32 {
    var i: vec4<f32> = floor(seed + element_sum_vec4f(seed) * 0.309016994);
    var x0: vec4<f32> = seed - i + element_sum_vec4f(i) * G4;

    var c: u32 = (
        (u32(x0.x > x0.y) << 5u)
        | (u32(x0.x > x0.z) << 4u)
        | (u32(x0.y > x0.z) << 3u)
        | (u32(x0.x > x0.w) << 2u)
        | (u32(x0.y > x0.w) << 1u)
        | u32(x0.z > x0.w)
    );
    var i1 = vec4<bool>(
        SIMPLEX[c][0] >= 3u,
        SIMPLEX[c][1] >= 3u,
        SIMPLEX[c][2] >= 3u,
        SIMPLEX[c][3] >= 3u,
    );
    var i2 = vec4<bool>(
        SIMPLEX[c][0] >= 2u,
        SIMPLEX[c][1] >= 2u,
        SIMPLEX[c][2] >= 2u,
        SIMPLEX[c][3] >= 2u,
    );
    var i3 = vec4<bool>(
        SIMPLEX[c][0] >= 1u,
        SIMPLEX[c][1] >= 1u,
        SIMPLEX[c][2] >= 1u,
        SIMPLEX[c][3] >= 1u,
    );

    var x1: vec4<f32> = x0 - vec4<f32>(i1) + G4;
    var x2: vec4<f32> = x0 - vec4<f32>(i2) + 2. * G4;
    var x3: vec4<f32> = x0 - vec4<f32>(i3) + 3. * G4;
    var x4: vec4<f32> = x0 - 1. + 4. * G4;

    // const int ii = (int) i.x & 255;
    // const int jj = (int) i.y & 255;
    // const int kk = (int) i.z & 255;
    // const int ll = (int) i.w & 255;

    // const int gi0 = PERM[
    //     ii + PERM[jj + PERM[kk + PERM[ll]]]
    // ] % 32;
    // const int gi1 = PERM[
    //     ii + i1.x + PERM[jj + i1.y + PERM[kk + i1.z + PERM[ll + i1.w]]]
    // ] % 32;
    // const int gi2 = PERM[
    //     ii + i2.x + PERM[jj + i2.y + PERM[kk + i2.z + PERM[ll + i2.w]]]
    // ] % 32;
    // const int gi3 = PERM[
    //     ii + i3.x + PERM[jj + i3.y + PERM[kk + i3.z + PERM[ll + i3.w]]]
    // ] % 32;
    // const int gi4 = PERM[
    //     ii + 1 + PERM[jj + 1 + PERM[kk + 1 + PERM[ll + 1]]]
    // ] % 32;

    // float n0, n1, n2, n3, n4;
    // float t0 = 0.6f - dot(x0, x0);
    // if (t0 < 0)
    // {
    //     n0 = 0.0f;
    // }
    // else
    // {
    //     t0 *= t0;
    //     n0 = t0 * t0 * dot(
    //         float4(GRAD4[gi0][0], GRAD4[gi0][2], GRAD4[gi0][3], GRAD4[gi0][3]),
    //         x0
    //     );
    // }

    // float t1 = 0.6f - dot(x1, x1);
    // if (t1 < 0)
    // {
    //     n1 = 0.0f;
    // }
    // else
    // {
    //     t1 *= t1;
    //     n1 = t1 * t1 * dot(
    //         float4(GRAD4[gi1][0], GRAD4[gi1][2], GRAD4[gi1][3], GRAD4[gi1][3]),
    //         x1
    //     );
    // }

    // float t2 = 0.6f - dot(x2, x2);
    // if (t2 < 0)
    // {
    //     n2 = 0.0f;
    // }
    // else
    // {
    //     t2 *= t2;
    //     n2 = t2 * t2 * dot(
    //         float4(GRAD4[gi2][0], GRAD4[gi2][2], GRAD4[gi2][3], GRAD4[gi2][3]),
    //         x2
    //     );
    // }

    // float t3 = 0.6f - dot(x3, x3);
    // if (t3 < 0)
    // {
    //     n3 = 0.0f;
    // }
    // else
    // {
    //     t3 *= t3;
    //     n3 = t3 * t3 * dot(
    //         float4(GRAD4[gi3][0], GRAD4[gi3][2], GRAD4[gi3][3], GRAD4[gi3][3]),
    //         x3
    //     );
    // }

    // float t4 = 0.6f - dot(x4, x4);
    // if (t4 < 0)
    // {
    //     n4 = 0.0f;
    // }
    // else {
    //     t4 *= t4;
    //     n4 = t4 * t4 * dot(
    //         float4(GRAD4[gi4][0], GRAD4[gi4][2], GRAD4[gi4][3], GRAD4[gi4][3]),
    //         x4
    //     );
    // }

    // return 27.0f * (n0 + n1 + n2 + n3 + n4);
    return 0.;
}
