#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct UIData {
    pub tooltip: Option<String>,
}

impl Default for UIData {
    fn default() -> Self {
        Self { tooltip: None }
    }
}

impl UIData {
    pub fn new(tooltip: &str) -> Self {
        Self {
            tooltip: if tooltip.is_empty() {
                None
            } else {
                Some(tooltip.to_string())
            },
        }
    }
}
