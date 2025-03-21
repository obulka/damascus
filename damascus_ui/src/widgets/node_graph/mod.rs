// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui;
use egui_node_graph::{GraphResponse, NodeResponse};

mod graph;
pub mod node;
mod response;
mod state;

pub use graph::{evaluate_node, Graph};
use node::{AllNodeTemplates, NodeData};
pub use response::NodeGraphResponse;
pub use state::{NodeGraphEditorState, NodeGraphState};

pub struct NodeGraph {
    editor_state: NodeGraphEditorState,
    user_state: NodeGraphState,
}

impl NodeGraph {
    pub fn new(editor_state: NodeGraphEditorState) -> Self {
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
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let style = ui.style_mut();
                style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
                style.visuals.widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;

                let (_id, mut rect) = ui.allocate_space(ui.available_size());
                rect.min.y -= 8.;
                ui.set_clip_rect(rect);

                if ctx.memory(|memory| memory.focused().is_none())
                    && ui.ctx().input(|input| input.key_pressed(egui::Key::F))
                {
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
                            output
                                .commands
                                .push(egui::OutputCommand::CopyText(serialized_state));
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
            .inner
    }
}
