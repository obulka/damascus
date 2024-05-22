use std::fs::File;
use std::io::{BufReader, Read, Write};

use eframe::egui;
use egui_modal;

use super::panels::node_graph::NodeGraph;
use super::widgets::dialog;

pub fn show_toolbar(ctx: &egui::Context, node_graph: &mut NodeGraph) {
    let mut modal =
        egui_modal::Modal::new(ctx, "dialog_modal").with_style(&egui_modal::ModalStyle {
            ..Default::default()
        });
    modal.show_dialog();

    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("load").clicked() {
                    ui.close_menu();

                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let file_path = path.display().to_string();

                        let Ok(file) = File::open(&file_path) else {
                            dialog::error(
                                &modal,
                                "File Open Error",
                                &format!("Could not open file from {:}", file_path),
                            );
                            return;
                        };
                        let mut buf_reader = BufReader::new(file);
                        let mut contents = String::new();
                        let Ok(_) = buf_reader.read_to_string(&mut contents) else {
                            dialog::error(
                                &modal,
                                "File Read Error",
                                &format!("Could not read file from {:}", file_path),
                            );
                            return;
                        };
                        let Ok(state) = serde_yaml::from_str(&contents) else {
                            dialog::error(
                                &modal,
                                "Deserialization Error",
                                &format!("Could not load node graph from {:}", file_path),
                            );
                            return;
                        };

                        node_graph.set_editor_state(state);
                    }
                } else if ui.button("save").clicked() {
                    ui.close_menu();

                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let file_path = path.display().to_string();

                        let Ok(mut file) = File::create(&file_path) else {
                            dialog::error(
                                &modal,
                                "File Creation Error",
                                &format!("Could not save file at {:}", file_path),
                            );
                            return;
                        };
                        let Ok(serialization) = serde_yaml::to_string(node_graph.editor_state())
                        else {
                            dialog::error(
                                &modal,
                                "Node Graph Serialization Error",
                                &format!("Could not save file at {:}", file_path),
                            );
                            return;
                        };
                        let Ok(_) = file.write_all(serialization.as_bytes()) else {
                            dialog::error(
                                &modal,
                                "File Write Error",
                                &format!("Could not save file at {:}", file_path),
                            );
                            return;
                        };

                        dialog::success(
                            &modal,
                            "Success",
                            &format!("File saved at {:}", file_path),
                        );
                    }
                }
            });
        });
    });
}
