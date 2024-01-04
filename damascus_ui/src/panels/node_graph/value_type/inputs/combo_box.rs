use std::fmt::Display;

use strum::IntoEnumIterator;

#[derive(Clone, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ComboBox {
    pub selected: String,
    pub options: Vec<String>,
}

impl ComboBox {
    pub fn new<E: IntoEnumIterator + Display>(enumeration: E) -> Self {
        let mut options = vec![];
        for enum_option in E::iter() {
            options.push(format!("{}", enum_option));
        }
        Self {
            selected: format!("{}", enumeration),
            options: options,
        }
    }
}
