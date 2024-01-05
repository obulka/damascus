use core::ops::RangeInclusive;

use crate::panels::node_graph::value_type::{RangedInput, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Integer {
    value: i32,
    ui_data: UIData,
    pub range: RangeInclusive<i32>,
}

impl Default for Integer {
    fn default() -> Self {
        Self {
            value: 0,
            ui_data: UIData::default(),
            range: -10..=10,
        }
    }
}

impl UIInput<i32> for Integer {
    fn new(value: i32) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &i32 {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}

impl RangedInput<i32> for Integer {
    fn value_mut(&mut self) -> &mut i32 {
        &mut self.value
    }

    fn range_mut(&mut self) -> &mut RangeInclusive<i32> {
        &mut self.range
    }

    fn range(&self) -> RangeInclusive<i32> {
        self.range.clone()
    }
}
