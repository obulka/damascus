// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use slotmap::SlotMap;

pub mod output;
pub mod output_data;

use output::Output;

slotmap::new_key_type! { pub struct OutputId; }

pub type Outputs = SlotMap<OutputId, Output>;
