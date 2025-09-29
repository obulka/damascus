// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt;

use slotmap::SlotMap;
use strum::{EnumCount, EnumIter, EnumString};

use crate::{Enumerator, Errors};

use super::inputs::input_data::InputData;

pub mod node;
pub mod node_data;

use node::Node;

slotmap::new_key_type! { pub struct NodeId; }

pub type Nodes = SlotMap<NodeId, Node>;

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
pub enum NodeErrors {
    InputDowncastError {
        data: InputData,
        conversion_to: String,
    },
    InputDoesNotExistError(String),
    InputDataDoesNotExistError(String),
    InvalidData {
        node_id: NodeId,
        input_data: String,
    },
    ParseOutputError(String),
    NotImplementedError,
    #[default]
    UnknownError,
}

pub type NodeResult<T> = std::result::Result<T, NodeErrors>;

impl fmt::Display for NodeErrors {
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
            Self::InvalidData {
                node_id,
                input_data,
            } => write!(
                formatter,
                "{}: Node({:?}) should contain data for input {:?}",
                self, node_id, input_data
            ),
            Self::ParseOutputError(error) => write!(formatter, "{}: {}", self, error),
            Self::UnknownError => write!(formatter, "{}: Skill issue tbh", self),
            _ => write!(formatter, "{}", self),
        }
    }
}

impl Enumerator for NodeErrors {}
impl Errors for NodeErrors {}
