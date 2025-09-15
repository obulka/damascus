// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt;

use slotmap::SlotMap;
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{Enumerator, Errors};

pub mod node;
pub mod node_data;

use node::Node;

slotmap::new_key_type! { pub struct NodeId; }

pub type Nodes = SlotMap<NodeId, Node>;

#[derive(
    Debug, Default, Clone, EnumCount, EnumIter, EnumString, serde::Serialize, serde::Deserialize,
)]
pub enum NodeErrors {
    NodeDoesNotContainInputError {
        node_id: NodeId,
        input_name: String,
    },
    NoOutputError(NodeId),
    #[default]
    UnknownError,
}

pub type NodeResult<T> = std::result::Result<T, NodeErrors>;

impl fmt::Display for NodeErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeErrors::NodeDoesNotContainInputError {
                node_id,
                input_name,
            } => write!(
                f,
                "{}: Node({:?}) does not contain an input named: {:?}",
                self, node_id, input_name,
            ),
            NodeErrors::NoOutputError(node_id) => {
                write!(f, "{}: No output on Node({:?})", self, node_id)
            }
            NodeErrors::UnknownError => write!(f, "{}: Skill issue tbh", self),
        }
    }
}

impl Enumerator for NodeErrors {}
impl Errors for NodeErrors {}
