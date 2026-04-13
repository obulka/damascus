// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;
use macro_rules_attribute::derive;

use crate::{
    EnumTraits,
    app::Context,
    icons::Icons,
    widgets::{Widget, style::Style},
};

#[derive(Default, EnumTraits!)]
pub enum PreferenceMessage {
    #[default]
    Style,
}
