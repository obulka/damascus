use core::ops::RangeInclusive;

use crate::panels::node_graph::value_type::{RangedInput, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct UnsignedInteger {
    pub value: u32,
    pub ui_data: Option<UIData>,
    pub range: RangeInclusive<u32>,
}

impl UIInput<u32> for UnsignedInteger {
    fn get_value(&self) -> &u32 {
        &self.value
    }

    fn get_ui_data(&self) -> &Option<UIData> {
        &self.ui_data
    }

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData> {
        &mut self.ui_data
    }
}

impl RangedInput<u32> for UnsignedInteger {
    fn get_value_mut(&mut self) -> &mut u32 {
        &mut self.value
    }

    fn with_range(value: u32, ui_data: Option<UIData>, range: RangeInclusive<u32>) -> Self {
        Self {
            value: value,
            range: range,
            ui_data: ui_data,
        }
    }

    fn get_range(&self) -> RangeInclusive<u32> {
        self.range.clone()
    }
}
