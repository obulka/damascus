// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    fmt,
    fs::File,
    io::{BufReader, Read, Write},
    str::FromStr,
};

use iced;
use macro_rules_attribute::derive;
use strum::IntoEnumIterator;

use damascus::Enumerator;

use crate::{
    EnumTraits, ErrorTraits,
    app::Context,
    widgets::{Widget, style},
};

#[derive(Default, EnumTraits!)]
pub enum PreferenceMessage {
    #[default]
    Settings,
}
