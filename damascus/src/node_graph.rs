// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::{HashMap, HashSet};

use quick_cache::{
    unsync::{Cache, DefaultLifecycle},
    DefaultHashBuilder, OptionsBuilder, UnitWeighter,
};

use crate::impl_slot_map_indexing;

pub mod edges;
pub mod inputs;
pub mod nodes;
pub mod outputs;

use edges::Edges;
use inputs::{
    input::Input,
    input_data::{InputData, NodeInputData},
    InputId, Inputs,
};
use nodes::{node::Node, node_data::NodeData, NodeErrors, NodeId, NodeResult, Nodes};
use outputs::{
    output::Output,
    output_data::{NodeOutputData, OutputData},
    OutputId, Outputs,
};


pub trait Graph {
    pub fn new() -> Self;

    pub fn node_count(&self) -> usize;

    pub fn edge_count(&self) -> usize;

    pub fn clear(&mut self);

    pub fn new_from_nodes(&self, node_ids: &HashSet<NodeId>) -> Self {
        let mut new_graph: Self = self.clone();

        for node_id in self.iter_nodes() {
            if node_ids.contains(&node_id) {
                continue;
            }
            let (_node, disconnected_edges) = new_graph.remove_node(node_id);
            for (input_id, _output_id) in disconnected_edges.iter() {
                new_graph.edges.disconnect_input(*input_id);
            }
        }

        new_graph
    }

    pub fn is_valid_edge(&self, output_id: OutputId, input_id: InputId) -> bool {
        let input_node_id: NodeId = self[input_id].node_id;
        let output_node_id: NodeId = self[output_id].node_id;

        input_node_id != output_node_id
            && self[output_id]
                .data
                .can_connect_to_input(&self[input_id].data)
            && !self.is_ancestor(output_node_id, input_node_id)
    }

    pub fn add_node(&mut self, node_data: NodeData) -> NodeId {
        let node_id: NodeId = self.nodes.insert(Node::new(node_data));
        node_data.add_to_graph(self, node_id);
        node_id
    }

    pub fn remove_node(&mut self, node_id: NodeId) -> (Node, HashMap<InputId, OutputId>) {
        let mut disconnected_edges = HashMap::<InputId, OutputId>::new();

        let input_ids: Vec<InputId> = self[node_id].input_ids.clone();
        let output_ids: Vec<OutputId> = self[node_id].output_ids.clone();

        disconnected_edges.extend(self.edges.disconnect_inputs(input_ids.iter()));
        disconnected_edges.extend(self.edges.disconnect_outputs(output_ids.iter()));

        for input in input_ids.iter() {
            self.inputs.remove(*input);
        }
        for output in output_ids.iter() {
            self.outputs.remove(*output);
        }
        let removed_node = self.nodes.remove(node_id).expect("Node must exist.");

        (removed_node, disconnected_edges)
    }

    pub fn children(&self, node_id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self[node_id]
            .output_ids
            .iter()
            .filter_map(|output_id| self.edges.children(*output_id))
            .flat_map(|input_ids| input_ids.iter().map(|input_id| self[*input_id].node_id))
    }

    pub fn parents(&self, node_id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self[node_id]
            .input_ids
            .iter()
            .filter_map(|input_id| self.edges.parent(*input_id))
            .map(|output_id| self[*output_id].node_id)
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.keys()
    }

    pub fn descendants_output_ids(&self, node_id: NodeId) -> Vec<OutputId> {
        self.descendants(node_id)
            .iter()
            .flat_map(|descendant_id| {
                self[*descendant_id]
                    .output_ids
                    .iter()
                    .map(|output_id| *output_id)
            })
            .collect()
    }

    /// Get all ancestor nodes of `node_id` without using recursion
    pub fn for_each_descendant<B, F>(&self, node_id: NodeId, closure: F) -> Vec<B>
    where
        F: Fn(NodeId) -> B,
    {
        let mut result: Vec<B> = vec![];
        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        while let Some(search_node_id) = nodes_to_search.pop() {
            result.extend(self.children(search_node_id).map(|descendant_id| {
                nodes_to_search.push(descendant_id);
                closure(descendant_id)
            }));
        }
        result
    }

    /// Check if a node is an ancestor of another
    pub fn is_descendant(&self, node_id: NodeId, potential_descendant_id: NodeId) -> bool {
        // Nodes are not their own descendant
        if node_id == potential_descendant_id {
            return false;
        }

        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        while let Some(search_node_id) = nodes_to_search.pop() {
            for descendant_id in self.children(search_node_id) {
                if descendant_id == potential_descendant_id {
                    return true;
                }
                nodes_to_search.push(descendant_id);
            }
        }
        false
    }

    /// Get all descendant nodes of `node_id` without using recursion
    pub fn descendants(&self, node_id: NodeId) -> Vec<NodeId> {
        self.for_each_descendant(node_id, |descendant_id| descendant_id)
    }

    /// Get all ancestor nodes of `node_id` without using recursion
    pub fn for_each_ancestor<B, F>(&self, node_id: NodeId, closure: F) -> Vec<B>
    where
        F: Fn(NodeId) -> B,
    {
        let mut result: Vec<B> = vec![];
        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        while let Some(search_node_id) = nodes_to_search.pop() {
            result.extend(self.parents(search_node_id).map(|ancestor_id| {
                nodes_to_search.push(ancestor_id);
                closure(ancestor_id)
            }));
        }
        result
    }

    /// Get all ancestor nodes of `node_id` without using recursion
    pub fn ancestors(&self, node_id: NodeId) -> Vec<NodeId> {
        self.for_each_ancestor(node_id, |ancestor_id| ancestor_id)
    }

    /// Check if a node is an ancestor of another
    pub fn is_ancestor(&self, node_id: NodeId, potential_ancestor_id: NodeId) -> bool {
        // Nodes are not their own ancestor
        if node_id == potential_ancestor_id {
            return false;
        }

        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        while let Some(search_node_id) = nodes_to_search.pop() {
            for ancestor_id in self.parents(search_node_id) {
                if ancestor_id == potential_ancestor_id {
                    return true;
                }
                nodes_to_search.push(ancestor_id);
            }
        }
        false
    }

    /// Merge two node graphs together
    ///
    /// All nodes and edges will be moved into this graph without cloning,
    /// and their ids will be updated to avoid clashing
    ///
    /// Returns: A map of the former to new node ids
    pub fn merge(&mut self, other: &mut Self) -> HashMap<NodeId, NodeId> {
        let mut other_to_new_node_ids = HashMap::<NodeId, NodeId>::new();
        let mut edges_to_recreate = HashMap::<OutputId, HashSet<InputId>>::new();
        let mut other_to_new_outputs = HashMap::<OutputId, OutputId>::new();

        for node_id in self.iter_nodes().collect::<HashSet<NodeId>>().into_iter() {
            if let Some(other_node) = other.nodes.remove(node_id) {
                // Move the node to this node graph and update its id
                let new_node_id: NodeId = self.nodes.insert(other_node);

                // Update the nodes inputs with new ids, and the new node's id
                let mut new_inputs: Vec<InputId> = self[new_node_id].input_ids.clone();
                for input_id in new_inputs.iter_mut() {
                    if let Some(mut input) = other.inputs.remove(*input_id) {
                        input.node_id = new_node_id;
                        let new_id = self.inputs.insert(input);
                        if let Some(output_id) = other.edges.parent(*input_id) {
                            // Maintain a list of edges to duplicate
                            if let Some(inputs) = edges_to_recreate.get_mut(output_id) {
                                inputs.insert(new_id);
                            } else {
                                let mut inputs = HashSet::<InputId>::new();
                                inputs.insert(new_id);
                                edges_to_recreate.insert(*output_id, inputs);
                            }
                        }
                        *input_id = new_id;
                    }
                }

                // Update the outputs with new ids, and the new node's id
                let mut new_outputs: Vec<OutputId> = self[new_node_id].output_ids.clone();
                for output_id in new_outputs.iter_mut() {
                    if let Some(mut output) = other.outputs.remove(*output_id) {
                        output.node_id = new_node_id;
                        let new_id = self.outputs.insert(output);
                        // Maintain a LUT of the original to new ids
                        other_to_new_outputs.insert(*output_id, new_id);
                        *output_id = new_id;
                    }
                }

                self[new_node_id].input_ids = new_inputs;
                self[new_node_id].output_ids = new_outputs;
                other_to_new_node_ids.insert(node_id, new_node_id);
            }
        }

        // Form equivalent edges
        for (other_output_id, new_input_ids) in edges_to_recreate.iter() {
            if let Some(new_output_id) = other_to_new_outputs.get(other_output_id) {
                for new_input_id in new_input_ids.iter() {
                    self.edges.connect(*new_output_id, *new_input_id);
                }
            }
        }

        other_to_new_node_ids
    }

    pub fn disconnect_node_input<I: NodeInputData>(
        &mut self,
        node_id: NodeId,
        node_input_data: I,
    ) -> Option<OutputId> {
        match self.node_input_id(node_id, node_input_data) {
            Ok(input_id) => self.edges.disconnect_input(input_id),
            Err(_) => None,
        }
    }

    pub fn input_is_connected(&self, input_id: InputId) -> bool {
        self.edges.parent(input_id).is_some()
    }

    pub fn node_input_is_connected<I: NodeInputData>(
        &self,
        node_id: NodeId,
        node_input_data: I,
    ) -> bool {
        match self.node_input_id(node_id, node_input_data) {
            Ok(input_id) => self.input_is_connected(input_id),
            Err(_) => false,
        }
    }

    pub fn output_is_connected(&self, output_id: OutputId) -> bool {
        if let Some(children) = self.edges.children(output_id) {
            return !children.is_empty();
        }
        false
    }

    pub fn node_output_is_connected(&self, node_id: NodeId) -> bool {
        if let Some(output_id) = self.node_first_output_id(node_id) {
            return self.output_is_connected(*output_id);
        }
        false
    }
}
