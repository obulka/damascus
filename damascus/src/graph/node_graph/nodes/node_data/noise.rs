// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

// input_float(
//     graph,
//     "gain",
//     Float::new(default_procedural_texture.grade.gain)
//         .with_ui_data(
//             UIData::default()
//                 .with_tooltip("The gain to apply to the texture colour.")
//                 .with_hidden(),
//         )
//         .with_range(0.0001..=10.),
// );
// input_uint(
//     graph,
//     "octaves",
//     UnsignedInteger::new(default_procedural_texture.octaves)
//         .with_ui_data(
//             UIData::default()
//                 .with_tooltip("The number of different frequencies to superimpose.")
//                 .with_hidden(),
//         )
//         .with_range(1..=12),
// );
// input_float(
//     graph,
//     "lacunarity",
//     Float::new(default_procedural_texture.lacunarity)
//         .with_ui_data(
//             UIData::default()
//                 .with_tooltip(indoc! {
//                     "The lacunarity is the initial frequency of the noise,
//                     and the amount to scale the frequency for each octave."
//                 })
//                 .with_hidden(),
//         )
//         .with_range(1.0..=10.),
// );
// input_float(
//     graph,
//     "amplitude_gain",
//     Float::new(default_procedural_texture.amplitude_gain)
//         .with_ui_data(
//             UIData::default()
//                 .with_tooltip(indoc! {
//                     "The gain to apply to the texture. This scales the
//                     noise amplitude between octaves."
//                 })
//                 .with_hidden(),
//         )
//         .with_range(0.0..=2.),
// );
// input_float(
//     graph,
//     "gamma",
//     Float::new(default_procedural_texture.grade.gamma)
//         .with_ui_data(
//             UIData::default()
//                 .with_tooltip(indoc! {
//                     "The gamma to apply to the texture. This is computed
//                     by raising the colour to the power of 1/gamma."
//                 })
//                 .with_hidden(),
//         )
//         .with_range(0.01..=2.),
// );
// input_vector4(
//     graph,
//     "low_frequency_scale",
//     Vec4::new(default_procedural_texture.low_frequency_scale).with_ui_data(
//         UIData::default()
//             .with_tooltip("The amount to scale lower frequencies between octaves.")
//             .with_hidden(),
//     ),
// );
// input_vector4(
//     graph,
//     "high_frequency_scale",
//     Vec4::new(default_procedural_texture.high_frequency_scale).with_ui_data(
//         UIData::default()
//             .with_tooltip("The amount to scale higher frequencies between octaves.")
//             .with_hidden(),
//     ),
// );
// input_vector4(
//     graph,
//     "low_frequency_translation",
//     Vec4::new(default_procedural_texture.low_frequency_translation).with_ui_data(
//         UIData::default()
//             .with_tooltip(
//                 "The amount to translate lower frequencies between octaves.",
//             )
//             .with_hidden(),
//     ),
// );
// input_vector4(
//     graph,
//     "high_frequency_translation",
//     Vec4::new(default_procedural_texture.high_frequency_translation).with_ui_data(
//         UIData::default()
//             .with_tooltip(
//                 "The amount to translate higher frequencies between octaves.",
//             )
//             .with_hidden(),
//     ),
// );
// input_bool(
//     graph,
//     "invert",
//     Bool::new(default_procedural_texture.grade.invert).with_ui_data(
//         UIData::default()
//             .with_tooltip("Invert the colour.")
//             .with_hidden(),
//     ),
// );
