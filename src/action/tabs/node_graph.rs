use crate::state::widget::NodeType;

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    ToggleGrid,
    AddNode { node_type: NodeType },
    ClearCache,
    ClearNodeCaches,
    ClearSelected,
    SelectNode { label: String },
}
