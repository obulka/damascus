use eframe::egui;
use egui_node_graph::{Graph, NodeDataTrait, NodeId, NodeResponse, UserResponseTrait};

use crate::panels::node_graph::{
    data_type::DamascusDataType, node_graph_state::DamascusGraphState,
    node_template::DamascusNodeTemplate, response::DamascusResponse, value_type::DamascusValueType,
};

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DamascusNodeData {
    pub template: DamascusNodeTemplate,
}

impl NodeDataTrait for DamascusNodeData {
    type Response = DamascusResponse;
    type UserState = DamascusGraphState;
    type DataType = DamascusDataType;
    type ValueType = DamascusValueType;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<DamascusNodeData, DamascusDataType, DamascusValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<DamascusResponse, DamascusNodeData>>
    where
        DamascusResponse: UserResponseTrait,
    {
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        // Pressing the button will emit a custom user response to either set,
        // or clear the active node. These responses do nothing by themselves,
        // the library only makes the responses available to you after the graph
        // has been drawn. See below at the update method for an example.
        if !is_active {
            if ui.button("👁 Set active").clicked() {
                responses.push(NodeResponse::User(DamascusResponse::SetActiveNode(node_id)));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(DamascusResponse::ClearActiveNode));
            }
        }

        responses
    }
}
