// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use meval;
use num_traits::{AsPrimitive, FromPrimitive};

mod float;
mod mat4;
mod vec4;

pub use float::Float;
pub use mat4::{Mat4, Mat4f};
pub use vec4::{Vec4, Vec4f};

pub trait ExpressionData<T>:
    Add
    + Add<T>
    + AddAssign
    + AddAssign<T>
    + Default
    + Div
    + Div<T>
    + DivAssign
    + DivAssign<T>
    + From<T>
    + Mul
    + Mul<T>
    + MulAssign
    + MulAssign<T>
    + Sub
    + Sub<T>
    + SubAssign
    + SubAssign<T>
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

    fn value(&self) -> T;
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

    fn set_expression(&mut self, expression: String) {
        if let Some(value) = Self::evaluate(&expression) {
            self.set_value(value);
        }
    }

    fn with_expression(mut self, expression: String) -> Self
    where
        Self: Sized,
    {
        self.set_expression(expression);
        self
    }
}
