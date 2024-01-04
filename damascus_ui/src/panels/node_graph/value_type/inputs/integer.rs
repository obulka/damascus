use core::ops::RangeInclusive;

use crate::panels::node_graph::value_type::{RangedInput, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Integer {
    pub value: i32,
    pub ui_data: Option<UIData>,
    pub range: RangeInclusive<i32>,
}

impl UIInput<i32> for Integer {
    fn get_value(&self) -> i32 {
        self.value
    }

    fn get_value_mut(&mut self) -> &mut i32 {
        &mut self.value
    }

    fn get_ui_data(&self) -> &Option<UIData> {
        &self.ui_data
    }

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData> {
        &mut self.ui_data
    }
}

impl RangedInput<i32> for Integer {
    fn with_range(value: i32, ui_data: Option<UIData>, range: RangeInclusive<i32>) -> Self {
        Self {
            value: value,
            ui_data: ui_data,
            range: range,
        }
    }

    fn get_range(&self) -> RangeInclusive<i32> {
        self.range.clone()
    }
}
