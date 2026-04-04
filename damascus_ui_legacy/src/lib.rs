// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]
#![allow(long_running_const_eval)]

pub mod app;
pub mod icons;
pub mod widgets;

pub use app::Damascus;

pub const MAX_TEXTURE_DIMENSION: u32 = 8192;
pub const MAX_BUFFER_SIZE: usize = 1024 << 20; // (1Gb)
