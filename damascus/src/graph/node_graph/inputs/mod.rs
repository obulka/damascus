// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt;

use slotmap::SlotMap;

pub mod input;
pub mod input_data;

use input::Input;

slotmap::new_key_type! { pub struct InputId; }

impl fmt::Display for InputId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub type Inputs = SlotMap<InputId, Input>;
