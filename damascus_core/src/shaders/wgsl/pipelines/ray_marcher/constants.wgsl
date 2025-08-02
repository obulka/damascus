// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

// Bind groups
const UNIFORM_BIND_GROUP: u32 = 0u;
const STORAGE_BIND_GROUP: u32 = 1u;
// const TEXTURE_BIND_GROUP: u32 = 2u;
const STORAGE_TEXTURE_BIND_GROUP: u32 = 2u;

// Bindings
const RENDER_PARAMETERS_BINDING: u32 = 0u;
const SCENE_PARAMETERS_BINDING: u32 = 1u;
const RENDER_STATE_BINDING: u32 = 2u;
const RENDER_CAMERA_BINDING: u32 = 3u;
const PROGRESSIVE_RENDERING_TEXTURE_BINDING: u32 = 0u;
const PRIMITIVES_BINDING: u32 = 0u;
const LIGHTS_BINDING: u32 = 1u;
const ATMOSPHERE_BINDING: u32 = 2u;
const EMISSIVE_INDICES_BINDING: u32 = 3u;

// Interstage variable locations
const VERTEX_UV_LOCATION: u32 = 0u;
const TEXTURE_UV_LOCATION: u32 = 0u;
const PIXEL_COLOUR_LOCATION: u32 = 0u;