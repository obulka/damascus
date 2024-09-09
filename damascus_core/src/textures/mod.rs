// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Texture {
    pub dimensions: u32,
    pub filepath: Box<std::path::Path>,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            dimensions: 4,
            filepath: std::path::Path::new("").into(),
        }
    }
}
