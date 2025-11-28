// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

@group(UNIFORM_BIND_GROUP) @binding(VIEWER_GRADE_BINDING)
var<uniform> _viewer_grade: Grade;

@group(TEXTURE_BIND_GROUP) @binding(TEXTURE_BINDING)
var _texture: texture_2d<f32>;
