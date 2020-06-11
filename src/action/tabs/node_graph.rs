use crate::state::widget::NodeType;

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    ToggleGrid,
    ClearCache,
    ClearNodeCaches,
    ClearSelected,
    AddNode(NodeType),
    DeselectNode(String),
    SelectNode(String),
}
