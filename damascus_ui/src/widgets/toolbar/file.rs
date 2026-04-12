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

use crate::{EnumTraits, ErrorTraits, app::Context};

#[derive(Default, ErrorTraits!)]
pub enum FileErrors {
    FileCreationError(String),
    SerializationError(String),
    FileWriteError(String),
    #[default]
    UnknownError,
}

impl fmt::Display for FileErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileCreationError(file_path) => write!(
                formatter,
                "{}: Could not create file at: {:?}",
                self, file_path
            ),
            Self::SerializationError(error) => write!(formatter, "{}: {:?}", self, error),
            Self::FileWriteError(file_path) => write!(
                formatter,
                "{}: Could not write to file: {:?}",
                self, file_path
            ),
            _ => write!(formatter, "{}", self),
        }
    }
}

pub type FileResult<E> = Result<E, FileErrors>;

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

pub fn load(context: &mut Context, success_dialog: bool) -> bool {
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
