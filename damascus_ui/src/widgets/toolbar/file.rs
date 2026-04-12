// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use macro_rules_attribute::derive;

use crate::{EnumTraits, app::Context};

pub fn save(context: &mut Context, success_dialog: bool) -> bool {
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

pub fn save_as(context: &mut Context, success_dialog: bool) -> bool {
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
pub enum FileMessage {
    #[default]
    Save,
    SaveAs,
    Load,
}
