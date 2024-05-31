// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use std::collections::HashMap;

use eframe::egui::{self, include_image};
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
use super::MAX_TEXTURE_DIMENSION;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Context {
    pub working_file: Option<String>,
}

pub struct Damascus {
    context: Context,
    node_graph: NodeGraph,
    viewport_3d: Option<Viewport3d>,
}

const PERSISTENCE_KEY: &str = "damascus";

impl Damascus {
    /// Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            context: Context::default(),
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
                        if let Some(viewport_3d) = &mut self.viewport_3d {
                            viewport_3d.enable();
                            viewport_3d.play();
                        }
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
                if let Some(ref mut viewport_3d) = &mut self.viewport_3d {
                    match value_type {
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
            }
        }
        if self.node_graph.user_state().active_node.is_none() {
            if let Some(viewport_3d) = &mut self.viewport_3d {
                viewport_3d.disable();
            }
        }

        let screen_size: egui::Vec2 = ctx.input(|input| input.screen_rect.size());

        egui::Window::new("viewer")
            .default_width(720.)
            .default_height(405.)
            .max_width(
                (screen_size.x * 0.9)
                    .round()
                    .min(MAX_TEXTURE_DIMENSION as f32),
            )
            .max_height(
                (screen_size.y * 0.9)
                    .round()
                    .min(MAX_TEXTURE_DIMENSION as f32),
            )
            .resizable(true)
            .movable(true)
            .constrain(true)
            .show(ctx, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    if let Some(viewport_3d) = &mut self.viewport_3d {
                        viewport_3d.custom_painting(ui);
                    }
                });
                if let Some(viewport_3d) = &mut self.viewport_3d {
                    let style = ui.style_mut();
                    style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                    style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
                    style.visuals.widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        let pause_icon = egui::Image::new(if viewport_3d.paused() {
                            include_image!("../assets/icons/play.svg")
                        } else {
                            include_image!("../assets/icons/pause.svg")
                        })
                        .fit_to_exact_size(egui::Vec2::splat(20.));
                        if ui
                            .add_enabled(viewport_3d.enabled(), egui::ImageButton::new(pause_icon))
                            .clicked()
                        {
                            viewport_3d.toggle_play_pause();
                        }
                    });

                    ui.add(egui::Label::new(&viewport_3d.stats_text).truncate(true));
                }
            });
    }
}
