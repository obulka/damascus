// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt;

use super::{
    super::{
        inputs::{input::Input, InputId},
        outputs::{output::Output, OutputId},
        NodeGraph,
    },
    node_data::NodeData,
    NodeId,
};

use crate::Error;

#[derive(Debug, Clone)]
pub struct NodeDoesNotContainInputError {
    node_id: NodeId,
    input_label: String,
}

type Result<T> = std::result::Result<T, NodeDoesNotContainInputError>;

impl NodeDoesNotContainInputError {
    pub fn new(node_id: NodeId, input_label: &str) -> Self {
        Self {
            node_id: node_id,
            input_label: input_label.to_owned(),
        }
    }
}

impl fmt::Display for NodeDoesNotContainInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Node: {:?} does not contain an input named: {:?}",
            self.node_id, self.input_label,
        )
    }
}

impl Error for NodeDoesNotContainInputError {}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub label: String,
    pub inputs: Vec<(String, InputId)>,
    pub outputs: Vec<(String, OutputId)>,
    pub data: NodeData,
}

impl Node {
    pub fn inputs<'a>(&'a self, graph: &'a NodeGraph) -> impl Iterator<Item = &'a Input> + 'a {
        self.input_ids().map(|id| graph.get_input(id))
    }

    pub fn outputs<'a>(&'a self, graph: &'a NodeGraph) -> impl Iterator<Item = &'a Output> + 'a {
        self.output_ids().map(|id| graph.get_output(id))
    }

    pub fn input_ids(&self) -> impl Iterator<Item = InputId> + '_ {
        self.inputs.iter().map(|(_label, id)| *id)
    }

    pub fn output_ids(&self) -> impl Iterator<Item = OutputId> + '_ {
        self.outputs.iter().map(|(_label, id)| *id)
    }

    pub fn get_input(&self, input_label: &str) -> Result<InputId> {
        self.inputs
            .iter()
            .find(|(label, _id)| label == input_label)
            .map(|input| input.1)
            .ok_or_else(|| NodeDoesNotContainInputError::new(self.id, input_label))
    }

    pub fn get_output(&self, output_label: &str) -> Result<OutputId> {
        self.outputs
            .iter()
            .find(|(label, _id)| label == output_label)
            .map(|output| output.1)
            .ok_or_else(|| NodeDoesNotContainInputError::new(self.id, output_label))
    }
}
