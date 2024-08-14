use std::fs::File;
use std::io::{BufReader, Read, Write};

use eframe::egui;
use egui_modal;

use damascus_core::scene::Scene;

use super::{
    dialog,
    node_graph::{NodeGraph, NodeGraphResponse},
    viewport::Viewport,
};
use crate::{app::Context, MAX_BUFFER_SIZE};

fn save(file_path: &str, node_graph: &NodeGraph, modal: &egui_modal::Modal, success_dialog: bool) {
    let Ok(mut file) = File::create(file_path) else {
        dialog::error(
            modal,
            "File Creation Error",
            &format!("Could not save file at {:}", file_path),
        );
        return;
    };
    let Ok(serialization) = serde_json::to_string_pretty(node_graph.editor_state()) else {
        dialog::error(
            modal,
            "Node Graph Serialization Error",
            &format!("Could not save file at {:}", file_path),
        );
        return;
    };
    let Ok(_) = file.write_all(serialization.as_bytes()) else {
        dialog::error(
            modal,
            "File Write Error",
            &format!("Could not save file at {:}", file_path),
        );
        return;
    };
    if success_dialog {
        dialog::success(&modal, "Success", &format!("File saved at {:}", file_path));
    }
}

fn load(file_path: &str, node_graph: &mut NodeGraph, modal: &egui_modal::Modal) {
    let Ok(file) = File::open(file_path) else {
        dialog::error(
            modal,
            "File Open Error",
            &format!("Could not open file from {:}", file_path),
        );
        return;
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let Ok(_) = buf_reader.read_to_string(&mut contents) else {
        dialog::error(
            modal,
            "File Read Error",
            &format!("Could not read file from {:}", file_path),
        );
        return;
    };
    let Ok(state) = serde_json::from_str(&contents) else {
        dialog::error(
            modal,
            "Deserialization Error",
            &format!("Could not load node graph from {:}", file_path),
        );
        return;
    };

    node_graph.set_editor_state(state);
}

pub fn show_toolbar(
    egui_context: &egui::Context,
    context: &mut Context,
    node_graph: &mut NodeGraph,
    viewport: &mut Viewport,
) -> Vec<NodeGraphResponse> {
    let mut response = Vec::<NodeGraphResponse>::new();

    let mut modal =
        egui_modal::Modal::new(egui_context, "dialog_modal").with_style(&egui_modal::ModalStyle {
            ..Default::default()
        });
    modal.show_dialog();

    egui::TopBottomPanel::top("toolbar").show(egui_context, |ui| {
        egui::menu::bar(ui, |ui| {
            // File menu
            let mut load_requested: bool = egui_context.input(|input| {
                input.key_pressed(egui::Key::L)
                    && input.modifiers.matches_logically(egui::Modifiers::CTRL)
            });
            let mut save_requested: bool = egui_context.input(|input| {
                input.key_pressed(egui::Key::S)
                    && input.modifiers.matches_exact(egui::Modifiers::CTRL)
            });
            let mut save_as_requested: bool = egui_context.input(|input| {
                input.key_pressed(egui::Key::S)
                    && input
                        .modifiers
                        .matches_exact(egui::Modifiers::CTRL | egui::Modifiers::SHIFT)
            });

            let success_dialog: bool = !save_requested;

            ui.menu_button("File", |ui| {
                load_requested |= ui
                    .add(egui::Button::new("load").shortcut_text("Ctrl+L"))
                    .clicked();
                save_requested |= ui
                    .add(egui::Button::new("save").shortcut_text("Ctrl+S"))
                    .clicked();
                save_as_requested |= ui
                    .add(egui::Button::new("save as").shortcut_text("Ctrl+Shift+S"))
                    .clicked();

                if load_requested || save_requested || save_as_requested {
                    ui.close_menu();
                }
            });

            let mut saved: bool = false;
            if load_requested {
                let mut file_dialog = rfd::FileDialog::new().set_title("load from file");
                if let Some(file_path) = &context.working_file {
                    file_dialog = file_dialog.set_file_name(file_path);
                }
                if let Some(path) = file_dialog.pick_file() {
                    let file_path: String = path.display().to_string();
                    load(&file_path, node_graph, &modal);
                    context.update(file_path.to_string(), node_graph);
                }
            } else if save_requested {
                if let Some(file_path) = &context.working_file {
                    save(file_path, node_graph, &modal, success_dialog);
                    context.update(file_path.to_string(), node_graph);
                    saved = true;
                }
            }
            if (save_requested && !saved) || save_as_requested {
                let mut file_dialog = rfd::FileDialog::new().set_title("save to file");
                if let Some(file_path) = &context.working_file {
                    if let Some(directory) = std::path::Path::new(file_path).parent() {
                        file_dialog = file_dialog.set_directory(directory);
                    }
                    file_dialog = file_dialog.set_file_name(file_path);
                }
                if let Some(path) = file_dialog.save_file() {
                    let file_path: String = path.display().to_string();
                    save(&file_path, node_graph, &modal, true);
                    context.update(file_path, node_graph);
                }
            }

            let mut dynamic_compilation_settings_changed: bool = false;
            let mut pipeline_reconstruction_required: bool = false;
            // Settings menu
            ui.menu_button("Settings", |ui| {
                ui.horizontal(|ui| {
                    ui.label("dynamic recompilation for materials");
                    dynamic_compilation_settings_changed |= ui
                        .add(egui::Checkbox::without_text(
                            &mut viewport.settings.enable_dynamic_recompilation_for_materials,
                        ))
                        .clicked();
                });
                ui.horizontal(|ui| {
                    ui.label("dynamic recompilation for primitives");
                    dynamic_compilation_settings_changed |= ui
                        .add(egui::Checkbox::without_text(
                            &mut viewport
                                .settings
                                .enable_dynamic_recompilation_for_primitives,
                        ))
                        .clicked();
                });
                ui.horizontal(|ui| {
                    ui.label("dynamic recompilation for ray marcher");
                    dynamic_compilation_settings_changed |= ui
                        .add(egui::Checkbox::without_text(
                            &mut viewport
                                .settings
                                .enable_dynamic_recompilation_for_ray_marcher,
                        ))
                        .clicked();
                });
                ui.horizontal(|ui| {
                    ui.label("dynamic recompilation for lights");
                    dynamic_compilation_settings_changed |= ui
                        .add(egui::Checkbox::without_text(
                            &mut viewport.settings.enable_dynamic_recompilation_for_lights,
                        ))
                        .clicked();
                });
                ui.horizontal(|ui| {
                    ui.label("max primitives");
                    pipeline_reconstruction_required |= ui
                        .add(egui::DragValue::new(&mut viewport.settings.max_primitives))
                        .changed();
                })
                .response
                .changed();
                ui.horizontal(|ui| {
                    ui.label("max lights");
                    pipeline_reconstruction_required |= ui
                        .add(egui::DragValue::new(&mut viewport.settings.max_lights))
                        .changed();
                });
            });
            viewport.settings.max_primitives = Scene::max_primitives_in_buffer(MAX_BUFFER_SIZE)
                .min(viewport.settings.max_primitives)
                .max(1);
            viewport.settings.max_lights = Scene::max_lights_in_buffer(MAX_BUFFER_SIZE)
                .min(viewport.settings.max_lights)
                .max(1);

            if pipeline_reconstruction_required {
                response.push(NodeGraphResponse::ReconstructRenderPipeline);
            } else if dynamic_compilation_settings_changed {
                response.push(NodeGraphResponse::CheckPreprocessorDirectives);
            }
        });
    });

    response
}
