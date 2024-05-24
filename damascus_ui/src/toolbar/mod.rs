use std::fs::File;
use std::io::{BufReader, Read, Write};

use eframe::egui;
use egui_modal;

use super::app::Context;
use super::panels::node_graph::NodeGraph;
use super::widgets::dialog;

fn save(file_path: &str, node_graph: &NodeGraph, modal: &egui_modal::Modal) {
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
    dialog::success(&modal, "Success", &format!("File saved at {:}", file_path));
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
) {
    let mut modal =
        egui_modal::Modal::new(egui_context, "dialog_modal").with_style(&egui_modal::ModalStyle {
            ..Default::default()
        });
    modal.show_dialog();

    egui::TopBottomPanel::top("toolbar").show(egui_context, |ui| {
        egui::menu::bar(ui, |ui| {
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
                    context.working_file = Some(file_path.to_string());
                }
            } else if save_requested {
                if let Some(file_path) = &context.working_file {
                    save(file_path, node_graph, &modal);
                    context.working_file = Some(file_path.to_string());
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
                    save(&file_path, node_graph, &modal);
                    context.working_file = Some(file_path);
                }
            }
        });
    });
}
