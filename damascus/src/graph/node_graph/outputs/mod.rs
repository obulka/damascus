// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt;

use slotmap::SlotMap;

pub mod output;
pub mod output_data;

use output::Output;

slotmap::new_key_type! { pub struct OutputId; }

impl fmt::Display for OutputId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub type Outputs = SlotMap<OutputId, Output>;
