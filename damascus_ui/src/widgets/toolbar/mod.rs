// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fs::File;
use std::io::{BufReader, Read, Write};

use iced;
use macro_rules_attribute::derive;

// use super::{
//     dialog,
//     viewport::Viewport,
// };
use crate::{
    app::Context,
    widgets::{node_graph::NodeGraph, panel::PanelMessage, style},
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FileDescriptor {
    path: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum FileMessage {
    Load(FileDescriptor),
    Save,
    SaveAs(FileDescriptor),
}

fn save(file_path: &str, context: &Context, success_dialog: bool) {
    let Ok(mut file) = File::create(file_path) else {
        println!("File Creation Error");
        // dialog::error(
        //     modal,
        //     "File Creation Error",
        //     &format!("Could not save file at {:}", file_path),
        // );
        return;
    };
    let Ok(serialization) = serde_json::to_string_pretty(context) else {
        println!("Serialization Error");
        // dialog::error(
        //     modal,
        //     "Node Graph Serialization Error",
        //     &format!("Could not save file at {:}", file_path),
        // );
        return;
    };
    let Ok(_) = file.write_all(serialization.as_bytes()) else {
        println!("File Write Error");
        // dialog::error(
        //     modal,
        //     "File Write Error",
        //     &format!("Could not save file at {:}", file_path),
        // );
        return;
    };
    if success_dialog {
        println!("File saved at {:}", file_path);
        // dialog::success(&modal, "Success", &format!("File saved at {:}", file_path));
    }
}

// fn load(file_path: &str, node_graph: &mut NodeGraph) {
//     let Ok(file) = File::open(file_path) else {
//         dialog::error(
//             modal,
//             "File Open Error",
//             &format!("Could not open file from {:}", file_path),
//         );
//         return;
//     };
//     let mut buf_reader = BufReader::new(file);
//     let mut contents = String::new();
//     let Ok(_) = buf_reader.read_to_string(&mut contents) else {
//         dialog::error(
//             modal,
//             "File Read Error",
//             &format!("Could not read file from {:}", file_path),
//         );
//         return;
//     };
//     let Ok(state) = serde_json::from_str(&contents) else {
//         dialog::error(
//             modal,
//             "Deserialization Error",
//             &format!("Could not load node graph from {:}", file_path),
//         );
//         return;
//     };

//     *node_graph = state;
// }

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ToolbarMessage {
    File(FileMessage),
}

#[derive(Clone, Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Toolbar {}

impl Toolbar {
    pub fn update(
        &mut self,
        context: &Context,
        message: ToolbarMessage,
    ) -> iced::Task<ToolbarMessage> {
        match message {
            ToolbarMessage::File(file_message) => match file_message {
                FileMessage::Load(_file) => {}
                FileMessage::Save => {
                    if let Some(working_file) = &context.working_file {
                        save(working_file, &context, true);
                    }
                }
                FileMessage::SaveAs(_file) => {}
            },
        }
        iced::Task::none()
    }

    pub fn view<'a>(
        &'a self,
        preferences: &'a style::Preferences,
        // _viewport: &mut Viewport,
    ) -> iced::Element<'a, ToolbarMessage> {
        // let mut modal =
        //     egui_modal::Modal::new(egui_context, "dialog_modal").with_style(&egui_modal::ModalStyle {
        //         ..Default::default()
        //     });
        // modal.show_dialog();

        let file_menu = iced::widget::button(iced::widget::text("File/Save").height(16))
            .style(iced::widget::button::secondary)
            .padding(3)
            .on_press(ToolbarMessage::File(FileMessage::Save));

        iced::widget::row![file_menu]
            .spacing(3)
            // .style(style::title_bar(preferences))
            .into()

        // egui::TopBottomPanel::top("toolbar").show(egui_context, |ui| {
        //     egui::MenuBar::new().ui(ui, |ui| {
        //         // File menu
        //         let mut new_requested: bool = egui_context.memory(|memory| memory.focused().is_none())
        //             && egui_context.input(|input| {
        //                 input.key_pressed(egui::Key::N)
        //                     && input.modifiers.matches_logically(egui::Modifiers::CTRL)
        //             });
        //         let mut load_requested: bool = egui_context.memory(|memory| memory.focused().is_none())
        //             && egui_context.input(|input| {
        //                 input.key_pressed(egui::Key::L)
        //                     && input.modifiers.matches_logically(egui::Modifiers::CTRL)
        //             });
        //         let mut save_requested: bool = egui_context.memory(|memory| memory.focused().is_none())
        //             && egui_context.input(|input| {
        //                 input.key_pressed(egui::Key::S)
        //                     && input.modifiers.matches_exact(egui::Modifiers::CTRL)
        //             });
        //         let mut save_as_requested: bool = egui_context
        //             .memory(|memory| memory.focused().is_none())
        //             && egui_context.input(|input| {
        //                 input.key_pressed(egui::Key::S)
        //                     && input
        //                         .modifiers
        //                         .matches_exact(egui::Modifiers::CTRL | egui::Modifiers::SHIFT)
        //             });

        //         let success_dialog: bool = !save_requested;

        //         ui.menu_button("File", |ui| {
        //             new_requested |= ui
        //                 .add(egui::Button::new("new").shortcut_text("Ctrl+N"))
        //                 .clicked();
        //             load_requested |= ui
        //                 .add(egui::Button::new("load").shortcut_text("Ctrl+L"))
        //                 .clicked();
        //             save_requested |= ui
        //                 .add(egui::Button::new("save").shortcut_text("Ctrl+S"))
        //                 .clicked();
        //             save_as_requested |= ui
        //                 .add(egui::Button::new("save as").shortcut_text("Ctrl+Shift+S"))
        //                 .clicked();

        //             if load_requested || save_requested || save_as_requested {
        //                 ui.close();
        //             }
        //         });

        //         save_requested |= new_requested && context.dirty(node_graph);

        //         let mut saved: bool = false;
        //         if load_requested {
        //             let mut file_dialog = rfd::FileDialog::new()
        //                 .set_title("load from file")
        //                 .add_filter("damascus", &["dam"]);
        //             if let Some(file_path) = &context.working_file {
        //                 if let Some(directory) = std::path::Path::new(file_path).parent() {
        //                     file_dialog = file_dialog.set_directory(directory);
        //                 }
        //                 file_dialog = file_dialog.set_file_name(file_path);
        //             }
        //             if let Some(path) = file_dialog.pick_file() {
        //                 let file_path: String = path.display().to_string();
        //                 load(&file_path, node_graph, &modal);
        //                 context.update(file_path.to_string(), node_graph);
        //             }
        //         } else if save_requested {
        //             if let Some(file_path) = &context.working_file {
        //                 save(file_path, node_graph, &modal, success_dialog);
        //                 context.update(file_path.to_string(), node_graph);
        //                 saved = true;
        //             }
        //         }
        //         if save_requested && !saved || save_as_requested {
        //             let mut file_dialog = rfd::FileDialog::new()
        //                 .set_title("save to file")
        //                 .add_filter("damascus", &["dam"]);
        //             if let Some(file_path) = &context.working_file {
        //                 if let Some(directory) = std::path::Path::new(file_path).parent() {
        //                     file_dialog = file_dialog.set_directory(directory);
        //                 }
        //                 file_dialog = file_dialog.set_file_name(file_path);
        //             }
        //             if let Some(path) = file_dialog.save_file() {
        //                 let file_path: String = path.display().to_string();
        //                 save(&file_path, node_graph, &modal, true);
        //                 context.update(file_path, node_graph);
        //             }
        //         }
        //         if new_requested {
        //             node_graph.clear();
        //             *context = Context::default();
        //         }

        //         // Cache menu
        //         ui.menu_button("Cache", |ui| {
        //             ui.horizontal(|ui| {
        //                 if ui.button("clear node cache").clicked() {
        //                     node_graph.output_cache.clear();
        //                 }
        //             });
        //         });
        //     });
        // });

        // Vec::<NodeGraphResponse>::new()
    }
}
