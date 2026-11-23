// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use super::ExpressionData;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Float {
    value: f32,
    expression: String,
}

impl Default for Float {
    fn default() -> Self {
        Self::new(0.)
    }
}

impl ExpressionData<f32> for Float {
    fn new(value: f32) -> Self {
        Self {
            value,
            expression: String::new(),
        }
    }

    fn value(&self) -> &f32 {
        &self.value
    }

    fn deref(self) -> f32 {
        self.value
    }

    fn set_value(&mut self, new_value: f32) {
        self.value = new_value;
    }

    fn expression(&self) -> &str {
        &self.expression
    }

    fn set_expression(&mut self, new_expression: String) {
        self.expression = new_expression;
        self.reevaluate();
    }
}
