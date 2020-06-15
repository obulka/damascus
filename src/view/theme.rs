// 3rd Party Imports
use iced::{
    button, checkbox, container, progress_bar, radio, scrollable, slider, text_input, Color,
};

pub mod dark;
pub mod light;

use crate::view::style::tab;

pub const DARK_GREY: Color = Color::from_rgb(
    0x28 as f32 / 255.0,
    0x29 as f32 / 255.0,
    0x23 as f32 / 255.0,
);

pub const MEDIUM_GREY: Color = Color::from_rgb(
    0x42 as f32 / 255.0,
    0x43 as f32 / 255.0,
    0x3E as f32 / 255.0,
);

pub const LIGHT_GREY: Color = Color::from_rgb(
    0x6D as f32 / 255.0,
    0x6E as f32 / 255.0,
    0x6A as f32 / 255.0,
);

pub const OFF_WHITE: Color = Color::from_rgb(
    0xE4 as f32 / 255.0,
    0xEC as f32 / 255.0,
    0xE8 as f32 / 255.0,
);

pub const NODE_DEFAULT: Color = Color::from_rgb(
    0xAB as f32 / 255.0,
    0xAB as f32 / 255.0,
    0xAB as f32 / 255.0,
);

pub const BLUE: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x76 as f32 / 255.0,
    0xBF as f32 / 255.0,
);

pub const PURPLE: Color = Color::from_rgb(
    0x41 as f32 / 255.0,
    0x1D as f32 / 255.0,
    0x4E as f32 / 255.0,
);

pub const DARK_PURPLE: Color = Color::from_rgb(
    0x14 as f32 / 255.0,
    0x10 as f32 / 255.0,
    0x27 as f32 / 255.0,
);

pub const TURQOISE: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x8D as f32 / 255.0,
    0x92 as f32 / 255.0,
);

pub const INDIGO: Color = Color::from_rgb(
    0x36 as f32 / 255.0,
    0x2F as f32 / 255.0,
    0x7D as f32 / 255.0,
);

pub const ORANGE: Color = Color::from_rgb(
    0xE3 as f32 / 255.0,
    0x8E as f32 / 255.0,
    0x21 as f32 / 255.0,
);

pub const CLOSE: Color = Color::from_rgb(
    0xE7 as f32 / 255.0,
    0x5B as f32 / 255.0,
    0x2B as f32 / 255.0,
);

pub enum Button {
    Primary,
    Destructive,
    CloseTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Dark
    }
}

impl Theme {
    pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];

    pub fn button_style(&self, button: Button) -> Box<dyn button::StyleSheet> {
        match button {
            Button::Primary => match self {
                Theme::Dark => dark::Button::Primary.into(),
                Theme::Light => light::Button::Primary.into(),
            },
            Button::Destructive => match self {
                Theme::Dark => dark::Button::Destructive.into(),
                Theme::Light => light::Button::Destructive.into(),
            },
            Button::CloseTab => match self {
                Theme::Dark => dark::Button::CloseTab.into(),
                Theme::Light => light::Button::CloseTab.into(),
            },
        }
    }

    pub fn pane_style(&self, is_focused: bool) -> Box<dyn container::StyleSheet> {
        match self {
            Theme::Dark => dark::Pane {
                is_focused: is_focused,
            }
            .into(),
            Theme::Light => light::Pane {
                is_focused: is_focused,
            }
            .into(),
        }
    }

    pub fn tab_style(&self, is_focused: bool) -> Box<dyn tab::StyleSheet> {
        match self {
            Theme::Dark => dark::Tab {
                is_focused: is_focused,
            }
            .into(),
            Theme::Light => light::Tab {
                is_focused: is_focused,
            }
            .into(),
        }
    }

    pub fn tab_bar_style(&self) -> Box<dyn container::StyleSheet> {
        match self {
            Theme::Dark => dark::TabBar.into(),
            Theme::Light => light::TabBar.into(),
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            Theme::Dark => dark::TEXT_COLOR,
            Theme::Light => light::TEXT_COLOR,
        }
    }

    pub fn primary_color(&self) -> Color {
        match self {
            Theme::Dark => dark::PRIMARY,
            Theme::Light => light::PRIMARY,
        }
    }

    pub fn secondary_color(&self) -> Color {
        match self {
            Theme::Dark => dark::SECONDARY,
            Theme::Light => light::SECONDARY,
        }
    }

    pub fn tertiary_color(&self) -> Color {
        match self {
            Theme::Dark => dark::TERTIARY,
            Theme::Light => light::TERTIARY,
        }
    }
}

pub struct NodeStyle {
    pub background: Color,
    pub text_color: Color,
}

impl Default for NodeStyle {
    fn default() -> Self {
        NodeStyle {
            background: NODE_DEFAULT,
            text_color: Color::BLACK,
        }
    }
}

pub struct NodeGraphStyle {
    pub border_color: Color,
    pub border_width: f32,
    pub selected_color: Color,
    pub working_color: Color,
    pub selection_box_color: Color,
    pub selection_box_border_color: Color,
    pub selection_box_border_width: f32,
}

impl From<Theme> for NodeGraphStyle {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark => dark::NODE_GRAPH_STYLE,
            Theme::Light => light::NODE_GRAPH_STYLE,
        }
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark => dark::Container.into(),
            Theme::Light => Default::default(),
        }
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark => dark::Radio.into(),
            Theme::Light => Default::default(),
        }
    }
}

impl From<Theme> for Box<dyn text_input::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark => dark::TextInput.into(),
            Theme::Light => Default::default(),
        }
    }
}

impl From<Theme> for Box<dyn scrollable::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark => dark::Scrollable.into(),
            Theme::Light => Default::default(),
        }
    }
}

impl From<Theme> for Box<dyn slider::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark => dark::Slider.into(),
            Theme::Light => Default::default(),
        }
    }
}

impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark => dark::ProgressBar.into(),
            Theme::Light => Default::default(),
        }
    }
}

impl From<Theme> for Box<dyn checkbox::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Dark => dark::Checkbox.into(),
            Theme::Light => Default::default(),
        }
    }
}
