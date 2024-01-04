use core::ops::RangeInclusive;

use crate::panels::node_graph::value_type::{RangedInput, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Float {
    value: f32,
    ui_data: Option<UIData>,
    pub range: RangeInclusive<f32>,
}

impl UIInput<f32> for Float {
    fn get_value(&self) -> &f32 {
        &self.value
    }

    fn get_ui_data(&self) -> &Option<UIData> {
        &self.ui_data
    }

    fn get_ui_data_mut(&mut self) -> &mut Option<UIData> {
        &mut self.ui_data
    }
}

impl RangedInput<f32> for Float {
    fn get_value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }

    fn with_range(value: f32, ui_data: Option<UIData>, range: RangeInclusive<f32>) -> Self {
        Self {
            value: value,
            range: range,
            ui_data: ui_data,
        }
    }

    fn get_range(&self) -> RangeInclusive<f32> {
        self.range.clone()
    }
}
