// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use eframe::{
    egui,
    egui_wgpu::{self, wgpu},
    epaint,
};
use egui_node_graph::{GraphResponse, Node, NodeId, NodeResponse, OutputId};
use quick_cache::{
    unsync::{Cache, DefaultLifecycle},
    DefaultHashBuilder, OptionsBuilder, UnitWeighter,
};

use damascus::{
    camera::Camera,
    geometry::primitive::Primitive,
    lights::Light,
    materials::{Material, ProceduralTexture},
    render_passes::{
        resources::{BufferData, RenderResource, RenderResources},
        RenderPass, RenderPasses,
    },
    scene::Scene,
};

mod graph;
pub mod node;
mod response;
mod state;

pub use graph::evaluate_output;
use node::{value_type::NodeValueType, AllNodeTemplates, NodeData, NodeDataType};
pub use response::NodeGraphResponse;
pub use state::{NodeGraphEditorState, NodeGraphState};

pub type Graph = egui_node_graph::Graph<NodeData, NodeDataType, NodeValueType, NodeGraphState>;
pub type NodeOutputCache = Cache<OutputId, NodeValueType>;

pub struct NodeGraph {
    pub output_cache: NodeOutputCache,
    editor_state: NodeGraphEditorState,
    user_state: NodeGraphState,
}

impl NodeGraph {
    pub fn new(editor_state: NodeGraphEditorState) -> Self {
        Self {
            output_cache: NodeOutputCache::with_options(
                OptionsBuilder::new()
                    .estimated_items_capacity(10000)
                    .weight_capacity(10000)
                    .build()
                    .unwrap(),
                UnitWeighter,
                DefaultHashBuilder::default(),
                DefaultLifecycle::default(),
            ),
            editor_state,
            user_state: NodeGraphState::default(),
        }
    }

    pub fn remove_from_cache(&mut self, ids: Vec<OutputId>) {
        for id in ids.iter() {
            self.output_cache.remove(id);
        }
    }

    pub fn clear_cache(&mut self) {
        self.output_cache.clear();
    }

    pub fn clear(&mut self) {
        self.editor_state = NodeGraphEditorState::default();
        self.user_state = NodeGraphState::default();
        self.output_cache.clear();
    }

    pub fn set_editor_state(&mut self, editor_state: NodeGraphEditorState) {
        self.user_state = NodeGraphState::default();
        self.editor_state = editor_state;
    }

    pub fn editor_state(&self) -> &NodeGraphEditorState {
        &self.editor_state
    }

    pub fn editor_state_mut(&mut self) -> &mut NodeGraphEditorState {
        &mut self.editor_state
    }

    pub fn user_state(&self) -> &NodeGraphState {
        &self.user_state
    }

    pub fn user_state_mut(&mut self) -> &mut NodeGraphState {
        &mut self.user_state
    }

    pub fn populate_output(
        &mut self,
        node_id: NodeId,
        param_name: &str,
        value: NodeValueType,
    ) -> anyhow::Result<NodeValueType> {
        let output_id = self.node(node_id).get_output(param_name)?;
        self.output_cache.insert(output_id, value.clone());
        Ok(value)
    }

    pub fn graph(&self) -> &Graph {
        &self.editor_state.graph
    }

    pub fn graph_mut(&mut self) -> &mut Graph {
        &mut self.editor_state.graph
    }

    pub fn node(&self, node_id: NodeId) -> &Node<NodeData> {
        &self.graph()[node_id]
    }

    pub fn node_mut(&mut self, node_id: NodeId) -> &mut Node<NodeData> {
        &mut self.graph_mut()[node_id]
    }

    // Evaluates the input value of
    pub fn evaluate_input(
        &mut self,
        node_id: NodeId,
        param_name: &str,
    ) -> anyhow::Result<NodeValueType> {
        let input_id = self.node(node_id).get_input(param_name)?;

        // The output of another node is connected.
        if let Some(other_output_id) = self.editor_state.graph.connection(input_id) {
            // The value was already computed due to the evaluation of some other
            // node. We simply return value from the cache.
            if let Some(other_value) = self.output_cache.get(&other_output_id) {
                Ok((*other_value).clone())
            }
            // This is the first time encountering this node, so we need to
            // recursively evaluate it.
            else {
                // Calling this will populate the cache
                evaluate_output(self, other_output_id)?;

                // Now that we know the value is cached, return it
                Ok((*self
                    .output_cache
                    .get(&other_output_id)
                    .expect("Cache should be populated"))
                .clone())
            }
        }
        // No existing connection, take the inline value instead.
        else {
            Ok(self.editor_state.graph[input_id].value.clone())
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> GraphResponse<NodeGraphResponse, NodeData> {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let style = ui.style_mut();
                style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
                style.visuals.widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;

                let (_id, mut rect) = ui.allocate_space(ui.available_size());
                rect.min.y -= 8.;
                ui.set_clip_rect(rect);

                if ctx.memory(|memory| memory.focused().is_none())
                    && ui.ctx().input(|input| input.key_pressed(egui::Key::F))
                {
                    self.editor_state.reset_zoom(ui);
                }

                let mut copy_selected: bool = false;
                let mut pasted = String::new();

                ui.ctx().input(|input| {
                    for event in input.events.iter() {
                        match event {
                            egui::Event::Copy => {
                                // App freezes if we set copied_text here, so flag and do it later
                                copy_selected = true;
                            }
                            egui::Event::Paste(text) => {
                                // App freezes if we paste here, so clone the text and do it later
                                pasted = text.to_string();
                            }
                            _ => {}
                        }
                    }
                });

                let mut responses = vec![];

                if copy_selected {
                    if let Ok(serialized_state) =
                        serde_json::to_string_pretty(&self.editor_state.from_selected())
                    {
                        ui.output_mut(|output| {
                            output
                                .commands
                                .push(egui::OutputCommand::CopyText(serialized_state));
                        });
                    }
                } else if !pasted.is_empty() {
                    if let Ok(mut deserialized_editor_state) = serde_json::from_str(&pasted) {
                        let new_nodes = self.editor_state.merge(ui, &mut deserialized_editor_state);
                        for node_id in new_nodes.into_iter() {
                            responses.push(NodeResponse::CreatedNode(node_id));
                        }
                    }
                }

                self.editor_state.draw_graph_editor(
                    ui,
                    AllNodeTemplates,
                    &mut self.user_state,
                    responses,
                )
            })
            .inner
    }
}
