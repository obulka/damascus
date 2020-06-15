// 3rd Party Imports
use iced_core::{Background, Color};

/// The appearance of a tab.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub background: Option<Background>,
    pub border_radius: u16,
    pub border_width: u16,
    pub border_color: Color,
    pub text_color: Color,
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        }
    }
}

/// A set of rules that dictate the style of a tab.
pub trait StyleSheet {
    fn style(&self) -> Style;
}

struct Default;

impl StyleSheet for Default {
    fn style(&self) -> Style {
        Style {
            background: Some(Background::Color([0.87, 0.87, 0.87].into())),
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        }
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
