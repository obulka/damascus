use egui_modal::{Icon, Modal};

pub fn error(modal: &Modal, title: &str, body: &str) {
    let mut dialog_builder = modal
        .dialog()
        .with_title(title)
        .with_body(body)
        .with_icon(Icon::Error);
    dialog_builder.open();
}

pub fn info(modal: &Modal, title: &str, body: &str) {
    let mut dialog_builder = modal
        .dialog()
        .with_title(title)
        .with_body(body)
        .with_icon(Icon::Info);
    dialog_builder.open();
}

pub fn warning(modal: &Modal, title: &str, body: &str) {
    let mut dialog_builder = modal
        .dialog()
        .with_title(title)
        .with_body(body)
        .with_icon(Icon::Warning);
    dialog_builder.open();
}

pub fn success(modal: &Modal, title: &str, body: &str) {
    let mut dialog_builder = modal
        .dialog()
        .with_title(title)
        .with_body(body)
        .with_icon(Icon::Success);
    dialog_builder.open();
}
