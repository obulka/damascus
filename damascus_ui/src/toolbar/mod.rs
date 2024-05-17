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
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let file_path = path.display().to_string();

                        match File::open(&file_path) {
                            Ok(file) => {
                                let mut buf_reader = BufReader::new(file);
                                let mut contents = String::new();
                                match buf_reader.read_to_string(&mut contents) {
                                    Ok(_) => match serde_yaml::from_str(&contents) {
                                        Ok(state) => {
                                            node_graph.set_editor_state(state);
                                        }
                                        Err(error) => {
                                            dialog::error(
                                                &modal,
                                                "Deserialization Error",
                                                &format!(
                                                    "Could not load node graph from {:}\n\n{:?}",
                                                    file_path, error,
                                                ),
                                            );
                                        }
                                    },
                                    Err(error) => {
                                        dialog::error(
                                            &modal,
                                            "File Read Error",
                                            &format!(
                                                "Could not read file from {:}\n\n{:?}",
                                                file_path, error,
                                            ),
                                        );
                                    }
                                }
                            }
                            Err(error) => {
                                dialog::error(
                                    &modal,
                                    "File Open Error",
                                    &format!(
                                        "Could not open file from {:}\n\n{:?}",
                                        file_path, error,
                                    ),
                                );
                            }
                        }
                    }
                    ui.close_menu();
                } else if ui.button("save").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let file_path = path.display().to_string();

                        match File::create(&file_path) {
                            Ok(mut file) => {
                                match serde_yaml::to_string(node_graph.editor_state()) {
                                    Ok(serialization) => {
                                        match file.write_all(serialization.as_bytes()) {
                                            Ok(_) => {
                                                dialog::success(
                                                    &modal,
                                                    "Success",
                                                    &format!("File saved at {:}", file_path),
                                                );
                                            }
                                            Err(error) => {
                                                dialog::error(
                                                    &modal,
                                                    "File Write Error",
                                                    &format!(
                                                        "Could not save file at {:}\n\n{:?}",
                                                        file_path, error,
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        dialog::error(
                                            &modal,
                                            "Node Graph Serialization Error",
                                            &format!(
                                                "Could not save file at {:}\n\n{:?}",
                                                file_path, error,
                                            ),
                                        );
                                    }
                                }
                            }
                            Err(error) => {
                                dialog::error(
                                    &modal,
                                    "File Creation Error",
                                    &format!(
                                        "Could not save file at {:}\n\n{:?}",
                                        file_path, error,
                                    ),
                                );
                            }
                        }
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
}
