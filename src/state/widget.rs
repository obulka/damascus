pub mod panel;
pub mod tab;
pub mod tabs;

pub use tab::Tab;

#[derive(Debug, Clone)]
pub enum TabType {
    NodeGraph,
    Viewer,
}
