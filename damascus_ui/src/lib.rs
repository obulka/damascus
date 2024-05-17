// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]
#![allow(long_running_const_eval)]

mod app;
mod panels;
mod toolbar;
mod widgets;

pub use app::Damascus;
