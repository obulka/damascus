// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt;

use slotmap::SlotMap;
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{Enumerator, Errors};

pub mod input;
pub mod input_data;

use input::Input;
use input_data::InputData;

slotmap::new_key_type! { pub struct InputId; }

pub type Inputs = SlotMap<InputId, Input>;

#[derive(
    Debug, Default, Clone, EnumCount, EnumIter, EnumString, serde::Serialize, serde::Deserialize,
)]
pub enum InputErrors {
    InputDowncastError {
        data: InputData,
        conversion_to: String,
    },
    #[default]
    UnknownError,
}

pub type InputResult<T> = std::result::Result<T, InputErrors>;

impl fmt::Display for InputErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputErrors::InputDowncastError {
                data,
                conversion_to,
            } => write!(
                f,
                "{}: Invalid cast from input data of type: {:?} to: {:?}",
                self, data, conversion_to,
            ),
            InputErrors::UnknownError => write!(f, "{}: Skill issue tbh", self),
        }
    }
}

impl Enumerator for InputErrors {}
impl Errors for InputErrors {}
