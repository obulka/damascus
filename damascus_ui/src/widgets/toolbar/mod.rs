// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    fmt,
    fs::File,
    io::{BufReader, Read, Write},
    str::FromStr,
};

use iced;
use macro_rules_attribute::derive;
use strum::IntoEnumIterator;

use damascus::Enumerator;

// use super::{
//     dialog,
//     viewport::Viewport,
// };
use crate::{
    EnumTraits, ErrorTraits,
    app::Context,
    widgets::{Widget, style},
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FileDescriptor {
    path: String,
}

#[derive(Default, EnumTraits!)]
pub enum FileMessage {
    #[default]
    Save,
    SaveAs,
    Load,
}

fn save(context: &mut Context, success_dialog: bool) -> bool {
    let Some(file_path) = context.working_file() else {
        return false;
    };

    let Ok(mut file) = File::create(file_path) else {
        println!("File Creation Error");
        // dialog::error(
        //     modal,
        //     "File Creation Error",
        //     &format!("Could not save file at {:}", file_path),
        // );
        return false;
    };
    let Ok(serialization) = serde_json::to_string_pretty(context) else {
        println!("Serialization Error");
        // dialog::error(
        //     modal,
        //     "Node Graph Serialization Error",
        //     &format!("Could not save file at {:}", file_path),
        // );
        return false;
    };
    let Ok(_) = file.write_all(serialization.as_bytes()) else {
        println!("File Write Error");
        // dialog::error(
        //     modal,
        //     "File Write Error",
        //     &format!("Could not save file at {:}", file_path),
        // );
        return false;
    };

    if success_dialog {
        println!("File saved at {:}", file_path);
        // dialog::success(&modal, "Success", &format!("File saved at {:}", file_path));
    }

    true
}

fn save_as(context: &mut Context, success_dialog: bool) -> bool {
    let mut file_dialog = rfd::FileDialog::new()
        .set_title("save to file")
        .add_filter("damascus", &["dam"]);

    if let Some(file_path) = context.working_file() {
        if let Some(directory) = std::path::Path::new(file_path).parent() {
            file_dialog = file_dialog.set_directory(directory);
        }
        file_dialog = file_dialog.set_file_name(file_path);
    }

    if let Some(path) = file_dialog.save_file() {
        context.set_working_file(path.display().to_string());
        save(context, success_dialog)
    } else {
        false
    }
}

fn load(context: &mut Context, success_dialog: bool) -> bool {
    // TODO Unload current

    let mut file_dialog = rfd::FileDialog::new()
        .set_title("load from file")
        .add_filter("damascus", &["dam"]);

    if let Some(file_path) = &context.working_file() {
        if let Some(directory) = std::path::Path::new(file_path).parent() {
            file_dialog = file_dialog.set_directory(directory);
        }
        file_dialog = file_dialog.set_file_name(file_path);
    }

    if let Some(path) = file_dialog.pick_file() {
        let file_path: String = path.display().to_string();

        let Ok(file) = File::open(&file_path) else {
            println!("Could not open file from {:}", file_path);
            // dialog::error(
            //     modal,
            //     "File Open Error",
            //     &format!("Could not open file from {:}", file_path),
            // );
            return false;
        };
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        let Ok(_) = buf_reader.read_to_string(&mut contents) else {
            println!("Could not read file from {:}", file_path);
            // dialog::error(
            //     modal,
            //     "File Read Error",
            //     &format!("Could not read file from {:}", file_path),
            // );
            return false;
        };
        let Ok(state) = serde_json::from_str(&contents) else {
            println!("Could not load node graph from {:}", file_path);
            // dialog::error(
            //     modal,
            //     "Deserialization Error",
            //     &format!("Could not load node graph from {:}", file_path),
            // );
            return false;
        };

        *context = state;

        println!("Loaded {:}", file_path);

        true
    } else {
        println!("No file chosen");
        false
    }
}

#[derive(Default, EnumTraits!)]
pub enum Menus {
    #[default]
    File,
}

impl Menus {
    pub fn menu_options(&self) -> Vec<String> {
        match self {
            Menus::File => FileMessage::iter()
                .map(|variant| variant.variant_pascal_label())
                .collect(),
        }
    }
}

#[derive(Default, ErrorTraits!)]
pub enum ToolbarErrors {
    DeserializeError(String),
    #[default]
    UnknownError,
}

impl fmt::Display for ToolbarErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeserializeError(error) => write!(
                formatter,
                "{}: Could not deserialize from: {:?}",
                self, error
            ),
            _ => write!(formatter, "{}", self),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ToolbarMessage {
    File(FileMessage),
    Error(ToolbarErrors),
}

impl FromStr for ToolbarMessage {
    type Err = ToolbarErrors;

    fn from_str(option: &str) -> Result<Self, Self::Err> {
        let variant: String = option.chars().filter(|c| !c.is_whitespace()).collect();

        // Currently different menus having the same option will not be supported
        // but I think unique names will be used anyway
        if let Ok(message) = FileMessage::from_str(&variant) {
            Ok(ToolbarMessage::File(message))
        } else {
            Err(Self::Err::DeserializeError(variant))
        }
    }
}

impl From<Result<Self, ToolbarErrors>> for ToolbarMessage {
    fn from(result: Result<Self, ToolbarErrors>) -> Self {
        match result {
            Ok(message) => message,
            Err(error) => ToolbarMessage::Error(error),
        }
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Toolbar {}

impl Widget<ToolbarMessage> for Toolbar {
    fn update(
        &mut self,
        context: &mut Context,
        message: ToolbarMessage,
    ) -> iced::Task<ToolbarMessage> {
        match message {
            ToolbarMessage::File(file_message) => match file_message {
                FileMessage::Save => {
                    if !save(context, true) {
                        save_as(context, true);
                    }
                }
                FileMessage::SaveAs => {
                    save_as(context, true);
                }
                FileMessage::Load => {
                    load(context, true);
                }
            },
            _ => {}
        }
        iced::Task::none()
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        _preferences: &'a style::Preferences,
    ) -> iced::Element<'a, ToolbarMessage> {
        // let mut modal =
        //     egui_modal::Modal::new(egui_context, "dialog_modal").with_style(&egui_modal::ModalStyle {
        //         ..Default::default()
        //     });
        // modal.show_dialog();

        iced::widget::row(Menus::iter().map(|menu_option| {
            iced::widget::pick_list(
                menu_option.menu_options(),
                None::<String>,
                |option| -> ToolbarMessage { ToolbarMessage::from_str(&option).into() },
            )
            .placeholder(&menu_option.to_string())
            .handle(iced::widget::pick_list::Handle::None)
            .style(
                |theme: &iced::Theme, status| -> iced::widget::pick_list::Style {
                    let palette = theme.extended_palette();
                    let base = iced::widget::pick_list::Style {
                        background: palette.background.weakest.color.into(),
                        text_color: palette.background.weakest.text,
                        border: iced::border::rounded(2),
                        handle_color: iced::Color::TRANSPARENT,
                        placeholder_color: palette.background.weakest.text,
                    };

                    match status {
                        iced::widget::pick_list::Status::Active => base,
                        iced::widget::pick_list::Status::Opened { is_hovered: _ }
                        | iced::widget::pick_list::Status::Hovered => {
                            iced::widget::pick_list::Style {
                                background: iced::Background::Color(
                                    palette.background.weaker.color,
                                ),
                                ..base
                            }
                        }
                    }
                },
            )
            .menu_style(|theme: &iced::Theme| -> iced::overlay::menu::Style {
                let palette = theme.extended_palette();
                iced::overlay::menu::Style {
                    background: palette.background.weakest.color.into(),
                    ..iced::overlay::menu::default(theme)
                }
            })
            .into()
        }))
        .spacing(3)
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
