pub mod node;
pub mod panel;
pub mod tab;
pub mod tabs;

pub use node::Node;
pub use tab::Tab;

#[derive(Debug, Clone)]
pub enum TabType {
    NodeGraph,
    Viewer,
}
