// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use egui_modal::{Icon, Modal};

pub fn error(modal: &Modal, title: &str, body: &str) {
    modal
        .dialog()
        .with_title(title)
        .with_body(body)
        .with_icon(Icon::Error)
        .open();
}

pub fn info(modal: &Modal, title: &str, body: &str) {
    modal
        .dialog()
        .with_title(title)
        .with_body(body)
        .with_icon(Icon::Info)
        .open();
}

pub fn warning(modal: &Modal, title: &str, body: &str) {
    modal
        .dialog()
        .with_title(title)
        .with_body(body)
        .with_icon(Icon::Warning)
        .open();
}

pub fn success(modal: &Modal, title: &str, body: &str) {
    modal
        .dialog()
        .with_title(title)
        .with_body(body)
        .with_icon(Icon::Success)
        .open();
}
