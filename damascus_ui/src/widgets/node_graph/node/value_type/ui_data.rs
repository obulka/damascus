// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

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
