// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt;

use slotmap::SlotMap;
use strum::{EnumCount, EnumIter, EnumString};

use crate::{Enumerator, Errors};

pub mod input;
pub mod input_data;

use input::Input;
use input_data::{InputData, NodeInputData};

slotmap::new_key_type! { pub struct InputId; }

pub type Inputs = SlotMap<InputId, Input>;

#[derive(
    Debug,
    Default,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum InputErrors {
    InputDowncastError {
        data: InputData,
        conversion_to: String,
    },
    InputDoesNotExistError(String),
    InputDataDoesNotExistError(String),
    #[default]
    UnknownError,
}

pub type InputResult<T> = std::result::Result<T, InputErrors>;

impl fmt::Display for InputErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InputDowncastError {
                data,
                conversion_to,
            } => write!(
                formatter,
                "{}: Invalid cast from input data of type: {:?} to: {:?}",
                self, data, conversion_to,
            ),
            Self::InputDoesNotExistError(name) => {
                write!(formatter, "{}: No input named: {:?}", self, name,)
            }
            Self::InputDataDoesNotExistError(name) => write!(
                formatter,
                "{}: No data received for an input named: {:?}",
                self, name,
            ),
            Self::UnknownError => write!(formatter, "{}: Skill issue tbh", self),
        }
    }
}

impl Enumerator for InputErrors {}
impl Errors for InputErrors {}
