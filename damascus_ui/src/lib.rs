// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]
#![allow(long_running_const_eval)]

use macro_rules_attribute::derive_alias;

use damascus::Enumerator;

// pub mod app;
pub mod icons;
pub mod style;
// pub mod widgets;

// pub use app::Damascus;

// pub const MAX_TEXTURE_DIMENSION: u32 = 8192;
// pub const MAX_BUFFER_SIZE: usize = 1024 << 20; // (1Gb)

derive_alias! {
    #[derive(EnumBaseTraits!)] = #[derive(
        Debug,
        Clone,
        strum::EnumIter,
        strum::EnumCount,
        strum::EnumString,
        PartialEq,
        serde::Serialize,
        serde::Deserialize,
        damascus::EnumTrait!,
    )];
    #[derive(EnumTraits!)] = #[derive(
        strum::Display,
        crate::EnumBaseTraits!,
    )];
}
