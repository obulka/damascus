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
use inputs::{
    input::Input,
    input_data::{
        axis::AxisInputData, camera::CameraInputData, grade::GradeInputData, light::LightInputData,
        material::MaterialInputData, primitive::PrimitiveInputData,
        ray_marcher::RayMarcherInputData, scene::SceneInputData, texture::TextureInputData,
        InputData, NodeInputData,
    },
    InputId, Inputs,
};
use nodes::{node::Node, node_data::NodeData, NodeErrors, NodeId, NodeResult, Nodes};
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
            nodes: Nodes::default(),
            inputs: Inputs::default(),
            outputs: Outputs::default(),
            edges: Edges::default(),
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
        }
    }

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
            && self.ancestors(output_node_id).get(&input_node_id).is_none()
    }

    pub fn add_node(&mut self, data: NodeData) -> NodeId {
        let node_id: NodeId = self.nodes.insert_with_key(|node_id| Node {
            id: node_id,
            input_ids: vec![],
            output_ids: vec![],
            data: data,
        });
        match data {
            NodeData::Axis => {
                AxisInputData::add_to_node(self, node_id);
                self.add_output(node_id, OutputData::Mat4);
            }
            NodeData::Camera => {
                CameraInputData::add_to_node(self, node_id);
                self.add_output(node_id, OutputData::Scene);
            }
            NodeData::Grade => {
                GradeInputData::add_to_node(self, node_id);
                self.add_output(node_id, OutputData::RenderPass);
            }
            NodeData::Light => {
                LightInputData::add_to_node(self, node_id);
                self.add_output(node_id, OutputData::Scene);
            }
            NodeData::Material => {
                MaterialInputData::add_to_node(self, node_id);
                self.add_output(node_id, OutputData::Scene);
            }
            NodeData::Primitive => {
                PrimitiveInputData::add_to_node(self, node_id);
                self.add_output(node_id, OutputData::Scene);
            }
            NodeData::RayMarcher => {
                RayMarcherInputData::add_to_node(self, node_id);
                self.add_output(node_id, OutputData::RenderPass);
            }
            NodeData::Scene => {
                SceneInputData::add_to_node(self, node_id);
                self.add_output(node_id, OutputData::Scene);
            }
            NodeData::Texture => {
                TextureInputData::add_to_node(self, node_id);
                self.add_output(node_id, OutputData::RenderPass);
            }
        }

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

    pub fn add_input(&mut self, node_id: NodeId, name: &str, data: InputData) -> InputId {
        let input_id = self
            .inputs
            .insert_with_key(|input_id| Input::new(input_id, node_id, name.to_string(), data));
        self[node_id].input_ids.push(input_id);
        input_id
    }

    pub fn remove_input(&mut self, input_id: InputId) {
        let node_id = self[input_id].node_id;
        self[node_id].input_ids.retain(|id| *id != input_id);
        self.inputs.remove(input_id);
        self.edges.disconnect_input(input_id);
    }

    pub fn add_output(&mut self, node_id: NodeId, data: OutputData) -> OutputId {
        let output_id = self
            .outputs
            .insert_with_key(|output_id| Output::new(output_id, node_id, data));
        self[node_id].output_ids.push(output_id);
        output_id
    }

    pub fn remove_output(&mut self, output_id: OutputId) {
        let node_id = self[output_id].node_id;
        self[node_id].output_ids.retain(|id| *id != output_id);
        self.outputs.remove(output_id);
        self.edges.disconnect_output(output_id);
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

    pub fn descendants_output_ids(&self, node_id: NodeId) -> HashSet<OutputId> {
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

    /// Get all descendant nodes of `node_id` without using recursion
    pub fn descendants(&self, node_id: NodeId) -> HashSet<NodeId> {
        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        let mut descendant_ids = HashSet::<NodeId>::new();
        while let Some(search_node_id) = nodes_to_search.pop() {
            self.children(search_node_id).for_each(|descendant_id| {
                nodes_to_search.push(descendant_id);
                descendant_ids.insert(descendant_id);
            });
        }
        descendant_ids
    }

    /// Get all ancestor nodes of `node_id` without using recursion
    pub fn ancestors(&self, node_id: NodeId) -> HashSet<NodeId> {
        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        let mut ancestor_ids = HashSet::<NodeId>::new();
        while let Some(search_node_id) = nodes_to_search.pop() {
            self.parents(search_node_id).for_each(|ancestor_id| {
                nodes_to_search.push(ancestor_id);
                ancestor_ids.insert(ancestor_id);
            });
        }
        ancestor_ids
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
                    self.edges.connect(*new_output_id, *new_input_id);
                }
            }
        }

        other_to_new_node_ids
    }

    pub fn node_inputs(&self, node_id: NodeId) -> impl Iterator<Item = &Input> {
        self[node_id]
            .input_ids
            .iter()
            .map(|input_id| &self[*input_id])
    }

    pub fn node_outputs(&self, node_id: NodeId) -> impl Iterator<Item = &Output> {
        self[node_id]
            .output_ids
            .iter()
            .map(|output_id| &self[*output_id])
    }

    pub fn node_input(&self, node_id: NodeId, name: &str) -> NodeResult<&Input> {
        self.node_inputs(node_id)
            .find(|input| input.name == name)
            .ok_or_else(|| NodeErrors::NodeDoesNotContainInputError {
                node_id,
                input_name: name.to_string(),
            })
    }

    pub fn node_input_id_from_string(&self, node_id: NodeId, name: &str) -> NodeResult<InputId> {
        self.node_inputs(node_id)
            .find(|input| input.name == name)
            .map(|input| input.id)
            .ok_or_else(|| NodeErrors::NodeDoesNotContainInputError {
                node_id,
                input_name: name.to_string(),
            })
    }

    pub fn node_input_id<I: NodeInputData>(
        &self,
        node_id: NodeId,
        node_input_data: I,
    ) -> NodeResult<InputId> {
        self.node_inputs(node_id)
            .find(|input| input.name == node_input_data.to_string())
            .map(|input| input.id)
            .ok_or_else(|| NodeErrors::NodeDoesNotContainInputError {
                node_id,
                input_name: node_input_data.to_string(),
            })
    }

    pub fn node_first_output(&self, node_id: NodeId) -> Option<&Output> {
        self.node_outputs(node_id).next()
    }

    pub fn node_first_output_id(&self, node_id: NodeId) -> Option<&OutputId> {
        self[node_id].output_ids.iter().next()
    }

    pub fn connect_node_to_input(&mut self, node_id: NodeId, input_id: InputId) -> bool {
        if let Some(output_id) = self.node_first_output_id(node_id) {
            if self.is_valid_edge(*output_id, input_id) {
                self.edges.connect(*output_id, input_id);
                return true;
            }
        }
        false
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
                    panic!(
                        "{} index error for {}[{:?}]",
                        stringify!($id_type),
                        stringify!($arena),
                        index
                    )
                })
            }
        }

        impl std::ops::IndexMut<$id_type> for NodeGraph {
            fn index_mut(&mut self, index: $id_type) -> &mut Self::Output {
                self.$arena.get_mut(index).unwrap_or_else(|| {
                    panic!(
                        "{} index error for {}[{:?}]",
                        stringify!($id_type),
                        stringify!($arena),
                        index
                    )
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
    use strum::{EnumCount, IntoEnumIterator};

    use super::*;

    #[test]
    fn test_node_creation() {
        let mut node_graph = NodeGraph::new();

        let mut node_ids = HashSet::<NodeId>::new();

        for node_data in NodeData::iter() {
            node_ids.insert(node_graph.add_node(node_data));
        }

        assert_eq!(node_graph.nodes.len(), NodeData::COUNT);
        assert_eq!(node_graph.edges.len(), 0);
        assert_eq!(
            node_ids,
            node_graph.iter_nodes().collect::<HashSet<NodeId>>()
        );
    }

    #[test]
    fn test_node_deletion() {
        let mut node_graph = NodeGraph::new();

        for node_data in NodeData::iter() {
            node_graph.add_node(node_data);
        }

        for node_id in node_graph.iter_nodes().collect::<Vec<NodeId>>() {
            let (_node, disconnections) = node_graph.remove_node(node_id);
            assert!(disconnections.is_empty());
        }

        assert_eq!(node_graph.nodes.len(), 0);
        assert_eq!(node_graph.edges.len(), 0);
    }

    #[test]
    fn test_node_edge_connection() {
        let mut node_graph = NodeGraph::new();

        let primary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let camera_id: NodeId = node_graph.add_node(NodeData::Camera);

        assert_eq!(node_graph.nodes.len(), 3);
        assert_eq!(node_graph.edges.len(), 0);

        node_graph.connect_node_to_input(
            primary_axis_id,
            node_graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .expect("Axis input should exist on Axis node"),
        );

        assert_eq!(node_graph.edges.len(), 1);
        assert_eq!(
            node_graph.children(primary_axis_id).next(),
            Some(secondary_axis_id)
        );

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .expect("Axis input should exist on Camera node"),
        );

        assert_eq!(node_graph.edges.len(), 2);
    }

    #[test]
    fn test_node_edge_connection_string() {
        let mut node_graph = NodeGraph::new();

        let primary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let camera_id: NodeId = node_graph.add_node(NodeData::Camera);

        assert_eq!(node_graph.nodes.len(), 3);
        assert_eq!(node_graph.edges.len(), 0);

        node_graph.connect_node_to_input(
            primary_axis_id,
            node_graph
                .node_input_id_from_string(secondary_axis_id, "Axis")
                .expect("Axis input should exist on Axis node"),
        );

        assert_eq!(node_graph.edges.len(), 1);

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id_from_string(camera_id, "Axis")
                .expect("Axis input should exist on Camera node"),
        );

        assert_eq!(node_graph.edges.len(), 2);
    }

    #[test]
    fn test_node_edge_disconnection() {
        let mut node_graph = NodeGraph::new();

        let primary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let camera_id: NodeId = node_graph.add_node(NodeData::Camera);

        node_graph.connect_node_to_input(
            primary_axis_id,
            node_graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .expect("Axis input should exist on Axis node"),
        );

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .expect("Axis input should exist on Camera node"),
        );

        assert_eq!(node_graph.edges.len(), 2);

        node_graph.disconnect_node_input(secondary_axis_id, AxisInputData::Axis);

        assert_eq!(node_graph.edges.len(), 1);

        node_graph.disconnect_node_input(camera_id, CameraInputData::Axis);

        assert_eq!(node_graph.edges.len(), 0);
    }

    #[test]
    fn test_node_edge_disconnection_on_node_removal() {
        let mut node_graph = NodeGraph::new();

        let primary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let camera_id: NodeId = node_graph.add_node(NodeData::Camera);

        let primitive0_id: NodeId = node_graph.add_node(NodeData::Primitive);
        let primitive1_id: NodeId = node_graph.add_node(NodeData::Primitive);

        let primary_axis_output_id: OutputId =
            *node_graph.node_first_output_id(primary_axis_id).unwrap();

        let secondary_axis_output_id: OutputId =
            *node_graph.node_first_output_id(secondary_axis_id).unwrap();
        let secondary_axis_axis_input_id: InputId = node_graph
            .node_input_id(secondary_axis_id, AxisInputData::Axis)
            .expect("Axis input should exist on Axis node");

        let camera_axis_input_id: InputId = node_graph
            .node_input_id(camera_id, CameraInputData::Axis)
            .expect("Axis input should exist on Camera node");

        let primitive_axis_input_id: InputId = node_graph
            .node_input_id(primitive0_id, PrimitiveInputData::Axis)
            .expect("Axis input should exist on Primitive node");

        node_graph.connect_node_to_input(
            primary_axis_id,
            node_graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .expect("Axis input should exist on Axis node"),
        );

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .expect("Axis input should exist on Camera node"),
        );

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id(primitive0_id, PrimitiveInputData::Axis)
                .expect("Axis input should exist on Primitive node"),
        );

        node_graph.connect_node_to_input(
            primitive0_id,
            node_graph
                .node_input_id(primitive1_id, PrimitiveInputData::Siblings)
                .expect("Siblings input should exist on Primitive node"),
        );

        let mut expected_disconnections = HashMap::<InputId, OutputId>::new();
        expected_disconnections.insert(secondary_axis_axis_input_id, primary_axis_output_id);
        expected_disconnections.insert(camera_axis_input_id, secondary_axis_output_id);
        expected_disconnections.insert(primitive_axis_input_id, secondary_axis_output_id);

        let (_node, disconnections) = node_graph.remove_node(secondary_axis_id);
        assert_eq!(disconnections, expected_disconnections);
    }

    #[test]
    fn test_valid_edge() {
        let mut node_graph = NodeGraph::new();

        let primary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let camera_id: NodeId = node_graph.add_node(NodeData::Camera);

        let primary_axis_output_id: OutputId =
            *node_graph.node_first_output_id(primary_axis_id).unwrap();
        let primary_axis_axis_input_id: InputId = node_graph
            .node_input_id(primary_axis_id, AxisInputData::Axis)
            .expect("Axis input should exist on Axis node");

        let secondary_axis_output_id: OutputId =
            *node_graph.node_first_output_id(secondary_axis_id).unwrap();
        let secondary_axis_axis_input_id: InputId = node_graph
            .node_input_id(secondary_axis_id, AxisInputData::Axis)
            .expect("Axis input should exist on Axis node");

        let camera_output_id: OutputId = *node_graph.node_first_output_id(camera_id).unwrap();

        assert!(node_graph.is_valid_edge(primary_axis_output_id, secondary_axis_axis_input_id));
        assert!(!node_graph.is_valid_edge(primary_axis_output_id, primary_axis_axis_input_id));
        assert!(node_graph.is_valid_edge(secondary_axis_output_id, primary_axis_axis_input_id));
        assert!(!node_graph.is_valid_edge(camera_output_id, primary_axis_axis_input_id));

        node_graph.connect_node_to_input(
            primary_axis_id,
            node_graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .expect("Axis input should exist on Axis node"),
        );

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .expect("Axis input should exist on Camera node"),
        );

        assert!(!node_graph.is_valid_edge(secondary_axis_output_id, primary_axis_axis_input_id));
    }

    #[test]
    fn test_node_ancestors() {
        let mut node_graph = NodeGraph::new();

        let primary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let camera_id: NodeId = node_graph.add_node(NodeData::Camera);

        let primitive0_id: NodeId = node_graph.add_node(NodeData::Primitive);
        let primitive1_id: NodeId = node_graph.add_node(NodeData::Primitive);

        node_graph.connect_node_to_input(
            primary_axis_id,
            node_graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .expect("Axis input should exist on Axis node"),
        );

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .expect("Axis input should exist on Camera node"),
        );

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id(primitive0_id, PrimitiveInputData::Axis)
                .expect("Axis input should exist on Primitive node"),
        );

        node_graph.connect_node_to_input(
            primitive0_id,
            node_graph
                .node_input_id(primitive1_id, PrimitiveInputData::Siblings)
                .expect("Siblings input should exist on Primitive node"),
        );

        let mut camera_ancestors = HashSet::<NodeId>::new();
        camera_ancestors.insert(primary_axis_id);
        camera_ancestors.insert(secondary_axis_id);

        assert_eq!(node_graph.ancestors(camera_id), camera_ancestors);

        let mut secondary_axis_ancestors = HashSet::<NodeId>::new();
        secondary_axis_ancestors.insert(primary_axis_id);

        assert_eq!(
            node_graph.ancestors(secondary_axis_id),
            secondary_axis_ancestors
        );

        assert!(node_graph.ancestors(primary_axis_id).is_empty());

        let mut primitive1_ancestors = HashSet::<NodeId>::new();
        primitive1_ancestors.insert(primitive0_id);
        primitive1_ancestors.insert(secondary_axis_id);
        primitive1_ancestors.insert(primary_axis_id);

        assert_eq!(node_graph.ancestors(primitive1_id), primitive1_ancestors);
    }

    #[test]
    fn test_node_descendants() {
        let mut node_graph = NodeGraph::new();

        let primary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let camera_id: NodeId = node_graph.add_node(NodeData::Camera);

        let primitive0_id: NodeId = node_graph.add_node(NodeData::Primitive);
        let primitive1_id: NodeId = node_graph.add_node(NodeData::Primitive);

        node_graph.connect_node_to_input(
            primary_axis_id,
            node_graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .expect("Axis input should exist on Axis node"),
        );

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .expect("Axis input should exist on Camera node"),
        );

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id(primitive0_id, PrimitiveInputData::Axis)
                .expect("Axis input should exist on Primitive node"),
        );

        node_graph.connect_node_to_input(
            primitive0_id,
            node_graph
                .node_input_id(primitive1_id, PrimitiveInputData::Siblings)
                .expect("Siblings input should exist on Primitive node"),
        );

        assert!(node_graph.descendants(camera_id).is_empty());
        assert!(node_graph.descendants(primitive1_id).is_empty());
        assert!(node_graph.descendants_output_ids(camera_id).is_empty());
        assert!(node_graph.descendants_output_ids(primitive1_id).is_empty());

        let mut secondary_axis_descendants = HashSet::<NodeId>::new();
        secondary_axis_descendants.insert(camera_id);
        secondary_axis_descendants.insert(primitive0_id);
        secondary_axis_descendants.insert(primitive1_id);

        assert_eq!(
            node_graph.descendants(secondary_axis_id),
            secondary_axis_descendants,
        );

        let mut secondary_axis_descendants_output_ids = HashSet::<OutputId>::new();
        secondary_axis_descendants_output_ids.extend(node_graph[camera_id].output_ids.iter());
        secondary_axis_descendants_output_ids.extend(node_graph[primitive0_id].output_ids.iter());
        secondary_axis_descendants_output_ids.extend(node_graph[primitive1_id].output_ids.iter());

        assert_eq!(
            node_graph.descendants_output_ids(secondary_axis_id),
            secondary_axis_descendants_output_ids,
        );

        let mut primary_axis_descendants = HashSet::<NodeId>::new();
        primary_axis_descendants.insert(secondary_axis_id);
        primary_axis_descendants.insert(camera_id);
        primary_axis_descendants.insert(primitive0_id);
        primary_axis_descendants.insert(primitive1_id);

        assert_eq!(
            node_graph.descendants(primary_axis_id),
            primary_axis_descendants,
        );

        let mut primary_axis_descendants_output_ids = HashSet::<OutputId>::new();
        primary_axis_descendants_output_ids.extend(node_graph[secondary_axis_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(node_graph[camera_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(node_graph[primitive0_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(node_graph[primitive1_id].output_ids.iter());

        assert_eq!(
            node_graph.descendants_output_ids(primary_axis_id),
            primary_axis_descendants_output_ids,
        );

        let mut primitive0_descendants = HashSet::<NodeId>::new();
        primitive0_descendants.insert(primitive1_id);

        assert_eq!(
            node_graph.descendants(primitive0_id),
            primitive0_descendants,
        );

        let mut primitive0_descendants_output_ids = HashSet::<OutputId>::new();
        primitive0_descendants_output_ids.extend(node_graph[primitive1_id].output_ids.iter());

        assert_eq!(
            node_graph.descendants_output_ids(primitive0_id),
            primitive0_descendants_output_ids,
        );
        assert_eq!(
            node_graph.descendants_output_ids(primitive0_id),
            primitive0_descendants_output_ids,
        );
    }

    #[test]
    fn test_node_graph_merge() {
        let mut node_graph = NodeGraph::new();
        let mut node_graph1 = NodeGraph::new();

        let mut node_ids = HashSet::<NodeId>::new();
        let mut node_ids1 = HashSet::<NodeId>::new();

        for node_data in NodeData::iter() {
            node_ids.insert(node_graph.add_node(node_data));
            node_ids1.insert(node_graph1.add_node(node_data));
        }

        let node_id_lut: HashMap<NodeId, NodeId> = node_graph.merge(&mut node_graph1);

        assert_eq!(node_graph.nodes.len(), NodeData::COUNT * 2);
        assert_eq!(node_graph.edges.len(), 0);
        assert_eq!(node_graph1.nodes.len(), 0);
        assert_eq!(
            node_ids1,
            node_id_lut
                .keys()
                .map(|node_id| *node_id)
                .collect::<HashSet<NodeId>>(),
        );
        node_ids.extend(node_id_lut.values());
        assert_eq!(
            node_ids,
            node_graph.iter_nodes().collect::<HashSet<NodeId>>(),
        );
    }

    #[test]
    fn test_node_graph_merge_edges() {
        let mut node_graphs = vec![NodeGraph::new(), NodeGraph::new()];

        let mut primary_axis_ids = Vec::<NodeId>::new();
        let mut secondary_axis_ids = Vec::<NodeId>::new();
        let mut camera_ids = Vec::<NodeId>::new();
        let mut primitive0_ids = Vec::<NodeId>::new();
        let mut primitive1_ids = Vec::<NodeId>::new();

        node_graphs.iter_mut().for_each(|node_graph| {
            let primary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
            let secondary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
            let camera_id: NodeId = node_graph.add_node(NodeData::Camera);
            let primitive0_id: NodeId = node_graph.add_node(NodeData::Primitive);
            let primitive1_id: NodeId = node_graph.add_node(NodeData::Primitive);

            node_graph.connect_node_to_input(
                primary_axis_id,
                node_graph
                    .node_input_id(secondary_axis_id, AxisInputData::Axis)
                    .expect("Axis input should exist on Axis node"),
            );

            node_graph.connect_node_to_input(
                secondary_axis_id,
                node_graph
                    .node_input_id(camera_id, CameraInputData::Axis)
                    .expect("Axis input should exist on Camera node"),
            );

            node_graph.connect_node_to_input(
                secondary_axis_id,
                node_graph
                    .node_input_id(primitive0_id, PrimitiveInputData::Axis)
                    .expect("Axis input should exist on Primitive node"),
            );

            node_graph.connect_node_to_input(
                primitive0_id,
                node_graph
                    .node_input_id(primitive1_id, PrimitiveInputData::Siblings)
                    .expect("Siblings input should exist on Primitive node"),
            );

            primary_axis_ids.push(primary_axis_id);
            secondary_axis_ids.push(secondary_axis_id);
            camera_ids.push(camera_id);
            primitive0_ids.push(primitive0_id);
            primitive1_ids.push(primitive1_id);
        });

        let mut node_graph: NodeGraph = node_graphs
            .pop()
            .expect("node_graphs vec has two node graphs");

        let node_id_lut: HashMap<NodeId, NodeId> = node_graphs[0].merge(&mut node_graph);

        let primary_axis_id: NodeId = *node_id_lut.get(&primary_axis_ids[1]).unwrap();
        let secondary_axis_id: NodeId = *node_id_lut.get(&secondary_axis_ids[1]).unwrap();
        let camera_id: NodeId = *node_id_lut.get(&camera_ids[1]).unwrap();
        let primitive0_id: NodeId = *node_id_lut.get(&primitive0_ids[1]).unwrap();
        let primitive1_id: NodeId = *node_id_lut.get(&primitive1_ids[1]).unwrap();

        assert!(node_graphs[0].descendants(camera_id).is_empty());
        assert!(node_graphs[0].descendants(primitive1_id).is_empty());
        assert!(node_graphs[0].descendants_output_ids(camera_id).is_empty());
        assert!(node_graphs[0]
            .descendants_output_ids(primitive1_id)
            .is_empty());

        let mut secondary_axis_descendants = HashSet::<NodeId>::new();
        secondary_axis_descendants.insert(camera_id);
        secondary_axis_descendants.insert(primitive0_id);
        secondary_axis_descendants.insert(primitive1_id);

        assert_eq!(
            node_graphs[0].descendants(secondary_axis_id),
            secondary_axis_descendants,
        );

        let mut secondary_axis_descendants_output_ids = HashSet::<OutputId>::new();
        secondary_axis_descendants_output_ids.extend(node_graphs[0][camera_id].output_ids.iter());
        secondary_axis_descendants_output_ids
            .extend(node_graphs[0][primitive0_id].output_ids.iter());
        secondary_axis_descendants_output_ids
            .extend(node_graphs[0][primitive1_id].output_ids.iter());

        assert_eq!(
            node_graphs[0].descendants_output_ids(secondary_axis_id),
            secondary_axis_descendants_output_ids,
        );

        let mut primary_axis_descendants = HashSet::<NodeId>::new();
        primary_axis_descendants.insert(secondary_axis_id);
        primary_axis_descendants.insert(camera_id);
        primary_axis_descendants.insert(primitive0_id);
        primary_axis_descendants.insert(primitive1_id);

        assert_eq!(
            node_graphs[0].descendants(primary_axis_id),
            primary_axis_descendants,
        );

        let mut primary_axis_descendants_output_ids = HashSet::<OutputId>::new();
        primary_axis_descendants_output_ids
            .extend(node_graphs[0][secondary_axis_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(node_graphs[0][camera_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(node_graphs[0][primitive0_id].output_ids.iter());
        primary_axis_descendants_output_ids.extend(node_graphs[0][primitive1_id].output_ids.iter());

        assert_eq!(
            node_graphs[0].descendants_output_ids(primary_axis_id),
            primary_axis_descendants_output_ids,
        );

        let mut primitive0_descendants = HashSet::<NodeId>::new();
        primitive0_descendants.insert(primitive1_id);

        assert_eq!(
            node_graphs[0].descendants(primitive0_id),
            primitive0_descendants,
        );

        let mut primitive0_descendants_output_ids = HashSet::<OutputId>::new();
        primitive0_descendants_output_ids.extend(node_graphs[0][primitive1_id].output_ids.iter());

        assert_eq!(
            node_graphs[0].descendants_output_ids(primitive0_id),
            primitive0_descendants_output_ids,
        );
    }

    #[test]
    fn test_new_from_nodes() {
        let mut node_graph = NodeGraph::new();

        let primary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let secondary_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
        let camera_id: NodeId = node_graph.add_node(NodeData::Camera);

        let primitive0_id: NodeId = node_graph.add_node(NodeData::Primitive);
        let primitive1_id: NodeId = node_graph.add_node(NodeData::Primitive);

        node_graph.connect_node_to_input(
            primary_axis_id,
            node_graph
                .node_input_id(secondary_axis_id, AxisInputData::Axis)
                .expect("Axis input should exist on Axis node"),
        );

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id(camera_id, CameraInputData::Axis)
                .expect("Axis input should exist on Camera node"),
        );

        node_graph.connect_node_to_input(
            secondary_axis_id,
            node_graph
                .node_input_id(primitive0_id, PrimitiveInputData::Axis)
                .expect("Axis input should exist on Primitive node"),
        );

        node_graph.connect_node_to_input(
            primitive0_id,
            node_graph
                .node_input_id(primitive1_id, PrimitiveInputData::Siblings)
                .expect("Siblings input should exist on Primitive node"),
        );

        let mut node_ids = HashSet::<NodeId>::new();
        node_ids.insert(primary_axis_id);
        node_ids.insert(secondary_axis_id);

        let new_node_graph = node_graph.new_from_nodes(&node_ids);

        assert_eq!(
            new_node_graph.iter_nodes().collect::<HashSet<NodeId>>(),
            node_ids,
        );
        assert_eq!(new_node_graph.descendants(primary_axis_id).len(), 1);
        assert_eq!(
            new_node_graph.descendants(primary_axis_id).iter().next(),
            Some(&secondary_axis_id)
        );
        assert!(new_node_graph.node_output_is_connected(primary_axis_id));
        assert!(!new_node_graph.node_output_is_connected(secondary_axis_id));
        assert!(!new_node_graph.node_input_is_connected(primary_axis_id, AxisInputData::Axis));
        assert!(new_node_graph.node_input_is_connected(secondary_axis_id, AxisInputData::Axis));
    }
}
