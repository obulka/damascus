// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt::Debug;

use meval;
use num_traits::{AsPrimitive, FromPrimitive};

mod float;

pub use float::Float;

pub trait ExpressionData<T>: Default
where
    T: FromPrimitive + Copy + Debug + 'static,
    f64: AsPrimitive<T> + From<T>,
{
    fn new(value: T) -> Self;

    fn evaluate(equation: &str) -> Option<T> {
        if let Ok(expression) = equation.parse::<meval::Expr>()
            // TODO && let Ok(function) = expression.bind("frame")
            && let Ok(evaluation) = expression.eval()
        {
            Some(evaluation.as_())
        } else {
            None
        }
    }

    fn reevaluate(&mut self) {
        if self.has_expression()
            && let Some(new_value) = Self::evaluate(self.expression())
        {
            self.set_value(new_value);
        }
    }

    fn deref(self) -> T;

    fn value(&self) -> &T;
    fn set_value(&mut self, new_value: T);

    fn with_value(mut self, new_value: T) -> Self
    where
        Self: Sized,
    {
        self.set_value(new_value);
        self
    }

    fn has_expression(&self) -> bool {
        !self.expression().is_empty()
    }

    fn expression(&self) -> &str {
        ""
    }

    fn set_expression(&mut self, new_expression: String) {
        if let Some(value) = Self::evaluate(&new_expression) {
            self.set_value(value);
        }
    }

    fn with_expression(mut self, new_expression: String) -> Self
    where
        Self: Sized,
    {
        self.set_expression(new_expression);
        self
    }
}
