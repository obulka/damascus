use std::collections::HashMap;

use eframe::egui;
use egui_node_graph::{GraphEditorState, NodeResponse};

use crate::panels::{
    node_graph::{
        evaluate_node, AllDamascusNodeTemplates, DamascusDataType, DamascusGraphState,
        DamascusNodeData, DamascusNodeTemplate, DamascusResponse, DamascusValueType,
        Bool,
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
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
            .unwrap_or_default();
        Self {
            state,
            user_state: DamascusGraphState::default(),
            viewport_3d: Viewport3d::new(cc),
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
        // egui::SidePanel::right("properties")
        //     .resizable(true)
        //     .default_width(250.0)
        //     .show(ctx, |ui| {
        //         ui.vertical_centered(|ui| {
        //             ui.heading("Right Panel");
        //         });
        //         egui::ScrollArea::vertical().show(ui, |ui| {
        //             ui.label("test");
        //         });
        //     });

        let graph_response = egui::TopBottomPanel::bottom("bottom")
            .resizable(true)
            .default_height(300.0)
            .show(ctx, |ui| {
                self.state
                    .draw_graph_editor(ui, AllDamascusNodeTemplates, &mut self.user_state)
            })
            .inner;
        for node_response in graph_response.node_responses {
            // Here, we ignore all other graph events. But you may find
            // some use for them. For example, by playing a sound when a new
            // connection is created
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    DamascusResponse::SetActiveNode(node) => {
                        self.user_state.active_node = Some(node)
                    }
                    DamascusResponse::ClearActiveNode => self.user_state.active_node = None,
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

                        DamascusValueType::Bool { value: Bool::new(false, None) }
                    }
                };
                if let Some(ref mut viewport_3d) = &mut self.viewport_3d {
                    match value_type {
                        // DamascusValueType::Mat4 { value } => {}
                        // DamascusValueType::Image { value } => {}
                        DamascusValueType::Camera { value } => {
                            viewport_3d.renderer.scene.render_camera = value;
                        }
                        DamascusValueType::Light { value } => {
                            viewport_3d.renderer.scene.lights = value;
                        }
                        DamascusValueType::Primitive { value } => {
                            viewport_3d.renderer.scene.primitives = value;
                        }
                        DamascusValueType::RayMarcher { value } => {
                            viewport_3d.renderer = value;
                        }
                        DamascusValueType::Scene { value } => {
                            viewport_3d.renderer.scene = value;
                        }
                        _ => {}
                    }
                }
            } else {
                self.user_state.active_node = None;
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
