// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use std::collections::HashMap;

use eframe::egui;
use egui_node_graph::NodeResponse;

use damascus_core::{
    geometry::Primitive,
    lights::{Light, Lights},
    materials::Material,
};

use super::panels::{
    node_graph::{
        evaluate_node, Bool, NodeCallbacks, NodeGraph, NodeGraphResponse, NodeValueType, UIInput,
    },
    viewport_3d::Viewport3d,
};
use super::toolbar::show_toolbar;

pub struct Damascus {
    node_graph: NodeGraph,
    viewport_3d: Option<Viewport3d>,
}

const PERSISTENCE_KEY: &str = "damascus";

impl Damascus {
    /// Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            node_graph: NodeGraph::new(creation_context, PERSISTENCE_KEY),
            viewport_3d: Viewport3d::new(creation_context),
        }
    }
}

impl eframe::App for Damascus {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, self.node_graph.editor_state());
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| {
            i.key_pressed(egui::Key::N) && i.modifiers.matches_logically(egui::Modifiers::CTRL)
        }) {
            self.node_graph.clear();
        }

        show_toolbar(ctx, &mut self.node_graph);

        let graph_response = self.node_graph.show(ctx);

        for node_response in graph_response.node_responses {
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    NodeGraphResponse::SetActiveNode(node) => {
                        self.node_graph.user_state_mut().active_node = Some(node);
                        if let Some(viewport_3d) = &mut self.viewport_3d {
                            viewport_3d.enable();
                            viewport_3d.play();
                        }
                    }
                    NodeGraphResponse::ClearActiveNode => {
                        self.node_graph.user_state_mut().active_node = None;
                        if let Some(viewport_3d) = &mut self.viewport_3d {
                            viewport_3d.disable();
                        }
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
                if let Some(ref mut viewport_3d) = &mut self.viewport_3d {
                    match value_type {
                        // NodeValueType::Mat4 { value } => {}
                        // NodeValueType::Image { value } => {}
                        NodeValueType::Camera { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene.render_camera = value;
                            viewport_3d.renderer.scene.primitives = vec![Primitive::default()];
                            viewport_3d.enable_camera_controls();
                        }
                        NodeValueType::Light { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene.lights = value;
                            viewport_3d.renderer.scene.primitives = vec![Primitive::default()];
                            viewport_3d.enable_camera_controls();
                        }
                        NodeValueType::Material { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene.clear_primitives();
                            viewport_3d.renderer.scene.clear_lights();
                            viewport_3d.renderer.scene.atmosphere = value;
                            viewport_3d.enable_camera_controls();
                        }
                        NodeValueType::ProceduralTexture { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene.clear_primitives();
                            viewport_3d.renderer.scene.clear_lights();
                            viewport_3d.renderer.scene.atmosphere = Material::default();
                            viewport_3d.renderer.scene.atmosphere.diffuse_colour_texture = value;
                            viewport_3d.enable_camera_controls();
                        }
                        NodeValueType::Primitive { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene.primitives = value;
                            viewport_3d.renderer.scene.lights = vec![Light {
                                light_type: Lights::AmbientOcclusion,
                                ..Default::default()
                            }];
                            viewport_3d.enable_camera_controls();
                        }
                        NodeValueType::RayMarcher { value } => {
                            viewport_3d.renderer = value;
                            viewport_3d.disable_camera_controls();
                        }
                        NodeValueType::Scene { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene = value;
                            viewport_3d.disable_camera_controls();
                        }
                        _ => {}
                    }
                }
            } else {
                self.node_graph.user_state_mut().active_node = None;
                if let Some(viewport_3d) = &mut self.viewport_3d {
                    viewport_3d.pause();
                }
            }
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default())
            .show(ctx, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    if let Some(viewport_3d) = &mut self.viewport_3d {
                        viewport_3d.custom_painting(ui);
                    }
                });
            });
    }
}
