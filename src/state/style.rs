// 3rd Party Imports
use iced::{
    Background,
    button,
    Color,
    container,
    Vector,
};


const SURFACE: Color = Color::from_rgb(
    0xF2 as f32 / 255.0,
    0xF3 as f32 / 255.0,
    0xF5 as f32 / 255.0,
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


pub struct Pane {
    pub is_focused: bool,
}

impl container::StyleSheet for Pane {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(SURFACE)),
            border_width: 2,
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
            Button::Primary => (Some(ACTIVE), Color::WHITE),
            Button::Destructive => {
                (None, Color::from_rgb8(0xFF, 0x47, 0x47))
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
            Button::Primary => Some(HOVERED),
            Button::Destructive => Some(Color {
                a: 0.2,
                ..active.text_color
            }),
        };

        button::Style {
            background: background.map(Background::Color),
            ..active
        }
    }
}
