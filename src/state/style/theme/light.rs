use iced::{
    button,
    Background,
    container,
    Color,
    Vector,
};

use crate::state::style::CLOSE;


const SECONDARY: Color = Color::from_rgb(
    0x42 as f32 / 255.0,
    0x43 as f32 / 255.0,
    0x3E as f32 / 255.0,
);

const TERTIARY: Color = Color::from_rgb(
    0x6D as f32 / 255.0,
    0x6E as f32 / 255.0,
    0x6A as f32 / 255.0,
);

const ACTIVE: Color = Color::from_rgb(
    0x72 as f32 / 255.0,
    0x89 as f32 / 255.0,
    0xDA as f32 / 255.0,
);

const HOVERED: Color = Color::from_rgb(
    0x67 as f32 / 255.0,
    0x7B as f32 / 255.0,
    0xC4 as f32 / 255.0,
);

const TEXT_COLOR: Color = Color::WHITE;


pub struct TabBar;

impl container::StyleSheet for TabBar {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(TERTIARY)),
            border_width: 1,
            border_color: Color::TRANSPARENT,
            ..container::Style::default()
        }
    }
}

pub struct Pane {
    pub is_focused: bool,
}

impl container::StyleSheet for Pane {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::WHITE)),
            border_width: 1,
            border_color: Color {
                a: if self.is_focused { 1.0 } else { 0.3 },
                ..Color::BLACK
            },
            ..Default::default()
        }
    }
}

pub enum Button {
    Primary,
    Destructive,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        let (background, text_color) = match self {
            Button::Primary => (Some(SECONDARY), TEXT_COLOR),
            Button::Destructive => {
                (Some(CLOSE), Color::BLACK)
            }
        };

        button::Style {
            text_color,
            background: background.map(Background::Color),
            border_radius: 5,
            shadow_offset: Vector::new(0.0, 0.0),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        let background = match self {
            Button::Primary => Some(Color {
                a: 0.6,
                ..SECONDARY
            }),
            Button::Destructive => Some(Color {
                a: 0.6,
                ..CLOSE
            }),
        };

        button::Style {
            background: background.map(Background::Color),
            ..active
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            border_width: 1,
            border_color: Color::WHITE,
            ..self.hovered()
        }
    }
}