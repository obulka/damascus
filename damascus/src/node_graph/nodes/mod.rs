// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt;

use slotmap::SlotMap;
use strum::{EnumCount, EnumIter, EnumString};

use crate::{Enumerator, Errors};

use super::inputs::InputErrors;

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
    InputError(InputErrors),
    InvalidData {
        node_id: NodeId,
        input_data: String,
    },
    NoOutputError(NodeId),
    #[default]
    UnknownError,
}

pub type NodeResult<T> = std::result::Result<T, NodeErrors>;

impl fmt::Display for NodeErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InputError(error) => error.fmt(formatter),
            Self::InvalidData {
                node_id,
                input_data,
            } => write!(
                formatter,
                "{}: Node({:?}) should contain data for input {:?}",
                self, node_id, input_data
            ),
            Self::NoOutputError(node_id) => {
                write!(formatter, "{}: No output on Node({:?})", self, node_id)
            }
            Self::UnknownError => write!(formatter, "{}: Skill issue tbh", self),
        }
    }
}

impl Enumerator for NodeErrors {}
impl Errors for NodeErrors {}
