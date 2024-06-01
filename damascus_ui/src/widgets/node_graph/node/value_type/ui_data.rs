// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct UIData {
    tooltip: Option<String>,
    hidden: bool,
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
    pub fn with_tooltip(mut self, tooltip: &str) -> Self {
        self.tooltip = if tooltip.is_empty() {
            None
        } else {
            Some(tooltip.to_string())
        };
        self
    }

    pub fn tooltip(&self) -> &Option<String> {
        &self.tooltip
    }

    pub fn with_hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    pub fn hide(&mut self) {
        self.hidden = true;
    }

    pub fn show(&mut self) {
        self.hidden = false;
    }

    pub fn hidden(&self) -> &bool {
        &self.hidden
    }
}
