// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime};

use eframe::egui;
use egui_node_graph::NodeResponse;
use serde_hashkey::{to_key_with_ordered_float, Key, OrderedFloatPolicy};

use super::widgets::{
    node_graph::{
        evaluate_node,
        node::{
            callbacks::NodeCallbacks,
            value_type::{Bool, NodeValueType, UIInput},
        },
        NodeGraph, NodeGraphEditorState, NodeGraphResponse,
    },
    toolbar::show_toolbar,
    viewport::{Viewport, ViewportSettings},
};

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PersistentData {
    pub context: Context,
    pub editor_state: NodeGraphEditorState,
    pub viewport_settings: ViewportSettings,
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
    const LAZY_UPDATE_DELAY: f32 = 0.3;

    /// Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        let persistent_data: PersistentData = creation_context
            .storage
            .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
            .unwrap_or_default();
        Self {
            last_lazy_update: SystemTime::now()
                - Duration::from_millis((Self::LAZY_UPDATE_DELAY * 1000.) as u64),
            context: persistent_data.context,
            node_graph: NodeGraph::new(persistent_data.editor_state),
            viewport: Viewport::new(creation_context, persistent_data.viewport_settings),
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
}

impl eframe::App for Damascus {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(
            storage,
            PERSISTENCE_KEY,
            &PersistentData {
                context: self.context.clone(),
                editor_state: self.node_graph.editor_state().clone(),
                viewport_settings: self.viewport.settings,
            },
        );
    }

    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(10)
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

        if ctx.memory(|memory| memory.focused().is_none())
            && ctx.input(|input| {
                input.key_pressed(egui::Key::N)
                    && input.modifiers.matches_logically(egui::Modifiers::CTRL)
            })
        {
            self.node_graph.clear();
        }

        let mut responses = show_toolbar(
            ctx,
            &mut self.context,
            &mut self.node_graph,
            &mut self.viewport,
        );

        let graph_response = self.node_graph.show(ctx);
        for node_response in graph_response.node_responses {
            match node_response {
                NodeResponse::User(user_event) => {
                    match user_event {
                        NodeGraphResponse::SetActiveNode(node) => {
                            self.node_graph.user_state_mut().active_node = Some(node);
                            self.viewport.enable_and_play_active();
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
                            responses.append(&mut node_template.input_value_changed(
                                &mut self.node_graph.editor_state_mut().graph,
                                node_id,
                                &input_name,
                            ));
                        }
                        NodeGraphResponse::CheckPreprocessorDirectives => {
                            self.viewport
                                .recompile_if_active_preprocessor_directives_changed(frame);
                        }
                        NodeGraphResponse::Reconstruct2DRenderPipeline => {
                            self.viewport.reconstruct_2d_render_pipeline(frame);
                        }
                        NodeGraphResponse::Reconstruct3DRenderPipeline => {
                            self.viewport.reconstruct_3d_render_pipeline(frame);
                        }
                    }
                }
                // NodeResponse::DisconnectEvent { output, input } => {
                //     let graph = &self.node_graph.editor_state().graph;
                //     let node_template = graph[graph.get_input(input).node].user_data.template;
                //     responses.append(&mut node_template.input_disconnected(
                //         &mut self.node_graph.editor_state_mut().graph,
                //         input,
                //         output,
                //     ));
                // }
                // NodeResponse::ConnectEventEnded { output, input } => {
                //     let graph = &self.node_graph.editor_state().graph;
                //     let node_template = graph[graph.get_input(input).node].user_data.template;
                //     responses.append(&mut node_template.input_connected(
                //         &mut self.node_graph.editor_state_mut().graph,
                //         input,
                //         output,
                //     ));
                // }
                _ => {}
            }
        }

        if let Some(node) = self.node_graph.user_state().active_node {
            if self
                .node_graph
                .editor_state()
                .graph
                .nodes
                .contains_key(node)
            {
                let value_type = match evaluate_node(
                    &self.node_graph.editor_state().graph,
                    node,
                    &mut HashMap::new(),
                ) {
                    Ok(value) => value,
                    Err(error) => {
                        ctx.debug_painter().text(
                            egui::pos2(10.0, 35.0),
                            egui::Align2::LEFT_TOP,
                            format!("Error: {}", error),
                            egui::TextStyle::Button.resolve(&ctx.style()),
                            egui::Color32::RED,
                        );

                        NodeValueType::Bool {
                            value: Bool::new(false),
                        }
                    }
                };
                match value_type {
                    NodeValueType::Camera { value } => {
                        self.viewport.default_renderer_with_camera(*value.value());
                    }
                    NodeValueType::Light { value } => self
                        .viewport
                        .default_renderer_with_lights(value.value().clone()),
                    NodeValueType::Material { value } => self
                        .viewport
                        .default_renderer_with_atmosphere(*value.value()),
                    NodeValueType::ProceduralTexture { value } => {
                        self.viewport.default_renderer_with_texture(*value.value())
                    }
                    NodeValueType::Primitive { value } => self
                        .viewport
                        .default_renderer_with_primitives(value.value().clone()),
                    NodeValueType::RayMarcher { value } => {
                        self.viewport.set_active_renderer(value);
                    }
                    NodeValueType::Scene { value } => {
                        self.viewport
                            .default_renderer_with_scene(value.value().clone());
                    }
                    NodeValueType::Texture { value } => {}
                    _ => {}
                }

                for response in responses
                    .into_iter()
                    .collect::<HashSet<NodeGraphResponse>>()
                {
                    match response {
                        NodeGraphResponse::CheckPreprocessorDirectives => {
                            self.viewport
                                .recompile_if_active_preprocessor_directives_changed(frame);
                        }
                        NodeGraphResponse::Reconstruct2DRenderPipeline => {
                            self.viewport.reconstruct_2d_render_pipeline(frame);
                        }
                        NodeGraphResponse::Reconstruct3DRenderPipeline => {
                            self.viewport.reconstruct_3d_render_pipeline(frame);
                        }
                        _ => {}
                    }
                }
            } else {
                self.node_graph.user_state_mut().active_node = None;
            }
        }

        if self.node_graph.user_state().active_node.is_none() {
            self.viewport.disable_active();
        }

        self.viewport.show(ctx, frame);
    }
}
