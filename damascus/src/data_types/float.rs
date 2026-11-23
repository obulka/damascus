// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl Add for Float {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.deref() + other.deref())
    }
}

impl Add<f32> for Float {
    type Output = f32;

    fn add(self, other: f32) -> Self::Output {
        self.deref() + other
    }
}

impl AddAssign<f32> for Float {
    fn add_assign(&mut self, other: f32) {
        *self = Self::new(self.value() + other);
    }
}

impl AddAssign for Float {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.value() + other.value());
    }
}

impl Div for Float {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(self.deref() / other.deref())
    }
}

impl Div<f32> for Float {
    type Output = f32;

    fn div(self, other: f32) -> Self::Output {
        self.deref() / other
    }
}

impl DivAssign<f32> for Float {
    fn div_assign(&mut self, other: f32) {
        *self = Self::new(self.value() / other);
    }
}

impl DivAssign for Float {
    fn div_assign(&mut self, other: Self) {
        *self = Self::new(self.value() / other.value());
    }
}

impl Mul for Float {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.deref() * other.deref())
    }
}

impl Mul<f32> for Float {
    type Output = f32;

    fn mul(self, other: f32) -> Self::Output {
        self.deref() * other
    }
}

impl MulAssign<f32> for Float {
    fn mul_assign(&mut self, other: f32) {
        *self = Self::new(self.value() * other);
    }
}

impl MulAssign for Float {
    fn mul_assign(&mut self, other: Self) {
        *self = Self::new(self.value() * other.value());
    }
}

impl Sub for Float {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.deref() - other.deref())
    }
}

impl Sub<f32> for Float {
    type Output = f32;

    fn sub(self, other: f32) -> Self::Output {
        self.deref() - other
    }
}

impl SubAssign<f32> for Float {
    fn sub_assign(&mut self, other: f32) {
        *self = Self::new(self.value() - other);
    }
}

impl SubAssign for Float {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::new(self.value() - other.value());
    }
}

impl ExpressionData<f32> for Float {
    fn new(value: f32) -> Self {
        Self {
            value,
            expression: String::new(),
        }
    }

    fn value(&self) -> f32 {
        self.value
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
