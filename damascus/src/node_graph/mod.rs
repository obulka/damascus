// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::{HashMap, HashSet};

use quick_cache::{
    unsync::{Cache, DefaultLifecycle},
    DefaultHashBuilder, OptionsBuilder, UnitWeighter,
};

pub mod edges;
pub mod inputs;
pub mod nodes;
pub mod outputs;

use edges::Edges;
use inputs::{input::Input, input_data::InputData, InputId, Inputs};
use nodes::{node::Node, node_data::NodeData, NodeId, Nodes};
use outputs::{output::Output, output_data::OutputData, OutputId, Outputs};

pub type OutputCache = Cache<OutputId, InputData>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NodeGraph {
    pub nodes: Nodes,
    pub inputs: Inputs,
    pub outputs: Outputs,
    pub edges: Edges,
    #[serde(skip)]
    pub cache: OutputCache,
}

impl NodeGraph {
    pub fn new() -> Self {
        Self {
            cache: OutputCache::with_options(
                OptionsBuilder::new()
                    .estimated_items_capacity(10000)
                    .weight_capacity(10000)
                    .build()
                    .unwrap(),
                UnitWeighter,
                DefaultHashBuilder::default(),
                DefaultLifecycle::default(),
            ),
            ..Default::default()
        }
    }

    pub fn valid_edge(&self, input_id: InputId, output_id: OutputId) -> bool {
        let input_node_id: NodeId = self[input_id].node_id;
        let output_node_id: NodeId = self[output_id].node_id;

        input_node_id != output_node_id
            && self
                .ancestor_node_ids(output_node_id)
                .get(&input_node_id)
                .is_none()
    }

    pub fn add_node(&mut self, data: NodeData) -> NodeId {
        self.nodes.insert_with_key(|node_id| Node {
            id: node_id,
            input_ids: vec![],
            output_ids: vec![],
            data: data,
        })
    }

    pub fn add_input(&mut self, node_id: NodeId, data: InputData) -> InputId {
        let input_id = self
            .inputs
            .insert_with_key(|input_id| Input::new(input_id, node_id, data));
        self[node_id].input_ids.push(input_id);
        input_id
    }

    pub fn remove_input(&mut self, input_id: InputId) {
        let node_id = self[input_id].node_id;
        self[node_id].input_ids.retain(|id| *id != input_id);
        self.inputs.remove(input_id);
        self.edges.remove_input(input_id);
    }

    pub fn remove_output(&mut self, output_id: OutputId) {
        let node_id = self[output_id].node_id;
        self[node_id].output_ids.retain(|id| *id != output_id);
        self.outputs.remove(output_id);
        self.edges.remove_output(output_id);
    }

    pub fn add_output(&mut self, node_id: NodeId, data: OutputData) -> OutputId {
        let output_id = self
            .outputs
            .insert_with_key(|output_id| Output::new(output_id, node_id, data));
        self[node_id].output_ids.push(output_id);
        output_id
    }

    pub fn remove_node(&mut self, node_id: NodeId) -> (Node, Vec<(InputId, OutputId)>) {
        let mut disconnected_edges = vec![];

        disconnected_edges.extend(
            self.edges
                .disconnect_inputs(self[node_id].input_ids.clone()),
        );
        disconnected_edges.extend(
            self.edges
                .disconnect_outputs(self[node_id].output_ids.clone()),
        );

        for input in self[node_id].input_ids.clone().iter() {
            self.inputs.remove(*input);
        }
        for output in self[node_id].output_ids.clone().iter() {
            self.outputs.remove(*output);
        }
        let removed_node = self.nodes.remove(node_id).expect("Node must exist.");

        (removed_node, disconnected_edges)
    }

    pub fn try_get_parent(&self, input_id: InputId) -> Option<&OutputId> {
        self.edges.parent(input_id)
    }

    pub fn try_get_children(&self, output_id: OutputId) -> Option<&HashSet<InputId>> {
        self.edges.children(output_id)
    }

    pub fn try_get_input(&self, input_id: InputId) -> Option<&Input> {
        self.inputs.get(input_id)
    }

    pub fn get_input(&self, input_id: InputId) -> &Input {
        &self[input_id]
    }

    pub fn try_get_output(&self, output_id: OutputId) -> Option<&Output> {
        self.outputs.get(output_id)
    }

    pub fn get_output(&self, output_id: OutputId) -> &Output {
        &self[output_id]
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.keys()
    }

    pub fn descendant_output_ids(&self, node_id: NodeId) -> HashSet<OutputId> {
        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        let mut output_ids = HashSet::<OutputId>::new();
        while let Some(search_node_id) = nodes_to_search.pop() {
            output_ids.extend(self[search_node_id].output_ids.iter().map(|output_id| {
                if let Some(input_ids) = self.edges.children(*output_id) {
                    nodes_to_search
                        .extend(input_ids.iter().map(|input_id| self[*input_id].node_id));
                }
                output_id
            }));
        }
        output_ids
    }

    pub fn descendant_node_ids(&self, node_id: NodeId) -> HashSet<NodeId> {
        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        let mut descendant_ids = HashSet::<NodeId>::new();
        while let Some(search_node_id) = nodes_to_search.pop() {
            self[search_node_id]
                .output_ids
                .iter()
                .for_each(|output_id| {
                    if let Some(input_ids) = self.edges.children(*output_id) {
                        nodes_to_search
                            .extend(input_ids.iter().map(|input_id| self[*input_id].node_id));
                        descendant_ids
                            .extend(input_ids.iter().map(|input_id| self[*input_id].node_id));
                    }
                });
        }
        descendant_ids
    }

    pub fn ancestor_node_ids(&self, node_id: NodeId) -> HashSet<NodeId> {
        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        let mut ancestor_ids = HashSet::<NodeId>::new();
        while let Some(search_node_id) = nodes_to_search.pop() {
            self[search_node_id].input_ids.iter().for_each(|input_id| {
                if let Some(parent_output_id) = self.edges.parent(*input_id) {
                    nodes_to_search.push(self[*parent_output_id].node_id);
                    ancestor_ids.insert(self[*parent_output_id].node_id);
                }
            });
        }
        ancestor_ids
    }

    pub fn merge(&mut self, other: &mut Self) -> HashMap<NodeId, NodeId> {
        let mut other_to_new_node_ids = HashMap::<NodeId, NodeId>::new();
        let mut edges_to_recreate = HashMap::<OutputId, HashSet<InputId>>::new();
        let mut other_to_new_outputs = HashMap::<OutputId, OutputId>::new();
        for node_id in self.iter_nodes().collect::<HashSet<_>>().into_iter() {
            if let Some(mut other_node) = other.nodes.remove(node_id) {
                // Move the node to this node graph and update its id
                let new_node_id: NodeId = self.nodes.insert_with_key(|new_node_id| {
                    other_node.id = new_node_id;
                    other_node
                });

                // Update the nodes inputs with new ids, and the new node's id
                let mut new_inputs: Vec<InputId> = self[new_node_id].input_ids.clone();
                for input_id in new_inputs.iter_mut() {
                    if let Some(mut input) = other.inputs.remove(*input_id) {
                        input.node_id = new_node_id;
                        let new_id = self.inputs.insert_with_key(|new_id| {
                            input.id = new_id;
                            input
                        });
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
                        let new_id = self.outputs.insert_with_key(|new_id| {
                            output.id = new_id;
                            output
                        });
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
                    self.edges.insert(*new_input_id, *new_output_id);
                }
            }
        }

        other_to_new_node_ids
    }

    pub fn from_nodes(&self, node_ids: &HashSet<NodeId>) -> Self {
        let mut new_graph: Self = self.clone();

        for node_id in self.iter_nodes() {
            if node_ids.contains(&node_id) {
                continue;
            }
            let (_node, disconnected_edges) = new_graph.remove_node(node_id);
            for (input_id, _output_id) in disconnected_edges.iter() {
                new_graph.edges.remove_input(*input_id);
            }
        }

        new_graph
    }
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! impl_index_traits {
    ($id_type:ty, $output_type:ty, $arena:ident) => {
        impl std::ops::Index<$id_type> for NodeGraph {
            type Output = $output_type;

            fn index(&self, index: $id_type) -> &Self::Output {
                self.$arena.get(index).unwrap_or_else(|| {
                    panic!("{} index error for {:?}", stringify!($id_type), index)
                })
            }
        }

        impl std::ops::IndexMut<$id_type> for NodeGraph {
            fn index_mut(&mut self, index: $id_type) -> &mut Self::Output {
                self.$arena.get_mut(index).unwrap_or_else(|| {
                    panic!("{} index error for {:?}", stringify!($id_type), index)
                })
            }
        }
    };
}

impl_index_traits!(NodeId, Node, nodes);
impl_index_traits!(InputId, Input, inputs);
impl_index_traits!(OutputId, Output, outputs);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let mut node_graph = NodeGraph::new();

        let axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let axis_input: InputId = node_graph.add_input(axis_id, InputData::Mat4(Mat4::IDENTITY));
        let axis_output: OutputId = node_graph.add_output(axis_id, OutputData::Mat4);

        assert_eq!(true, true);
        assert!(true);
    }
}
