// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.
use eframe::egui;
use egui_node_graph::{GraphResponse, NodeResponse};

mod data_type;
mod graph;
mod node;
mod response;
mod state;
mod value_type;

pub use data_type::NodeDataType;
pub use graph::{evaluate_node, Graph};
pub use node::{AllNodeTemplates, NodeCallbacks, NodeData, NodeTemplate};
pub use response::NodeGraphResponse;
pub use state::{NodeGraphEditorState, NodeGraphState};
pub use value_type::{Bool, NodeValueType, UIInput};

pub struct NodeGraph {
    editor_state: NodeGraphEditorState,
    user_state: NodeGraphState,
}

impl NodeGraph {
    pub fn new(creation_context: &eframe::CreationContext<'_>, persistence_key: &str) -> Self {
        let editor_state: NodeGraphEditorState = creation_context
            .storage
            .and_then(|storage| eframe::get_value(storage, persistence_key))
            .unwrap_or_default();
        Self {
            editor_state,
            user_state: NodeGraphState::default(),
        }
    }

    pub fn clear(&mut self) {
        self.editor_state = NodeGraphEditorState::default();
        self.user_state = NodeGraphState::default();
    }

    pub fn set_editor_state(&mut self, editor_state: NodeGraphEditorState) {
        self.user_state = NodeGraphState::default();
        self.editor_state = editor_state;
    }

    pub fn editor_state(&self) -> &NodeGraphEditorState {
        &self.editor_state
    }

    pub fn editor_state_mut(&mut self) -> &mut NodeGraphEditorState {
        &mut self.editor_state
    }

    pub fn user_state(&self) -> &NodeGraphState {
        &self.user_state
    }

    pub fn user_state_mut(&mut self) -> &mut NodeGraphState {
        &mut self.user_state
    }

    pub fn show(&mut self, ctx: &egui::Context) -> GraphResponse<NodeGraphResponse, NodeData> {
        egui::SidePanel::right("right")
            .resizable(true)
            .show_separator_line(false)
            .default_width(0.)
            .show(ctx, |ui| {
                ui.allocate_space(ui.available_size());
                self.editor_state.graph_editor_interaction(ui);
                if ui.rect_contains_pointer(ui.max_rect()) {
                    self.editor_state.pan_zoom.enable_zoom_from_out_of_rect = true;
                }
            });
        egui::SidePanel::left("left")
            .resizable(true)
            .show_separator_line(false)
            .default_width(0.)
            .show(ctx, |ui| {
                ui.allocate_space(ui.available_size());
                self.editor_state.graph_editor_interaction(ui);
                if ui.rect_contains_pointer(ui.max_rect()) {
                    self.editor_state.pan_zoom.enable_zoom_from_out_of_rect = true;
                }
            });
        let graph_response = egui::TopBottomPanel::bottom("bottom")
            .resizable(true)
            .default_height(300.0)
            .show(ctx, |ui| {
                ui.allocate_space(ui.available_size());
                if ui.ctx().input(|i| i.key_pressed(egui::Key::F)) {
                    self.editor_state.reset_zoom(ui);
                }

                let mut copy_selected: bool = false;
                let mut pasted = String::new();

                ui.ctx().input(|input| {
                    for event in input.events.iter() {
                        match event {
                            egui::Event::Copy => {
                                // App freezes if we set copied_text here, so flag and do it later
                                copy_selected = true;
                            }
                            egui::Event::Paste(text) => {
                                // App freezes if we paste here, so clone the text and do it later
                                pasted = text.to_string();
                            }
                            _ => {}
                        }
                    }
                });

                let mut responses = vec![];

                if copy_selected {
                    if let Ok(serialized_state) =
                        serde_json::to_string_pretty(&self.editor_state.from_selected())
                    {
                        ui.output_mut(|output| {
                            output.copied_text = serialized_state;
                        });
                    }
                } else if !pasted.is_empty() {
                    if let Ok(mut deserialized_editor_state) = serde_json::from_str(&pasted) {
                        let new_nodes = self.editor_state.merge(ui, &mut deserialized_editor_state);
                        for node_id in new_nodes.into_iter() {
                            responses.push(NodeResponse::CreatedNode(node_id));
                        }
                    }
                }

                self.editor_state.draw_graph_editor(
                    ui,
                    AllNodeTemplates,
                    &mut self.user_state,
                    responses,
                )
            })
            .inner;
        self.editor_state.pan_zoom.enable_zoom_from_out_of_rect = false;

        graph_response
    }
}
