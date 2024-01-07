use egui_node_graph::{NodeId, UserResponseTrait};

use super::node_template::DamascusNodeTemplate;

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DamascusResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
    InputValueChanged(NodeId, DamascusNodeTemplate, String),
}

impl UserResponseTrait for DamascusResponse {}
