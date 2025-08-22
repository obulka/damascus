// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime};

use eframe::egui;
use egui_node_graph::{NodeId, NodeResponse};
use serde_hashkey::{to_key_with_ordered_float, Key, OrderedFloatPolicy};

use damascus_core::render_passes::RenderPasses;

use super::widgets::{
    node_graph::{
        evaluate_output,
        node::{
            callbacks::NodeCallbacks,
            value_type::{Bool, NodeValueType, UIInput},
        },
        NodeGraph, NodeGraphEditorState, NodeGraphResponse, NodeOutputCache,
    },
    toolbar::show_toolbar,
    viewport::{Viewport, ViewportState},
};

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PersistentData {
    pub context: Context,
    pub node_graph_editor_state: NodeGraphEditorState,
    pub viewport_state: ViewportState,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Context {
    pub working_file: Option<String>,
    pub working_file_hash: Option<Key<OrderedFloatPolicy>>,
}

impl Context {
    pub fn update(&mut self, working_file: String, node_graph: &NodeGraph) {
        self.working_file = Some(working_file);
        self.working_file_hash =
            if let Ok(hash) = to_key_with_ordered_float(node_graph.editor_state()) {
                Some(hash)
            } else {
                None
            }
    }

    pub fn dirty(&self, node_graph: &NodeGraph) -> bool {
        if let Some(working_file_hash) = &self.working_file_hash {
            if let Ok(new_hash) = to_key_with_ordered_float(node_graph.editor_state()) {
                return new_hash != *working_file_hash;
            }
        }
        true
    }
}

pub struct Damascus {
    last_lazy_update: SystemTime,
    context: Context,
    node_graph: NodeGraph,
    viewport: Viewport,
}

const PERSISTENCE_KEY: &str = "damascus";

impl Damascus {
    const LAZY_UPDATE_DELAY: f32 = 1.0;

    /// Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        let persistent_data: PersistentData = creation_context
            .storage
            .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
            .unwrap_or_default();
        Self {
            last_lazy_update: SystemTime::now()
                - Duration::from_millis((Self::LAZY_UPDATE_DELAY * 1000.0) as u64),
            context: persistent_data.context,
            node_graph: NodeGraph::new(persistent_data.node_graph_editor_state),
            viewport: Viewport::new(
                persistent_data.viewport_state,
                creation_context.wgpu_render_state.as_ref().unwrap(),
            ),
        }
    }

    fn lazy_update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(
            if let Some(working_file) = &self.context.working_file {
                format!(
                    "damascus - {:}{:}",
                    working_file,
                    if self.context.dirty(&self.node_graph) {
                        "*"
                    } else {
                        ""
                    }
                )
            } else {
                "damascus".to_owned()
            },
        ));
    }

    fn display_error(ctx: &egui::Context, error: &anyhow::Error) {
        ctx.debug_painter().text(
            egui::pos2(10.0, 35.0),
            egui::Align2::LEFT_TOP,
            format!("Error: {}", error),
            egui::TextStyle::Button.resolve(&ctx.style()),
            egui::Color32::RED,
        );
    }
}

impl eframe::App for Damascus {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(
            storage,
            PERSISTENCE_KEY,
            &PersistentData {
                context: self.context.clone(),
                node_graph_editor_state: self.node_graph.editor_state().clone(),
                viewport_state: self.viewport.state,
            },
        );
    }

    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(15)
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Ok(duration_since_lazy_update) =
            SystemTime::now().duration_since(self.last_lazy_update)
        {
            if duration_since_lazy_update.as_secs_f32() >= Self::LAZY_UPDATE_DELAY {
                self.lazy_update(ctx, frame);
                self.last_lazy_update = SystemTime::now();
            }
        }

        let mut responses = show_toolbar(
            ctx,
            &mut self.context,
            &mut self.node_graph,
            &mut self.viewport,
        );

        if let Some(render_state) = frame.wgpu_render_state() {
            let graph_response = self.node_graph.show(ctx);
            for node_response in graph_response.node_responses {
                match node_response {
                    NodeResponse::User(user_event) => {
                        match user_event {
                            NodeGraphResponse::SetActiveNode(node) => {
                                self.node_graph.user_state_mut().active_node = Some(node);
                                self.viewport.enable();
                                responses.push(NodeGraphResponse::CheckPreprocessorDirectives)
                            }
                            NodeGraphResponse::ClearActiveNode => {
                                self.node_graph.user_state_mut().active_node = None;
                            }
                            NodeGraphResponse::InputValueChanged(
                                node_id,
                                node_template,
                                input_name,
                            ) => {
                                // Perform callbacks when inputs have changed
                                self.node_graph.remove_from_cache(
                                    self.node_graph.graph().child_outputs(node_id),
                                );
                                responses.append(&mut node_template.input_value_changed(
                                    &mut self.node_graph,
                                    node_id,
                                    &input_name,
                                ));
                            }
                            NodeGraphResponse::CheckPreprocessorDirectives => {
                                self.viewport
                                    .recompile_if_preprocessor_directives_changed(render_state);
                            }
                            NodeGraphResponse::ReconstructRenderResources => {
                                self.viewport.reconstruct_render_resources(render_state);
                            }
                        }
                    }
                    NodeResponse::DeleteNodeFull { node_id: _, node } => {
                        self.node_graph.remove_from_cache(
                            node.outputs
                                .into_iter()
                                .map(|(_, output_id)| output_id)
                                .collect(),
                        );
                    }
                    NodeResponse::DisconnectEvent {
                        input: input_id,
                        output: output_id,
                    } => {
                        // This can be triggered when nodes are deleted but the input node
                        // won't exist, so trust the DeleteNodeFull callback to remove
                        // the deleted outputs from the cache
                        if let Some(input) = self.node_graph.graph().try_get_input(input_id) {
                            let node_id: NodeId = input.node;
                            self.node_graph
                                .remove_from_cache(self.node_graph.graph().child_outputs(node_id));

                            let node_template = self.node_graph.graph()[node_id].user_data.template;
                            responses.append(&mut node_template.input_disconnected(
                                &mut self.node_graph,
                                input_id,
                                output_id,
                            ));
                        }
                    }
                    NodeResponse::ConnectEventEnded { input, output } => {
                        self.node_graph.remove_from_cache(
                            self.node_graph
                                .graph()
                                .child_outputs(self.node_graph.graph().get_input(input).node),
                        );
                        let graph = self.node_graph.graph();
                        let node_template = graph[graph.get_input(input).node].user_data.template;
                        responses.append(&mut node_template.input_connected(
                            &mut self.node_graph,
                            input,
                            output,
                        ));
                    }
                    _ => {}
                }
            }

            if let Some(node_id) = self.node_graph.user_state().active_node {
                if self.node_graph.graph().nodes.contains_key(node_id) {
                    let output = self.node_graph.node(node_id).output_ids().next();
                    if let Some(output_id) = output {
                        let value_type = match evaluate_output(&mut self.node_graph, output_id) {
                            Ok(value) => value,
                            Err(error) => {
                                Self::display_error(ctx, &error);

                                NodeValueType::Bool {
                                    value: Bool::new(false),
                                }
                            }
                        };
                        match value_type {
                            NodeValueType::Camera { value } => {
                                self.viewport.view_camera(value.deref(), render_state)
                            }
                            NodeValueType::Light { value } => {
                                self.viewport.view_lights(value.deref(), render_state)
                            }
                            NodeValueType::Material { value } => {
                                self.viewport.view_atmosphere(value.deref(), render_state)
                            }
                            NodeValueType::ProceduralTexture { value } => self
                                .viewport
                                .view_procedural_texture(value.deref(), render_state),
                            NodeValueType::Primitive { value } => {
                                self.viewport.view_primitives(value.deref(), render_state)
                            }
                            NodeValueType::RenderPass { value } => {
                                if let Some(final_pass) = value.value().last() {
                                    match final_pass {
                                        RenderPasses::RayMarcher { pass: _ } => {
                                            self.viewport.disable_camera_controls();
                                        }
                                        _ => {
                                            self.viewport.enable_camera_controls();
                                        }
                                    }
                                }
                                self.viewport
                                    .update_render_passes(value.deref(), render_state);
                            }
                            NodeValueType::Scene { value } => {
                                self.viewport.view_scene(value.deref(), render_state)
                            }
                            _ => {}
                        }
                    }

                    for response in responses
                        .into_iter()
                        .collect::<HashSet<NodeGraphResponse>>()
                    {
                        match response {
                            NodeGraphResponse::CheckPreprocessorDirectives => {
                                self.viewport
                                    .recompile_if_preprocessor_directives_changed(render_state);
                            }
                            NodeGraphResponse::ReconstructRenderResources => {
                                self.viewport.reconstruct_render_resources(render_state);
                            }
                            _ => {}
                        }
                    }
                } else {
                    self.node_graph.user_state_mut().active_node = None;
                }
            }

            if self.node_graph.user_state().active_node.is_none() {
                self.viewport.disable();
            }

            self.viewport.show(ctx, render_state);
        }
    }
}
