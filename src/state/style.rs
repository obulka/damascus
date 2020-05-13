// 3rd Party Imports
use iced::{
    button,
    checkbox,
    Color,
    container,
    progress_bar,
    radio,
    scrollable,
    slider,
    text_input,
};

pub mod dark;
pub mod light;


const BLUE: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x76 as f32 / 255.0,
    0xBF as f32 / 255.0,
);

const PURPLE: Color = Color::from_rgb(
    0x41 as f32 / 255.0,
    0x1D as f32 / 255.0,
    0x4E as f32 / 255.0,
);

const DARK_PURPLE: Color = Color::from_rgb(
    0x14 as f32 / 255.0,
    0x10 as f32 / 255.0,
    0x27 as f32 / 255.0,
);

const TURQOISE: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x8D as f32 / 255.0,
    0x92 as f32 / 255.0,
);

const INDIGO: Color = Color::from_rgb(
    0x36 as f32 / 255.0,
    0x2F as f32 / 255.0,
    0x7D as f32 / 255.0,
);


pub enum Button {
    Primary,
    Destructive,
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
            Button::Primary => {
                match self {
                    Theme::Dark => dark::Button::Primary.into(),
                    Theme::Light => light::Button::Primary.into(),
                }
            }
            Button::Destructive => {
                match self {
                    Theme::Dark => dark::Button::Destructive.into(),
                    Theme::Light => light::Button::Destructive.into(),
                }
            }
        }
    }

    pub fn pane_style(&self, is_focused: bool) -> Box<dyn container::StyleSheet> {
        match self {
            Theme::Dark => dark::Pane{is_focused: is_focused}.into(),
            Theme::Light => light::Pane{is_focused: is_focused}.into(),
        }
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Container.into(),
        }
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Radio.into(),
        }
    }
}

impl From<Theme> for Box<dyn text_input::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::TextInput.into(),
        }
    }
}

impl From<Theme> for Box<dyn scrollable::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Scrollable.into(),
        }
    }
}

impl From<Theme> for Box<dyn slider::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Slider.into(),
        }
    }
}

impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::ProgressBar.into(),
        }
    }
}

impl From<Theme> for Box<dyn checkbox::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Checkbox.into(),
        }
    }
}
