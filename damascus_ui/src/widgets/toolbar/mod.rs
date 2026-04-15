// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{fmt, str::FromStr};

use iced;
use macro_rules_attribute::derive;
use strum::IntoEnumIterator;

use damascus::Enumerator;

use crate::{
    EnumTraits, ErrorTraits,
    app::Context,
    widgets::{Widget, dialog::Dialog, menu, style::Style},
};

pub mod file;
pub mod preferences;

use file::{FileMenuOptions, FileMessage};
use preferences::PreferenceMessage;

#[derive(Default, EnumTraits!)]
pub enum Menus {
    #[default]
    File,
    Preferences,
}

impl Menus {
    pub fn menu_options(&self) -> Vec<String> {
        match self {
            Menus::File => FileMenuOptions::iter()
                .map(|variant| variant.variant_pascal_label())
                .collect(),
            Menus::Preferences => PreferenceMessage::iter()
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

pub type ToolbarResult<E> = Result<E, ToolbarErrors>;

impl fmt::Display for ToolbarErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeserializeError(error) => write!(
                formatter,
                "{}: Could not deserialize from: {}",
                self.variant(),
                error
            ),
            _ => write!(formatter, "{}: Sounds like a you problem.", self.variant()),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ToolbarMessage {
    File(FileMessage),
    Preferences(PreferenceMessage),
    ShowModal(Dialog),
    HideModal,
    Error(ToolbarErrors),
}

impl From<Dialog> for ToolbarMessage {
    fn from(dialog: Dialog) -> Self {
        Self::ShowModal(dialog)
    }
}

impl From<FileMenuOptions> for ToolbarMessage {
    fn from(menu_option: FileMenuOptions) -> Self {
        Self::File(FileMessage::OptionSelected(menu_option))
    }
}

impl FromStr for ToolbarMessage {
    type Err = ToolbarErrors;

    fn from_str(option: &str) -> ToolbarResult<Self> {
        let variant: String = option.chars().filter(|c| !c.is_whitespace()).collect();

        // Currently different menus having the same option will not be supported
        // but I think unique names will be used anyway
        if let Ok(option) = FileMenuOptions::from_str(&variant) {
            Ok(Self::File(FileMessage::OptionSelected(option)))
        } else if let Ok(message) = PreferenceMessage::from_str(&variant) {
            Ok(Self::Preferences(message))
        } else {
            Err(Self::Err::DeserializeError(variant))
        }
    }
}

impl From<ToolbarResult<Self>> for ToolbarMessage {
    fn from(result: ToolbarResult<Self>) -> Self {
        match result {
            Ok(message) => message,
            Err(error) => ToolbarMessage::Error(error),
        }
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Toolbar {
    pub dialog: Option<Dialog>,
}

impl Widget<ToolbarMessage> for Toolbar {
    fn update(
        &mut self,
        context: &mut Context,
        message: ToolbarMessage,
    ) -> iced::Task<ToolbarMessage> {
        match message {
            ToolbarMessage::File(file_message) => match file_message {
                FileMessage::OptionSelected(option) => match option {
                    FileMenuOptions::Save => match file::save(context) {
                        Ok(saved) => {
                            if !saved {
                                iced::Task::done(FileMenuOptions::SaveAs.into())
                            } else {
                                iced::Task::none()
                            }
                        }
                        Err(error) => iced::Task::done(Dialog::Error(error.to_string()).into()),
                    },
                    FileMenuOptions::SaveAs => {
                        let dialog: Option<Dialog> = match file::save_as(context) {
                            Ok(saved) => {
                                if saved {
                                    Some(if let Some(working_file) = context.working_file() {
                                        Dialog::Success(format!(
                                            "Successfully saved file at path '{}'.",
                                            working_file
                                        ))
                                    } else {
                                        Dialog::Error(
                                            "File saved, but working file was not updated."
                                                .to_string(),
                                        )
                                    })
                                } else {
                                    None
                                }
                            }
                            Err(error) => Some(Dialog::Error(error.to_string())),
                        };

                        if let Some(dialog) = dialog {
                            iced::Task::done(dialog.into())
                        } else {
                            iced::Task::none()
                        }
                    }
                    FileMenuOptions::Load => {
                        let Err(error) = file::load(context) else {
                            return iced::Task::none();
                        };
                        iced::Task::done(Dialog::Error(error.to_string()).into())
                    }
                },
                FileMessage::Error(error) => {
                    iced::Task::done(Dialog::Error(error.to_string()).into())
                }
            },
            ToolbarMessage::ShowModal(dialog) => {
                self.dialog = Some(dialog);
                iced::Task::none()
            }
            ToolbarMessage::HideModal => {
                self.dialog = None;
                iced::Task::none()
            }
            _ => iced::Task::none(),
        }
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, ToolbarMessage> {
        iced::widget::row(Menus::iter().map(|menu_option| {
            menu::DropdownMenu::new(
                menu_option.menu_options(),
                menu_option.variant(),
                |option| -> ToolbarMessage { ToolbarMessage::from_str(&option).into() },
            )
            .style(|theme: &iced::Theme, status| -> menu::Style {
                menu::from_style(theme, status, style)
            })
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
    }
}
