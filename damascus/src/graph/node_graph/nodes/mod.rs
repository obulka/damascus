// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt;

use macro_rules_attribute::derive;
use slotmap::SlotMap;

use crate::{Enumerator, ErrorTraits};

use super::inputs::input_data::InputData;

pub mod node;
pub mod node_data;

use node::Node;

slotmap::new_key_type! { pub struct NodeId; }

pub type Nodes = SlotMap<NodeId, Node>;

#[derive(Default, ErrorTraits!)]
pub enum NodeErrors {
    InvalidCachedData,
    InputDowncastError {
        data: InputData,
        conversion_to: String,
    },
    InputDoesNotExistError(String),
    OutputDoesNotExistError(String),
    InputDataDoesNotExistError(String),
    InvalidData {
        node_id: NodeId,
        input_data: String,
    },
    IncompatibleData {
        node_id: NodeId,
        input_name: String,
        input_data: String,
        expected_input_data: String,
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
                "{}: Invalid cast from input data of type: {:?} to: {}",
                self.variant(),
                data,
                conversion_to,
            ),
            Self::InputDoesNotExistError(name) => {
                write!(formatter, "{}: No input named: '{}'", self.variant(), name)
            }
            Self::OutputDoesNotExistError(name) => {
                write!(formatter, "{}: No output named: '{}'", self.variant(), name)
            }
            Self::InputDataDoesNotExistError(name) => write!(
                formatter,
                "{}: No data received for an input named '{}'",
                self.variant(),
                name,
            ),
            Self::InvalidData {
                node_id,
                input_data,
            } => write!(
                formatter,
                "{}: Node({:?}) should contain data for input '{}'",
                self.variant(),
                node_id,
                input_data,
            ),
            Self::IncompatibleData {
                node_id,
                input_name,
                input_data,
                expected_input_data,
            } => write!(
                formatter,
                "{}: Node({:?}) input '{}' received '{}' but expected '{}'",
                self.variant(),
                node_id,
                input_name,
                input_data,
                expected_input_data,
            ),
            Self::ParseOutputError(error) => write!(formatter, "{}: {}", self.variant(), error),
            Self::UnknownError => write!(formatter, "{}: Skill issue tbh", self.variant()),
            _ => write!(formatter, "{}", self.variant()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, NodeErrors, NodeId, Nodes, node_data::NodeData};

    #[test]
    fn test_node_errors() {
        assert_eq!(
            NodeErrors::InvalidCachedData.to_string(),
            "InvalidCachedData",
        );
        assert_eq!(
            NodeErrors::InputDoesNotExistError("testtest".to_string()).to_string(),
            "InputDoesNotExistError: No input named: 'testtest'",
        );
        assert_eq!(
            NodeErrors::ParseOutputError("testtest".to_string()).to_string(),
            "ParseOutputError: testtest"
        );

        let mut nodes = Nodes::default();
        let node_id: NodeId = nodes.insert(Node::new(NodeData::Axis));
        assert_eq!(
            NodeErrors::IncompatibleData {
                node_id: node_id,
                input_name: "testtest".to_string(),
                input_data: "test test".to_string(),
                expected_input_data: "testing testing".to_string(),
            }
            .to_string(),
            "IncompatibleData: Node(NodeId(1v1)) input 'testtest' received 'test test' but expected 'testing testing'"
        );
    }
}
