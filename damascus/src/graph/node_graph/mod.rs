// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::{HashMap, HashSet};

use slotmap::SparseSecondaryMap;

use super::{
    BidirectedGraph,
    edges::{BidirectedEdges, SingleParentBidirectedEdges},
    scene_graph::SceneGraph,
};
use crate::impl_slot_map_indexing;

pub mod inputs;
pub mod nodes;
pub mod outputs;

use inputs::{
    InputId, Inputs,
    input::Input,
    input_data::{InputData, NodeInputData},
};
use nodes::{NodeErrors, NodeId, NodeResult, Nodes, node::Node, node_data::NodeData};
use outputs::{
    OutputId, Outputs,
    output::Output,
    output_data::{NodeOutputData, OutputData},
};

pub type OutputCache = SparseSecondaryMap<OutputId, InputData>;
pub type Edges = SingleParentBidirectedEdges<OutputId, InputId>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NodeGraph {
    nodes: Nodes,
    inputs: Inputs,
    outputs: Outputs,
    edges: Edges,
    #[serde(skip)]
    scene_graph: SceneGraph,
    #[serde(skip)]
    cache: OutputCache,
}

impl BidirectedGraph<NodeId> for NodeGraph {
    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn clear(&mut self) {
        self.clear_cache();

        self.nodes.clear();
        self.inputs.clear();
        self.outputs.clear();
        self.edges.clear();
    }

    fn iter_children<'a>(&'a self, node_id: &'a NodeId) -> impl Iterator<Item = &'a NodeId> + 'a
    where
        NodeId: 'a,
    {
        self[*node_id]
            .output_ids
            .iter()
            .flat_map(|output_id| self.edges.iter_children(output_id))
            .map(|input_id| &self[*input_id].node_id)
    }

    fn iter_parents<'a>(&'a self, node_id: &'a NodeId) -> impl Iterator<Item = &'a NodeId> + 'a
    where
        NodeId: 'a,
    {
        self[*node_id]
            .input_ids
            .iter()
            .flat_map(|input_id| self.edges.iter_parents(input_id))
            .map(|output_id| &self[*output_id].node_id)
    }
}

impl NodeGraph {
    pub fn new() -> Self {
        Self {
            nodes: Nodes::default(),
            inputs: Inputs::default(),
            outputs: Outputs::default(),
            edges: Edges::default(),
            scene_graph: SceneGraph::default(),
            cache: OutputCache::default(),
        }
    }

    pub fn scene_graph(&self) -> &SceneGraph {
        &self.scene_graph
    }

    pub fn scene_graph_mut(&mut self) -> &mut SceneGraph {
        &mut self.scene_graph
    }

    pub fn iter(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.keys()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.parent_count()
    }

    pub fn input_count(&self) -> usize {
        self.inputs.len()
    }

    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }

    pub fn remove_output_from_cache(&mut self, output_id: &OutputId) -> Option<InputData> {
        if let Some(input_data) = self.cache.remove(*output_id) {
            match input_data {
                InputData::SceneGraphId(scene_graph_id) => {
                    self.scene_graph.remove(scene_graph_id);
                }
                _ => {}
            }
            Some(input_data)
        } else {
            None
        }
    }

    pub fn remove_node_from_cache(&mut self, node_id: NodeId) {
        let node_output_ids: Vec<OutputId> = self[node_id].output_ids.clone();
        self.descendants_output_ids(node_id)
            .iter()
            .chain(node_output_ids.iter())
            .for_each(|output_id| {
                self.remove_output_from_cache(output_id);
            });
    }

    pub fn insert_in_cache(&mut self, output_id: OutputId, input_data: InputData) {
        self.cache.insert(output_id, input_data);
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.scene_graph.clear();
    }

    fn evaluate_node(
        &mut self,
        output_id: OutputId,
        input_data_map: HashMap<String, InputData>,
    ) -> NodeResult<InputData> {
        let node_data: NodeData = self[self[output_id].node_id].data;
        let output_name: String = self[output_id].name.clone();
        let input_data: InputData = Node::evaluate(
            &mut self.scene_graph,
            node_data,
            input_data_map,
            output_name,
        )?;

        self.insert_in_cache(output_id, input_data.clone());

        Ok(input_data)
    }

    // Evaluate the input value of a node
    pub fn evaluate_output(&mut self, output_id: OutputId) -> NodeResult<InputData> {
        // The output depends on the data from each of the node's inputs
        // so iterate over the inputs and collect their data
        let input_ids: Vec<InputId> = self[self[output_id].node_id].input_ids.clone();

        // We will collect that input data in this map as we ascend the graph
        let mut all_input_data_for_node = HashMap::<String, InputData>::new();

        for input_id in input_ids.into_iter() {
            // Recursively retrieve the data for this input
            let result: NodeResult<InputData> = self.evaluate_input(input_id);

            if let Ok(input_data) = result {
                // If the data was valid, store it for the node to process
                all_input_data_for_node.insert(self[input_id].name.clone(), input_data);
                continue;
            }

            // If an error has occured, bail and propogate it
            return result;
        }

        // All input data for the node has been collected
        // so its time to process the data and start descending the graph
        self.evaluate_node(output_id, all_input_data_for_node)
    }

    pub fn evaluate_input(&mut self, input_id: InputId) -> NodeResult<InputData> {
        if let Some(output_id) = self.edges.parent(&input_id) {
            if let Some(input_data) = self.cache.get(*output_id) {
                // Data was already cached, return it
                Ok((*input_data).clone())
            } else {
                self.evaluate_output(*output_id)
            }
        } else {
            // Input is not connected
            // Return its current/default value
            Ok(self[input_id].data.clone())
        }
    }

    pub fn new_from_nodes(&self, node_ids: &HashSet<NodeId>) -> Self {
        let mut new_graph: Self = self.clone();

        new_graph.clear_cache();

        for node_id in self.iter() {
            if node_ids.contains(&node_id) {
                continue;
            }
            let (_node, disconnected_edges) = new_graph.remove_node(node_id);
            disconnected_edges.iter().for_each(|(output_id, input_id)| {
                new_graph.edges.disconnect(output_id, input_id);
            })
        }

        new_graph
    }

    pub fn is_valid_edge(&self, output_id: OutputId, input_id: InputId) -> bool {
        let input_node_id: NodeId = self[input_id].node_id;
        let output_node_id: NodeId = self[output_id].node_id;

        input_node_id != output_node_id
            && self[input_node_id]
                .data
                .output_compatible_with_input(&self[output_id].data, &self[input_id].name)
            && !self.is_ancestor(&output_node_id, &input_node_id)
    }

    pub fn add_node(&mut self, node_data: NodeData) -> NodeId {
        let node_id: NodeId = self.nodes.insert(Node::new(node_data));
        node_data.add_to_node_graph(self, node_id);
        node_id
    }

    pub fn remove_node(&mut self, node_id: NodeId) -> (Node, HashMap<OutputId, InputId>) {
        let mut disconnected_edges = HashMap::<OutputId, InputId>::new();

        let input_ids: Vec<InputId> = self[node_id].input_ids.clone();
        let output_ids: Vec<OutputId> = self[node_id].output_ids.clone();

        for input in input_ids.iter() {
            self.inputs.remove(*input);
        }
        for output in output_ids.iter() {
            self.outputs.remove(*output);
        }

        disconnected_edges.extend(
            self.edges
                .disconnect_parents_of_children(input_ids.iter())
                .map(|(output_id, input_id)| (output_id, *input_id)),
        );
        disconnected_edges.extend(
            self.edges
                .disconnect_children_of_parents(output_ids.iter())
                .map(|(output_id, input_id)| (*output_id, input_id)),
        );
        let removed_node = self.nodes.remove(node_id).expect("Node must exist.");

        (removed_node, disconnected_edges)
    }

    pub fn add_input(&mut self, node_id: NodeId, name: &str, data: InputData) -> InputId {
        let input_id = self
            .inputs
            .insert(Input::new(node_id, name.to_string(), data));
        self[node_id].input_ids.push(input_id);
        input_id
    }

    pub fn insert_input(
        &mut self,
        node_id: NodeId,
        name: &str,
        data: InputData,
        index: usize,
    ) -> InputId {
        let input_id = self
            .inputs
            .insert(Input::new(node_id, name.to_string(), data));
        self[node_id].input_ids.insert(index, input_id);
        input_id
    }

    pub fn remove_input(&mut self, input_id: InputId) -> Vec<OutputId> {
        let node_id = self[input_id].node_id;
        self[node_id].input_ids.retain(|id| *id != input_id);
        self.inputs.remove(input_id);
        self.edges.disconnect_parents_of_child(&input_id).collect()
    }

    pub fn add_output(&mut self, node_id: NodeId, name: &str, data: OutputData) -> OutputId {
        let output_id = self
            .outputs
            .insert(Output::new(node_id, name.to_string(), data));
        self[node_id].output_ids.push(output_id);
        output_id
    }

    pub fn remove_output(&mut self, output_id: OutputId) -> Vec<(OutputId, InputId)> {
        let node_id = self[output_id].node_id;
        self[node_id].output_ids.retain(|id| *id != output_id);
        self.outputs.remove(output_id);
        self.edges
            .disconnect_children_of_parent(&output_id)
            .map(|input_id| (output_id, input_id))
            .collect()
    }

    pub fn descendants_output_ids(&self, node_id: NodeId) -> Vec<OutputId> {
        self.descendants(&node_id)
            .iter()
            .flat_map(|descendant_id| {
                self[**descendant_id]
                    .output_ids
                    .iter()
                    .map(|output_id| *output_id)
            })
            .collect()
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

        for node_id in self.iter().collect::<HashSet<NodeId>>().into_iter() {
            if let Some(other_node) = other.nodes.remove(node_id) {
                // Move the node to this node graph and update its id
                let new_node_id: NodeId = self.nodes.insert(other_node);

                // Update the nodes inputs with new ids, and the new node's id
                let mut new_inputs: Vec<InputId> = self[new_node_id].input_ids.clone();
                for input_id in new_inputs.iter_mut() {
                    if let Some(mut input) = other.inputs.remove(*input_id) {
                        input.node_id = new_node_id;
                        let new_id = self.inputs.insert(input);
                        if let Some(output_id) = other.edges.parent(input_id) {
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

    pub fn input_index(&self, node_id: NodeId, input_id: InputId) -> usize {
        self[node_id]
            .input_ids
            .iter()
            .position(|&id| id == input_id)
            .unwrap()
    }

    pub fn node_inputs(&self, node_id: NodeId) -> impl Iterator<Item = (&InputId, &Input)> {
        self[node_id]
            .input_ids
            .iter()
            .map(|input_id| (input_id, &self[*input_id]))
    }

    pub fn node_outputs(&self, node_id: NodeId) -> impl Iterator<Item = (&OutputId, &Output)> {
        self[node_id]
            .output_ids
            .iter()
            .map(|output_id| (output_id, &self[*output_id]))
    }

    pub fn node_input_from_str(&self, node_id: NodeId, name: &str) -> NodeResult<&Input> {
        self.node_inputs(node_id)
            .find(|(_input_id, input)| input.name == name)
            .map(|(_input_id, input)| input)
            .ok_or_else(|| NodeErrors::InputDoesNotExistError(name.to_string()))
    }

    pub fn node_input_id_from_str(&self, node_id: NodeId, name: &str) -> NodeResult<InputId> {
        self.node_inputs(node_id)
            .find(|(_input_id, input)| input.name == name)
            .map(|(input_id, _input)| *input_id)
            .ok_or_else(|| NodeErrors::InputDoesNotExistError(name.to_string()))
    }

    pub fn node_output_from_str(&self, node_id: NodeId, name: &str) -> NodeResult<&Output> {
        self.node_outputs(node_id)
            .find(|(_output_id, output)| output.name == name)
            .map(|(_output_id, output)| output)
            .ok_or_else(|| NodeErrors::OutputDoesNotExistError(name.to_string()))
    }

    pub fn node_output_id_from_str(&self, node_id: NodeId, name: &str) -> NodeResult<OutputId> {
        self.node_outputs(node_id)
            .find(|(_output_id, output)| output.name == name)
            .map(|(output_id, _output)| *output_id)
            .ok_or_else(|| NodeErrors::OutputDoesNotExistError(name.to_string()))
    }

    pub fn node_input_id<I: NodeInputData>(
        &self,
        node_id: NodeId,
        node_input_data: I,
    ) -> NodeResult<InputId> {
        self.node_inputs(node_id)
            .find(|(_input_id, input)| input.name == node_input_data.name())
            .map(|(input_id, _input)| *input_id)
            .ok_or_else(|| NodeErrors::InputDoesNotExistError(node_input_data.name()))
    }

    pub fn node_output_id<O: NodeOutputData>(
        &self,
        node_id: NodeId,
        node_output_data: O,
    ) -> NodeResult<OutputId> {
        self.node_outputs(node_id)
            .find(|(_output_id, output)| output.name == node_output_data.name())
            .map(|(output_id, _output)| *output_id)
            .ok_or_else(|| NodeErrors::OutputDoesNotExistError(node_output_data.name()))
    }

    pub fn node_first_input(&self, node_id: NodeId) -> Option<&Input> {
        self.node_inputs(node_id)
            .map(|(_input_id, input)| input)
            .next()
    }

    pub fn node_first_input_id(&self, node_id: NodeId) -> Option<&InputId> {
        self[node_id].input_ids.iter().next()
    }

    pub fn node_first_output(&self, node_id: NodeId) -> Option<&Output> {
        self.node_outputs(node_id)
            .map(|(_output_id, output)| output)
            .next()
    }

    pub fn node_first_output_id(&self, node_id: NodeId) -> Option<&OutputId> {
        self[node_id].output_ids.iter().next()
    }

    pub fn connect_output_to_input(&mut self, output_id: OutputId, input_id: InputId) -> bool {
        if !self.is_valid_edge(output_id, input_id) || !self.edges.connect(output_id, input_id) {
            return false;
        }

        let node_id: NodeId = self[input_id].node_id;

        let node_data: NodeData = self[node_id].data;
        node_data.dynamic_input_connected(self, input_id);

        self.remove_node_from_cache(node_id);

        true
    }

    pub fn connect_node_to_input(&mut self, node_id: NodeId, input_id: InputId) -> bool {
        if let Some(output_id) = self.node_first_output_id(node_id) {
            self.connect_output_to_input(*output_id, input_id)
        } else {
            false
        }
    }

    pub fn disconnect_input_id(&mut self, input_id: &InputId) -> Option<OutputId> {
        if let Some(output_id) = self.edges.disconnect_parent_of_child(input_id) {
            let node_id: NodeId = self[*input_id].node_id;
            let node_data: NodeData = self[node_id].data;
            node_data.dynamic_input_disconnected(self, *input_id);

            self.remove_node_from_cache(node_id);

            Some(output_id)
        } else {
            None
        }
    }

    pub fn disconnect_named_node_input<'a>(
        &'a mut self,
        node_id: NodeId,
        input_name: &'a str,
    ) -> Option<(OutputId, InputId)> {
        if let Ok(input_id) = self.node_input_id_from_str(node_id, input_name)
            && let Some(output_id) = self.disconnect_input_id(&input_id)
        {
            Some((output_id, input_id))
        } else {
            None
        }
    }

    pub fn disconnect_node_input<I: NodeInputData>(
        &mut self,
        node_id: NodeId,
        node_input_data: I,
    ) -> Option<(OutputId, InputId)> {
        self.disconnect_named_node_input(node_id, &node_input_data.name())
    }

    pub fn input_is_connected(&self, input_id: InputId) -> bool {
        self.edges.parent(&input_id).is_some()
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
        self.edges.has_child(&output_id)
    }

    pub fn node_output_is_connected(&self, node_id: NodeId) -> bool {
        if let Some(output_id) = self.node_first_output_id(node_id) {
            return self.output_is_connected(*output_id);
        }
        false
    }
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl_slot_map_indexing!(NodeGraph, NodeId, Node, nodes);
impl_slot_map_indexing!(NodeGraph, InputId, Input, inputs);
impl_slot_map_indexing!(NodeGraph, OutputId, Output, outputs);

#[cfg(test)]
mod tests {
    use glam::*;
    use strum::{EnumCount, IntoEnumIterator};
    use wgpu;

    use super::{inputs::input_data::InputData, nodes::node_data::*, *};

    use crate::{
        geometry::primitives::Shapes,
        gpu::get_device_queue_encoder,
        gpu::resources::TextureResource,
        textures::evaluators::{
            GPUTextureEvaluator, TextureEvaluators, grade::Grade, view::TextureViewer,
        },
    };

    #[test]
    fn test_node_creation() {
        let mut graph = NodeGraph::new();

        let mut node_ids = HashSet::<NodeId>::new();

        for node_data in NodeData::iter() {
            node_ids.insert(graph.add_node(node_data));
        }

        assert_eq!(graph.node_count(), NodeData::COUNT);
        assert_eq!(graph.edge_count(), 0);
        assert_eq!(node_ids, graph.iter().collect::<HashSet<NodeId>>());
    }

    #[test]
    fn test_node_deletion() {
        let mut graph = NodeGraph::new();

        for node_data in NodeData::iter() {
            graph.add_node(node_data);
        }

        for node_id in graph.iter().collect::<Vec<NodeId>>() {
            let (_node, disconnections) = graph.remove_node(node_id);
            assert!(disconnections.is_empty());
        }

        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);

        for node_data in NodeData::iter() {
            graph.add_node(node_data);
        }

        assert_eq!(graph.node_count(), NodeData::COUNT);

        graph.clear();

        assert_eq!(graph.node_count(), 0);
    }

    #[test]
    fn test_node_edge_connection() {
        let mut graph = NodeGraph::new();

        let primary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let camera_id: NodeId = graph.add_node(NodeData::Camera);

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 0);

        graph.connect_node_to_input(
            primary_axis_id,
            graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .unwrap(),
        );

        assert_eq!(graph.edge_count(), 1);
        assert_eq!(
            graph.iter_children(&primary_axis_id).next(),
            Some(secondary_axis_id).as_ref()
        );

        graph.connect_node_to_input(
            secondary_axis_id,
            graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .unwrap(),
        );

        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_node_edge_connection_string() {
        let mut graph = NodeGraph::new();

        let primary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let camera_id: NodeId = graph.add_node(NodeData::Camera);

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 0);

        graph.connect_node_to_input(
            primary_axis_id,
            graph
                .node_input_id_from_str(secondary_axis_id, "Axis")
                .unwrap(),
        );

        assert_eq!(graph.edge_count(), 1);

        graph.connect_node_to_input(
            secondary_axis_id,
            graph.node_input_id_from_str(camera_id, "Axis").unwrap(),
        );

        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_node_edge_disconnection() {
        let mut graph = NodeGraph::new();

        let primary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let camera_id: NodeId = graph.add_node(NodeData::Camera);

        graph.connect_node_to_input(
            primary_axis_id,
            graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            secondary_axis_id,
            graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .unwrap(),
        );

        assert_eq!(graph.edge_count(), 2);

        graph.disconnect_node_input(secondary_axis_id, AxisInputData::Axis);

        assert_eq!(graph.edge_count(), 1);

        graph.disconnect_node_input(camera_id, CameraInputData::Axis);

        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_node_edge_disconnection_on_node_removal() {
        let mut graph = NodeGraph::new();

        let primary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let camera_id: NodeId = graph.add_node(NodeData::Camera);

        let primitive0_id: NodeId = graph.add_node(NodeData::Primitive);
        let primitive1_id: NodeId = graph.add_node(NodeData::Primitive);

        let primary_axis_output_id: OutputId =
            *graph.node_first_output_id(primary_axis_id).unwrap();

        let secondary_axis_output_id: OutputId =
            *graph.node_first_output_id(secondary_axis_id).unwrap();
        let secondary_axis_axis_input_id: InputId = graph
            .node_input_id(secondary_axis_id, AxisInputData::Axis)
            .unwrap();

        let camera_axis_input_id: InputId = graph
            .node_input_id(camera_id, CameraInputData::Axis)
            .unwrap();

        let primitive_axis_input_id: InputId = graph
            .node_input_id(primitive0_id, PrimitiveInputData::Axis)
            .unwrap();

        graph.connect_node_to_input(
            primary_axis_id,
            graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            secondary_axis_id,
            graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            secondary_axis_id,
            graph
                .node_input_id(primitive0_id, PrimitiveInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            primitive0_id,
            graph
                .node_input_id(primitive1_id, PrimitiveInputData::Child)
                .unwrap(),
        );

        let mut expected_disconnections = HashMap::<OutputId, InputId>::new();
        expected_disconnections.insert(primary_axis_output_id, secondary_axis_axis_input_id);
        expected_disconnections.insert(secondary_axis_output_id, camera_axis_input_id);
        expected_disconnections.insert(secondary_axis_output_id, primitive_axis_input_id);

        let (_node, disconnections) = graph.remove_node(secondary_axis_id);
        assert_eq!(disconnections, expected_disconnections);
    }

    #[test]
    fn test_valid_edge() {
        let mut graph = NodeGraph::new();

        let primary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let camera_id: NodeId = graph.add_node(NodeData::Camera);

        let primary_axis_output_id: OutputId =
            *graph.node_first_output_id(primary_axis_id).unwrap();
        let primary_axis_axis_input_id: InputId = graph
            .node_input_id(primary_axis_id, AxisInputData::Axis)
            .unwrap();

        let secondary_axis_output_id: OutputId =
            *graph.node_first_output_id(secondary_axis_id).unwrap();
        let secondary_axis_axis_input_id: InputId = graph
            .node_input_id(secondary_axis_id, AxisInputData::Axis)
            .unwrap();

        let camera_output_id: OutputId = *graph.node_first_output_id(camera_id).unwrap();

        assert!(graph.is_valid_edge(primary_axis_output_id, secondary_axis_axis_input_id));
        assert!(!graph.is_valid_edge(primary_axis_output_id, primary_axis_axis_input_id));
        assert!(graph.is_valid_edge(secondary_axis_output_id, primary_axis_axis_input_id));
        assert!(!graph.is_valid_edge(camera_output_id, primary_axis_axis_input_id));

        graph.connect_node_to_input(
            primary_axis_id,
            graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            secondary_axis_id,
            graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .unwrap(),
        );

        assert!(!graph.is_valid_edge(secondary_axis_output_id, primary_axis_axis_input_id));
    }

    #[test]
    fn test_node_ancestors() {
        let mut graph = NodeGraph::new();

        let primary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let camera_id: NodeId = graph.add_node(NodeData::Camera);

        let primitive0_id: NodeId = graph.add_node(NodeData::Primitive);
        let primitive1_id: NodeId = graph.add_node(NodeData::Primitive);

        graph.connect_node_to_input(
            primary_axis_id,
            graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            secondary_axis_id,
            graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            secondary_axis_id,
            graph
                .node_input_id(primitive0_id, PrimitiveInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            primitive0_id,
            graph
                .node_input_id(primitive1_id, PrimitiveInputData::Child)
                .unwrap(),
        );

        let mut camera_ancestors = Vec::<&NodeId>::new();
        camera_ancestors.push(&secondary_axis_id);
        camera_ancestors.push(&primary_axis_id);

        assert_eq!(graph.ancestors(&camera_id), camera_ancestors);

        let mut secondary_axis_ancestors = Vec::<&NodeId>::new();
        secondary_axis_ancestors.push(&primary_axis_id);

        assert_eq!(
            graph.ancestors(&secondary_axis_id),
            secondary_axis_ancestors
        );

        assert!(graph.ancestors(&primary_axis_id).is_empty());

        let mut primitive1_ancestors = Vec::<&NodeId>::new();
        primitive1_ancestors.push(&primitive0_id);
        primitive1_ancestors.push(&secondary_axis_id);
        primitive1_ancestors.push(&primary_axis_id);

        assert_eq!(graph.ancestors(&primitive1_id), primitive1_ancestors);
    }

    #[test]
    fn test_node_descendants() {
        let mut graph = NodeGraph::new();

        let primary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let camera_id: NodeId = graph.add_node(NodeData::Camera);

        let primitive0_id: NodeId = graph.add_node(NodeData::Primitive);
        let primitive1_id: NodeId = graph.add_node(NodeData::Primitive);

        graph.connect_node_to_input(
            primary_axis_id,
            graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            secondary_axis_id,
            graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            secondary_axis_id,
            graph
                .node_input_id(primitive0_id, PrimitiveInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            primitive0_id,
            graph
                .node_input_id(primitive1_id, PrimitiveInputData::Child)
                .unwrap(),
        );

        assert!(graph.descendants(&camera_id).is_empty());
        assert!(graph.descendants(&primitive1_id).is_empty());
        assert!(graph.descendants_output_ids(camera_id).is_empty());
        assert!(graph.descendants_output_ids(primitive1_id).is_empty());

        let mut secondary_axis_descendants = HashSet::<NodeId>::new();
        secondary_axis_descendants.insert(camera_id);
        secondary_axis_descendants.insert(primitive0_id);
        secondary_axis_descendants.insert(primitive1_id);

        assert_eq!(
            graph
                .descendants(&secondary_axis_id)
                .into_iter()
                .copied()
                .collect::<HashSet<NodeId>>(),
            secondary_axis_descendants,
        );

        let mut secondary_axis_descendants_output_ids = HashSet::<OutputId>::new();
        secondary_axis_descendants_output_ids.extend(graph[camera_id].output_ids.iter());
        secondary_axis_descendants_output_ids.extend(graph[primitive0_id].output_ids.iter());
        secondary_axis_descendants_output_ids.extend(graph[primitive1_id].output_ids.iter());

        assert_eq!(
            graph
                .descendants_output_ids(secondary_axis_id)
                .into_iter()
                .collect::<HashSet<OutputId>>(),
            secondary_axis_descendants_output_ids,
        );

        let mut primary_axis_descendants = HashSet::<NodeId>::new();
        primary_axis_descendants.insert(secondary_axis_id);
        primary_axis_descendants.insert(camera_id);
        primary_axis_descendants.insert(primitive0_id);
        primary_axis_descendants.insert(primitive1_id);

        assert_eq!(
            graph
                .descendants(&primary_axis_id)
                .into_iter()
                .copied()
                .collect::<HashSet<NodeId>>(),
            primary_axis_descendants,
        );

        let mut primary_axis_descendants_output_ids = HashSet::<OutputId>::new();
        primary_axis_descendants_output_ids.extend(graph[secondary_axis_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(graph[camera_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(graph[primitive0_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(graph[primitive1_id].output_ids.iter());

        assert_eq!(
            graph
                .descendants_output_ids(primary_axis_id)
                .into_iter()
                .collect::<HashSet<OutputId>>(),
            primary_axis_descendants_output_ids,
        );

        assert_eq!(graph.descendants(&primitive0_id), vec![&primitive1_id],);

        assert_eq!(
            graph.descendants_output_ids(primitive0_id),
            graph[primitive1_id].output_ids,
        );
        assert_eq!(
            graph.descendants_output_ids(primitive0_id),
            graph[primitive1_id].output_ids,
        );
    }

    #[test]
    fn test_graph_merge() {
        let mut graph = NodeGraph::new();
        let mut graph1 = NodeGraph::new();

        let mut node_ids = HashSet::<NodeId>::new();
        let mut node_ids1 = HashSet::<NodeId>::new();

        for node_data in NodeData::iter() {
            node_ids.insert(graph.add_node(node_data));
            node_ids1.insert(graph1.add_node(node_data));
        }

        let node_id_lut: HashMap<NodeId, NodeId> = graph.merge(&mut graph1);

        assert_eq!(graph.node_count(), NodeData::COUNT * 2);
        assert_eq!(graph.edge_count(), 0);
        assert_eq!(graph1.nodes.len(), 0);
        assert_eq!(
            node_ids1,
            node_id_lut
                .keys()
                .map(|node_id| *node_id)
                .collect::<HashSet<NodeId>>(),
        );
        node_ids.extend(node_id_lut.values());
        assert_eq!(node_ids, graph.iter().collect::<HashSet<NodeId>>(),);
    }

    #[test]
    fn test_graph_merge_edges() {
        let mut graphs = vec![NodeGraph::new(), NodeGraph::new()];

        let mut primary_axis_ids = Vec::<NodeId>::new();
        let mut secondary_axis_ids = Vec::<NodeId>::new();
        let mut camera_ids = Vec::<NodeId>::new();
        let mut primitive0_ids = Vec::<NodeId>::new();
        let mut primitive1_ids = Vec::<NodeId>::new();

        graphs.iter_mut().for_each(|graph| {
            let primary_axis_id: NodeId = graph.add_node(NodeData::Axis);
            let secondary_axis_id: NodeId = graph.add_node(NodeData::Axis);
            let camera_id: NodeId = graph.add_node(NodeData::Camera);
            let primitive0_id: NodeId = graph.add_node(NodeData::Primitive);
            let primitive1_id: NodeId = graph.add_node(NodeData::Primitive);

            graph.connect_node_to_input(
                primary_axis_id,
                graph
                    .node_input_id(secondary_axis_id, AxisInputData::Axis)
                    .unwrap(),
            );

            graph.connect_node_to_input(
                secondary_axis_id,
                graph
                    .node_input_id(camera_id, CameraInputData::Axis)
                    .unwrap(),
            );

            graph.connect_node_to_input(
                secondary_axis_id,
                graph
                    .node_input_id(primitive0_id, PrimitiveInputData::Axis)
                    .unwrap(),
            );

            graph.connect_node_to_input(
                primitive0_id,
                graph
                    .node_input_id(primitive1_id, PrimitiveInputData::Child)
                    .unwrap(),
            );

            primary_axis_ids.push(primary_axis_id);
            secondary_axis_ids.push(secondary_axis_id);
            camera_ids.push(camera_id);
            primitive0_ids.push(primitive0_id);
            primitive1_ids.push(primitive1_id);
        });

        let mut graph: NodeGraph = graphs.pop().unwrap();

        let node_id_lut: HashMap<NodeId, NodeId> = graphs[0].merge(&mut graph);

        let primary_axis_id: NodeId = *node_id_lut.get(&primary_axis_ids[1]).unwrap();
        let secondary_axis_id: NodeId = *node_id_lut.get(&secondary_axis_ids[1]).unwrap();
        let camera_id: NodeId = *node_id_lut.get(&camera_ids[1]).unwrap();
        let primitive0_id: NodeId = *node_id_lut.get(&primitive0_ids[1]).unwrap();
        let primitive1_id: NodeId = *node_id_lut.get(&primitive1_ids[1]).unwrap();

        assert!(graphs[0].descendants(&camera_id).is_empty());
        assert!(graphs[0].descendants(&primitive1_id).is_empty());
        assert!(graphs[0].descendants_output_ids(camera_id).is_empty());
        assert!(graphs[0].descendants_output_ids(primitive1_id).is_empty());

        let mut secondary_axis_descendants = HashSet::<NodeId>::new();
        secondary_axis_descendants.insert(camera_id);
        secondary_axis_descendants.insert(primitive0_id);
        secondary_axis_descendants.insert(primitive1_id);

        assert_eq!(
            graphs[0]
                .descendants(&secondary_axis_id)
                .into_iter()
                .copied()
                .collect::<HashSet<NodeId>>(),
            secondary_axis_descendants,
        );

        let mut secondary_axis_descendants_output_ids = HashSet::<OutputId>::new();
        secondary_axis_descendants_output_ids.extend(graphs[0][camera_id].output_ids.iter());
        secondary_axis_descendants_output_ids.extend(graphs[0][primitive0_id].output_ids.iter());
        secondary_axis_descendants_output_ids.extend(graphs[0][primitive1_id].output_ids.iter());

        assert_eq!(
            graphs[0]
                .descendants_output_ids(secondary_axis_id)
                .into_iter()
                .collect::<HashSet<OutputId>>(),
            secondary_axis_descendants_output_ids,
        );

        let mut primary_axis_descendants = HashSet::<NodeId>::new();
        primary_axis_descendants.insert(secondary_axis_id);
        primary_axis_descendants.insert(camera_id);
        primary_axis_descendants.insert(primitive0_id);
        primary_axis_descendants.insert(primitive1_id);

        assert_eq!(
            graphs[0]
                .descendants(&primary_axis_id)
                .into_iter()
                .copied()
                .collect::<HashSet<NodeId>>(),
            primary_axis_descendants,
        );

        let mut primary_axis_descendants_output_ids = HashSet::<OutputId>::new();
        primary_axis_descendants_output_ids.extend(graphs[0][secondary_axis_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(graphs[0][camera_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(graphs[0][primitive0_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(graphs[0][primitive1_id].output_ids.iter());

        assert_eq!(
            graphs[0]
                .descendants_output_ids(primary_axis_id)
                .into_iter()
                .collect::<HashSet<OutputId>>(),
            primary_axis_descendants_output_ids,
        );

        assert_eq!(graphs[0].descendants(&primitive0_id), vec![&primitive1_id],);
        assert_eq!(
            graphs[0].descendants_output_ids(primitive0_id),
            graphs[0][primitive1_id].output_ids,
        );
    }

    #[test]
    fn test_new_from_nodes() {
        let mut graph = NodeGraph::new();

        let primary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let camera_id: NodeId = graph.add_node(NodeData::Camera);

        let primitive0_id: NodeId = graph.add_node(NodeData::Primitive);
        let primitive1_id: NodeId = graph.add_node(NodeData::Primitive);

        graph.connect_node_to_input(
            primary_axis_id,
            graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            secondary_axis_id,
            graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            secondary_axis_id,
            graph
                .node_input_id(primitive0_id, PrimitiveInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            primitive0_id,
            graph
                .node_input_id(primitive1_id, PrimitiveInputData::Child)
                .unwrap(),
        );

        let mut node_ids = HashSet::<NodeId>::new();
        node_ids.insert(primary_axis_id);
        node_ids.insert(secondary_axis_id);

        let new_graph = graph.new_from_nodes(&node_ids);

        assert_eq!(new_graph.iter().collect::<HashSet<NodeId>>(), node_ids,);
        assert_eq!(new_graph.descendants(&primary_axis_id).len(), 1);
        assert_eq!(
            new_graph.descendants(&primary_axis_id).pop(),
            Some(&secondary_axis_id)
        );
        assert!(new_graph.node_output_is_connected(primary_axis_id));
        assert!(!new_graph.node_output_is_connected(secondary_axis_id));
        assert!(!new_graph.node_input_is_connected(primary_axis_id, AxisInputData::Axis));
        assert!(new_graph.node_input_is_connected(secondary_axis_id, AxisInputData::Axis));
    }

    #[test]
    fn test_axis_evaluation() {
        let mut graph = NodeGraph::new();

        let primary_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = graph.add_node(NodeData::Axis);

        assert!(!graph.is_descendant(&primary_axis_id, &secondary_axis_id));
        assert!(!graph.is_ancestor(&primary_axis_id, &secondary_axis_id));

        graph.connect_node_to_input(
            primary_axis_id,
            graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .unwrap(),
        );

        assert!(graph.is_descendant(&primary_axis_id, &secondary_axis_id));
        assert!(!graph.is_ancestor(&primary_axis_id, &secondary_axis_id));
        assert!(graph.is_ancestor(&secondary_axis_id, &primary_axis_id));
        assert!(!graph.is_descendant(&secondary_axis_id, &primary_axis_id));

        let primary_axis_translate_input_id: InputId = graph
            .node_input_id(primary_axis_id, AxisInputData::Translate)
            .unwrap();
        let primary_axis_rotate_input_id: InputId = graph
            .node_input_id(primary_axis_id, AxisInputData::Rotate)
            .unwrap();
        let primary_axis_output_id: OutputId =
            *graph.node_first_output_id(primary_axis_id).unwrap();

        let secondary_axis_translate_input_id: InputId = graph
            .node_input_id(secondary_axis_id, AxisInputData::Translate)
            .unwrap();
        let secondary_axis_rotate_input_id: InputId = graph
            .node_input_id(secondary_axis_id, AxisInputData::Rotate)
            .unwrap();
        let secondary_axis_output_id: OutputId =
            *graph.node_first_output_id(secondary_axis_id).unwrap();

        assert_eq!(
            graph.evaluate_output(primary_axis_output_id),
            Ok(InputData::Mat4(Mat4::IDENTITY))
        );
        assert_eq!(
            graph.evaluate_output(secondary_axis_output_id),
            Ok(InputData::Mat4(Mat4::IDENTITY))
        );

        let primary_translation = Vec3::new(1., 2., 3.);
        let secondary_translation = Vec3::new(3., 1., 2.);

        let primary_rotation = Vec3::new(13., 75., 69.);
        let secondary_rotation = Vec3::new(45., 15., 12.);

        let primary_euler_rotation = primary_rotation * std::f32::consts::PI / 180.;
        let secondary_euler_rotation = secondary_rotation * std::f32::consts::PI / 180.;

        let primary_matrix = Mat4::from_rotation_translation(
            Quat::from_euler(
                glam::EulerRot::XYZ,
                primary_euler_rotation.x,
                primary_euler_rotation.y,
                primary_euler_rotation.z,
            ),
            primary_translation,
        );
        let secondary_matrix = Mat4::from_rotation_translation(
            Quat::from_euler(
                glam::EulerRot::XYZ,
                secondary_euler_rotation.x,
                secondary_euler_rotation.y,
                secondary_euler_rotation.z,
            ),
            secondary_translation,
        );

        graph[primary_axis_translate_input_id].data = InputData::Vec3(primary_translation);
        graph[secondary_axis_translate_input_id].data = InputData::Vec3(secondary_translation);

        graph[primary_axis_rotate_input_id].data = InputData::Vec3(primary_rotation);
        graph[secondary_axis_rotate_input_id].data = InputData::Vec3(secondary_rotation);

        assert_eq!(
            graph.evaluate_output(primary_axis_output_id),
            Ok(InputData::Mat4(primary_matrix))
        );
        assert_eq!(
            graph.evaluate_output(secondary_axis_output_id),
            Ok(InputData::Mat4(primary_matrix * secondary_matrix))
        );
    }

    #[macro_rules_attribute::apply(smol_macros::test!)]
    async fn test_ray_marcher() {
        let mut graph = NodeGraph::new();

        let primary_camera_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let secondary_camera_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let camera_id: NodeId = graph.add_node(NodeData::Camera);

        let light_id: NodeId = graph.add_node(NodeData::Light);

        let primitive_id: NodeId = graph.add_node(NodeData::Primitive);
        let primitive_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let primitive_material_id: NodeId = graph.add_node(NodeData::Material);

        let primitive1_id: NodeId = graph.add_node(NodeData::Primitive);
        let primitive1_axis_id: NodeId = graph.add_node(NodeData::Axis);
        let primitive1_material_id: NodeId = graph.add_node(NodeData::Material);

        let primitive2_id: NodeId = graph.add_node(NodeData::Primitive);
        let primitive2_axis_id: NodeId = graph.add_node(NodeData::Axis);

        let scene_id: NodeId = graph.add_node(NodeData::Scene);

        let ray_marcher_id: NodeId = graph.add_node(NodeData::RayMarcher);

        // Connect camera to scene
        // /ray_marcher/scene/camera/secondary_axis/primary_axis

        graph.connect_node_to_input(
            primary_camera_axis_id,
            graph
                .node_input_id(secondary_camera_axis_id, AxisInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            secondary_camera_axis_id,
            graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            camera_id,
            graph
                .node_input_id(scene_id, SceneInputData::RenderCamera)
                .unwrap(),
        );

        // Connect light to scene
        // /ray_marcher/scene/camera/secondary_axis/primary_axis
        // |           |     /light

        graph.connect_node_to_input(
            light_id,
            graph
                .node_input_id(scene_id, SceneInputData::Scene)
                .unwrap(),
        );

        // Connect primitives to scene
        // /ray_marcher/scene/camera/secondary_axis/primary_axis
        // |           |     /light
        // |           |     /primitive/axis
        // |           |     |         /material
        // |           |     |         /primitive2/axis2
        // |           |     |                    /material1
        // |           |     /primitive1/axis1
        // |           |     |          /material1

        graph.connect_node_to_input(
            primitive_axis_id,
            graph
                .node_input_id(primitive_id, PrimitiveInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            primitive_material_id,
            graph
                .node_input_id(primitive_id, PrimitiveInputData::Material)
                .unwrap(),
        );

        graph.connect_node_to_input(
            primitive_id,
            graph.node_input_id_from_str(scene_id, "Scene1").unwrap(),
        );

        graph.connect_node_to_input(
            primitive1_axis_id,
            graph
                .node_input_id(primitive1_id, PrimitiveInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            primitive1_material_id,
            graph
                .node_input_id(primitive1_id, PrimitiveInputData::Material)
                .unwrap(),
        );

        graph.connect_node_to_input(
            primitive1_id,
            graph.node_input_id_from_str(scene_id, "Scene2").unwrap(),
        );

        graph.connect_node_to_input(
            primitive2_axis_id,
            graph
                .node_input_id(primitive2_id, PrimitiveInputData::Axis)
                .unwrap(),
        );

        graph.connect_node_to_input(
            primitive1_material_id,
            graph
                .node_input_id(primitive2_id, PrimitiveInputData::Material)
                .unwrap(),
        );

        graph.connect_node_to_input(
            primitive2_id,
            graph
                .node_input_id(primitive_id, PrimitiveInputData::Child)
                .unwrap(),
        );

        // Connect scene to ray marcher

        graph.connect_node_to_input(
            scene_id,
            graph
                .node_input_id(ray_marcher_id, RayMarcherInputData::SceneRoot)
                .unwrap(),
        );

        // Modify camera data

        let sensor_resolution_input_id: InputId = graph
            .node_input_id(camera_id, CameraInputData::SensorResolution)
            .unwrap();
        graph[sensor_resolution_input_id].data = InputData::UVec2(UVec2::new(2048u32, 1024u32));

        let secondary_camera_axis_translate_input_id: InputId = graph
            .node_input_id(secondary_camera_axis_id, AxisInputData::Translate)
            .unwrap();
        graph[secondary_camera_axis_translate_input_id].data = InputData::Vec3(Vec3::Z * 10.);

        let secondary_camera_axis_translate_input_id: InputId = graph
            .node_input_id(secondary_camera_axis_id, AxisInputData::Translate)
            .unwrap();
        graph[secondary_camera_axis_translate_input_id].data = InputData::Vec3(Vec3::Z * 10.);

        // Modify light data

        let light_colour_input_id: InputId = graph
            .node_input_id(light_id, LightInputData::Colour)
            .unwrap();
        graph[light_colour_input_id].data = InputData::Vec3(Vec3::new(1., 0.1, 0.1));

        // Modify primitive data

        let diffuse_colour_input_id: InputId = graph
            .node_input_id(primitive_material_id, MaterialInputData::DiffuseColour)
            .unwrap();
        graph[diffuse_colour_input_id].data = InputData::Vec3(Vec3::new(0.1, 0.1, 1.));

        let shape_input_id: InputId = graph
            .node_input_id(primitive_id, PrimitiveInputData::Shape)
            .unwrap();
        graph[shape_input_id].data = InputData::Enum(Shapes::Capsule.into());

        let blend_strength_input_id: InputId = graph
            .node_input_id(primitive_id, PrimitiveInputData::BlendStrength)
            .unwrap();
        graph[blend_strength_input_id].data = InputData::Float(0.5);

        let enable_orbit_trap_colour_input_id: InputId = graph
            .node_input_id(primitive_id, PrimitiveInputData::EnableOrbitTrapColour)
            .unwrap();
        graph[enable_orbit_trap_colour_input_id].data = InputData::Bool(true);

        let primitive1_axis_translate_input_id: InputId = graph
            .node_input_id(primitive1_axis_id, AxisInputData::Translate)
            .unwrap();
        graph[primitive1_axis_translate_input_id].data = InputData::Vec3(Vec3::X);

        let primitive2_axis_translate_input_id: InputId = graph
            .node_input_id(primitive2_axis_id, AxisInputData::Translate)
            .unwrap();
        graph[primitive2_axis_translate_input_id].data = InputData::Vec3(-Vec3::X);

        // Modify ray marcher data

        let ray_marcher_output_id: OutputId = *graph.node_first_output_id(ray_marcher_id).unwrap();

        if let Ok(input_data) = graph.evaluate_output(ray_marcher_output_id)
            && let Ok(texture_evaluator_id) = input_data.try_to_texture_evaluator_id()
        {
            // The node graph has produced the data needed to render a ray marching pass
            // test that it was built correctly, then render it on the gpu
            match &graph.scene_graph()[texture_evaluator_id] {
                TextureEvaluators::RayMarcher(ray_marcher) => {
                    assert_eq!(ray_marcher.render_data.gpu_scene.cameras.len(), 2);
                    assert_eq!(ray_marcher.render_data.gpu_scene.render_camera, 1);
                    assert_eq!(
                        ray_marcher.render_data.gpu_scene.cameras
                            [ray_marcher.render_data.gpu_scene.render_camera]
                            .camera_to_world,
                        glam::Mat4::from_translation(Vec3::Z * 10.),
                    );

                    assert_eq!(ray_marcher.render_data.gpu_scene.materials.len(), 3);
                    assert_eq!(ray_marcher.render_data.gpu_scene.primitives.len(), 3);
                    assert_eq!(
                        ray_marcher.render_data.gpu_scene.primitives[0].material_id,
                        1,
                    );
                    assert_eq!(
                        ray_marcher.render_data.gpu_scene.materials[1].diffuse_colour,
                        Vec3::new(0.1, 0.1, 1.),
                    );
                }
                _ => assert!(false),
            }

            if let Ok((device, queue, mut encoder)) = get_device_queue_encoder().await
                && let Some(output_texture_view) =
                    graph.scene_graph()[texture_evaluator_id].create_output_texture_view(&device)
                && graph.scene_graph_mut()[texture_evaluator_id].render_to_texture_view(
                    &device,
                    &queue,
                    &mut encoder,
                    &output_texture_view,
                )
            {
                // ---------------------------------------------------------
                // TODO test a grade pass here on the result then generalize
                // all of this
                // ---------------------------------------------------------

                let mut texture_viewer = TextureEvaluators::TextureViewer(
                    TextureViewer::default()
                        .input_texture_view(output_texture_view.clone())
                        .grade(Grade::default().gain(3.))
                        .finalized(),
                );

                if let Some(viewer_output_texture_view) =
                    texture_viewer.create_output_texture_view(&device)
                    && texture_viewer.render_to_texture_view(
                        &device,
                        &queue,
                        &mut encoder,
                        &viewer_output_texture_view,
                    )
                {
                    // Create a buffer that we can copy the render to

                    let output_buffer: wgpu::Buffer =
                        viewer_output_texture_view.copy_to_buffer(&device, &mut encoder);

                    queue.submit(Some(encoder.finish()));

                    {
                        // Wait for the buffer to be populated with the rendered data

                        let (transmitter, receiver) = smol::channel::bounded(1);

                        let buffer_slice: wgpu::BufferSlice<'_> = output_buffer.slice(..);
                        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                            assert!(transmitter.try_send(result).is_ok());
                        });

                        match device.poll(wgpu::PollType::Wait {
                            submission_index: None, // None for most recent submission
                            timeout: Some(core::time::Duration::new(5, 0)),
                        }) {
                            Err(error) => {
                                println!("{:?}", error);
                                assert!(false);
                            }
                            _ => {}
                        };

                        assert!(receiver.recv().await.is_ok());

                        // Get a read-only view into the buffer
                        let data = buffer_slice.get_mapped_range();

                        // Cast the buffer data into an image and save it to disk

                        let image_buffer = image::Rgba32FImage::from_raw(
                            viewer_output_texture_view.texture_view.texture().width(),
                            viewer_output_texture_view.texture_view.texture().height(),
                            bytemuck::cast_slice::<u8, f32>(&data).to_vec(),
                        )
                        .unwrap();
                        image_buffer.save("image.exr").unwrap();
                    }

                    // Release the buffer back to the GPU
                    output_buffer.unmap();
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }
}
