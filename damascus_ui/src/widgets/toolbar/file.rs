// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    fmt,
    fs::File,
    io::{BufReader, Read, Write},
};

use macro_rules_attribute::derive;

use damascus::Enumerator;

use crate::{EnumTraits, ErrorTraits, app::Context};

#[derive(Default, ErrorTraits!)]
pub enum FileErrors {
    FileCreationError(String),
    DeserializationError(String),
    SerializationError(String),
    FileWriteError(String),
    FileOpenError(String),
    FileReadError(String),
    #[default]
    UnknownError,
}

impl fmt::Display for FileErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileCreationError(file_path) => write!(
                formatter,
                "{}: Could not create file at '{}'",
                self.variant(),
                file_path
            ),
            Self::DeserializationError(error) => {
                write!(formatter, "{}: {}", self.variant(), error)
            }
            Self::SerializationError(error) => write!(formatter, "{}: {}", self.variant(), error),
            Self::FileWriteError(file_path) => write!(
                formatter,
                "{}: Could not write to file '{}'",
                self.variant(),
                file_path
            ),
            Self::FileOpenError(file_path) => {
                write!(
                    formatter,
                    "{}: Could not open file '{}'",
                    self.variant(),
                    file_path
                )
            }
            Self::FileReadError(file_path) => {
                write!(
                    formatter,
                    "{}: Could not read file '{}'",
                    self.variant(),
                    file_path
                )
            }
            _ => write!(formatter, "{}: Fuck you.", self.variant()),
        }
    }
}

pub type FileResult<E> = Result<E, FileErrors>;

#[derive(Default, EnumTraits!)]
pub enum FileMenuOptions {
    #[default]
    Save,
    SaveAs,
    Load,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum FileMessage {
    OptionSelected(FileMenuOptions),
    Error(FileErrors),
}

pub fn save(context: &mut Context) -> FileResult<bool> {
    let Some(file_path) = context.working_file() else {
        return Ok(false);
    };

    let Ok(mut file) = File::create(file_path) else {
        return Err(FileErrors::FileCreationError(file_path.to_string()));
    };
    match serde_json::to_string_pretty(context) {
        Ok(serialization) => {
            let Ok(_) = file.write_all(serialization.as_bytes()) else {
                return Err(FileErrors::FileWriteError(file_path.to_string()));
            };

            Ok(true)
        }
        Err(error) => Err(FileErrors::SerializationError(error.to_string())),
    }
}

pub fn save_as(context: &mut Context) -> FileResult<bool> {
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
        save(context)
    } else {
        Ok(false)
    }
}

pub fn load(context: &mut Context) -> FileResult<bool> {
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
            return Err(FileErrors::FileOpenError(file_path));
        };
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        let Ok(_) = buf_reader.read_to_string(&mut contents) else {
            return Err(FileErrors::FileReadError(file_path));
        };

        match serde_json::from_str(&contents) {
            Ok(state) => {
                *context = state;
                Ok(true)
            }
            Err(error) => Err(FileErrors::DeserializationError(error.to_string())),
        }
    } else {
        Ok(false)
    }
}
