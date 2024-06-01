// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use std::collections::HashMap;

use eframe::egui;
use egui_node_graph::NodeResponse;

use super::toolbar::show_toolbar;
use super::widgets::{
    node_graph::{
        evaluate_node,
        node::{
            callbacks::NodeCallbacks,
            value_type::{Bool, NodeValueType, UIInput},
        },
        NodeGraph, NodeGraphResponse,
    },
    viewport::Viewport,
};

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Context {
    pub working_file: Option<String>,
}

pub struct Damascus {
    context: Context,
    node_graph: NodeGraph,
    viewport: Viewport,
}

const PERSISTENCE_KEY: &str = "damascus";

impl Damascus {
    /// Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            context: Context::default(),
            node_graph: NodeGraph::new(creation_context, PERSISTENCE_KEY),
            viewport: Viewport::new(creation_context),
        }
    }
}

impl eframe::App for Damascus {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, self.node_graph.editor_state());
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(10)
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(
            if let Some(working_file) = &self.context.working_file {
                format!("damascus - {:}", working_file)
            } else {
                "damascus".to_owned()
            },
        ));

        if ctx.input(|input| {
            input.key_pressed(egui::Key::N)
                && input.modifiers.matches_logically(egui::Modifiers::CTRL)
        }) {
            self.node_graph.clear();
        }

        show_toolbar(ctx, &mut self.context, &mut self.node_graph);

        let graph_response = self.node_graph.show(ctx);

        for node_response in graph_response.node_responses {
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    NodeGraphResponse::SetActiveNode(node) => {
                        self.node_graph.user_state_mut().active_node = Some(node);
                        self.viewport.enable_and_play();
                    }
                    NodeGraphResponse::ClearActiveNode => {
                        self.node_graph.user_state_mut().active_node = None;
                    }
                    NodeGraphResponse::InputValueChanged(node_id, node_template, input_name) => {
                        // Perform callbacks when inputs have changed
                        node_template.input_value_changed(
                            &mut self.node_graph.editor_state_mut().graph,
                            node_id,
                            &input_name,
                        );
                    }
                }
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
                        self.viewport.default_renderer_with_camera(value);
                    }
                    NodeValueType::Light { value } => {
                        self.viewport.default_renderer_with_lights(value)
                    }
                    NodeValueType::Material { value } => {
                        self.viewport.default_renderer_with_atmosphere(value)
                    }
                    NodeValueType::ProceduralTexture { value } => {
                        self.viewport.default_renderer_with_texture(value)
                    }
                    NodeValueType::Primitive { value } => {
                        self.viewport.default_renderer_with_primitives(value)
                    }
                    NodeValueType::RayMarcher { value } => {
                        self.viewport.set_3d_renderer(value);
                    }
                    NodeValueType::Scene { value } => {
                        self.viewport.default_renderer_with_scene(value);
                    }
                    _ => {}
                }
            } else {
                self.node_graph.user_state_mut().active_node = None;
            }
        }
        if self.node_graph.user_state().active_node.is_none() {
            self.viewport.disable();
        }

        self.viewport.show(ctx);
    }
}
