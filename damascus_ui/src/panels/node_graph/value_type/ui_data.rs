#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct UIData {
    pub tooltip: Option<String>,
    pub hidden: bool,
}

impl Default for UIData {
    fn default() -> Self {
        Self {
            tooltip: None,
            hidden: false,
        }
    }
}

impl UIData {
    pub fn tooltip(mut self, tooltip: &str) -> Self {
        self.tooltip = if tooltip.is_empty() {
            None
        } else {
            Some(tooltip.to_string())
        };
        self
    }

    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
}
