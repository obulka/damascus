// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use std::collections::HashMap;

use eframe::egui;
use egui_node_graph::{GraphEditorState, NodeFinder, NodeResponse};

use damascus_core::{
    geometry::Primitive,
    lights::{Light, Lights},
    materials::Material,
};

use super::panels::{
    node_graph::{
        evaluate_node, AllDamascusNodeTemplates, Bool, DamascusDataType, DamascusGraphState,
        DamascusNodeData, DamascusNodeTemplate, DamascusResponse, DamascusValueType, NodeCallbacks,
        UIInput,
    },
    viewport_3d::Viewport3d,
};

type DamascusEditorState = GraphEditorState<
    DamascusNodeData,
    DamascusDataType,
    DamascusValueType,
    DamascusNodeTemplate,
    DamascusGraphState,
>;

pub struct Damascus {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: DamascusEditorState,
    user_state: DamascusGraphState,
    viewport_3d: Option<Viewport3d>,
}

const PERSISTENCE_KEY: &str = "damascus";

impl Damascus {
    /// Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        let state = creation_context
            .storage
            .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
            .unwrap_or_default();
        Self {
            state,
            user_state: DamascusGraphState::default(),
            viewport_3d: Viewport3d::new(creation_context),
        }
    }
}

impl eframe::App for Damascus {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, &self.state);
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("load").clicked() {
                        let mut load_file: Option<String> = None;
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            load_file = Some(path.display().to_string());
                        }
                        if let Some(file_path) = load_file {
                            println!("Loading: {:?}", file_path);
                        }
                        ui.close_menu();
                    }
                    if ui.button("save").clicked() {
                        let mut save_file: Option<String> = None;
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            save_file = Some(path.display().to_string());
                        }
                        if let Some(file_path) = save_file {
                            println!(
                                "Saving: {:#?}\n\n to: {:?}",
                                serde_yaml::to_string(&self.state.graph),
                                file_path,
                            );
                        }
                        ui.close_menu();
                    }
                    // ui.menu_button("SubMenu", |ui| {
                    //     ui.menu_button("SubMenu", |ui| {
                    //         if ui.button("Open...").clicked() {
                    //             ui.close_menu();
                    //         }
                    //         let _ = ui.button("Item");
                    //     });
                    //     ui.menu_button("SubMenu", |ui| {
                    //         if ui.button("Open...").clicked() {
                    //             ui.close_menu();
                    //         }
                    //         let _ = ui.button("Item");
                    //     });
                    //     let _ = ui.button("Item");
                    //     if ui.button("Open...").clicked() {
                    //         ui.close_menu();
                    //     }
                    // });
                    // ui.menu_button("SubMenu", |ui| {
                    //     let _ = ui.button("Item1");
                    //     let _ = ui.button("Item2");
                    //     let _ = ui.button("Item3");
                    //     let _ = ui.button("Item4");
                    //     if ui.button("Open...").clicked() {
                    //         ui.close_menu();
                    //     }
                    // });
                    // let _ = ui.button("Very long text for this item");
                });
            });
        });

        egui::SidePanel::right("right")
            .resizable(true)
            .show_separator_line(false)
            .default_width(0.)
            .show(ctx, |ui| {
                ui.allocate_space(ui.available_size());
                let response = ui.allocate_rect(
                    ui.min_rect(),
                    egui::Sense::click().union(egui::Sense::drag()),
                );
                if response.dragged() && ui.ctx().input(|i| i.pointer.middle_down()) {
                    self.state.pan_zoom.pan += ui.ctx().input(|i| i.pointer.delta());
                }

                let editor_rect = ui.max_rect();
                let cursor_in_editor = ui.rect_contains_pointer(editor_rect);
                let mouse = &ui.ctx().input(|i| i.pointer.clone());
                if mouse.secondary_released() && cursor_in_editor
                // && !graph_response.cursor_in_finder
                {
                    let cursor_pos = ui
                        .ctx()
                        .input(|i| i.pointer.hover_pos().unwrap_or(egui::Pos2::ZERO));
                    self.state.node_finder = Some(NodeFinder::new_at(cursor_pos));
                }
            });
        egui::SidePanel::left("left")
            .resizable(true)
            .show_separator_line(false)
            .default_width(0.)
            .show(ctx, |ui| {
                ui.allocate_space(ui.available_size());
                let response = ui.allocate_rect(
                    ui.min_rect(),
                    egui::Sense::click().union(egui::Sense::drag()),
                );
                if response.dragged() && ui.ctx().input(|i| i.pointer.middle_down()) {
                    self.state.pan_zoom.pan += ui.ctx().input(|i| i.pointer.delta());
                }

                let editor_rect = ui.max_rect();
                let cursor_in_editor = ui.rect_contains_pointer(editor_rect);
                let mouse = &ui.ctx().input(|i| i.pointer.clone());
                if mouse.secondary_released() && cursor_in_editor
                // && !graph_response.cursor_in_finder
                {
                    let cursor_pos = ui
                        .ctx()
                        .input(|i| i.pointer.hover_pos().unwrap_or(egui::Pos2::ZERO));
                    self.state.node_finder = Some(NodeFinder::new_at(cursor_pos));
                }
            });
        let graph_response = egui::TopBottomPanel::bottom("bottom")
            .resizable(true)
            .default_height(300.0)
            .show(ctx, |ui| {
                ui.allocate_space(ui.available_size());
                self.state.draw_graph_editor(
                    ui,
                    AllDamascusNodeTemplates,
                    &mut self.user_state,
                    Vec::default(),
                )
            })
            .inner;
        for node_response in graph_response.node_responses {
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    DamascusResponse::SetActiveNode(node) => {
                        self.user_state.active_node = Some(node);
                        if let Some(viewport_3d) = &mut self.viewport_3d {
                            viewport_3d.enable();
                            viewport_3d.play();
                        }
                    }
                    DamascusResponse::ClearActiveNode => {
                        self.user_state.active_node = None;
                        if let Some(viewport_3d) = &mut self.viewport_3d {
                            viewport_3d.disable();
                        }
                    }
                    DamascusResponse::InputValueChanged(node_id, node_template, input_name) => {
                        // Perform callbacks when inputs have changed
                        node_template.input_value_changed(
                            &mut self.state.graph,
                            node_id,
                            &input_name,
                        );
                    }
                }
            }
        }

        if let Some(node) = self.user_state.active_node {
            if self.state.graph.nodes.contains_key(node) {
                let value_type = match evaluate_node(&self.state.graph, node, &mut HashMap::new()) {
                    Ok(value) => value,
                    Err(error) => {
                        ctx.debug_painter().text(
                            egui::pos2(10.0, 35.0),
                            egui::Align2::LEFT_TOP,
                            format!("Error: {}", error),
                            egui::TextStyle::Button.resolve(&ctx.style()),
                            egui::Color32::RED,
                        );

                        DamascusValueType::Bool {
                            value: Bool::new(false),
                        }
                    }
                };
                if let Some(ref mut viewport_3d) = &mut self.viewport_3d {
                    match value_type {
                        // DamascusValueType::Mat4 { value } => {}
                        // DamascusValueType::Image { value } => {}
                        DamascusValueType::Camera { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene.render_camera = value;
                            viewport_3d.renderer.scene.primitives = vec![Primitive::default()];
                            viewport_3d.enable_camera_controls();
                        }
                        DamascusValueType::Light { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene.lights = value;
                            viewport_3d.renderer.scene.primitives = vec![Primitive::default()];
                            viewport_3d.enable_camera_controls();
                        }
                        DamascusValueType::Material { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene.clear_primitives();
                            viewport_3d.renderer.scene.clear_lights();
                            viewport_3d.renderer.scene.atmosphere = value;
                            viewport_3d.enable_camera_controls();
                        }
                        DamascusValueType::ProceduralTexture { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene.clear_primitives();
                            viewport_3d.renderer.scene.clear_lights();
                            viewport_3d.renderer.scene.atmosphere = Material::default();
                            viewport_3d.renderer.scene.atmosphere.diffuse_colour_texture = value;
                            viewport_3d.enable_camera_controls();
                        }
                        DamascusValueType::Primitive { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene.primitives = value;
                            viewport_3d.renderer.scene.lights = vec![Light {
                                light_type: Lights::AmbientOcclusion,
                                ..Default::default()
                            }];
                            viewport_3d.enable_camera_controls();
                        }
                        DamascusValueType::RayMarcher { value } => {
                            viewport_3d.renderer = value;
                            viewport_3d.disable_camera_controls();
                        }
                        DamascusValueType::Scene { value } => {
                            viewport_3d.renderer.reset_render_parameters();
                            viewport_3d.renderer.scene = value;
                            viewport_3d.enable_camera_controls();
                        }
                        _ => {}
                    }
                }
            } else {
                self.user_state.active_node = None;
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
