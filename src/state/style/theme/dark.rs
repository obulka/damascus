use iced::{
    Background,
    button,
    checkbox,
    Color,
    container,
    progress_bar,
    radio,
    scrollable,
    slider,
    text_input,
    Vector,
};

use crate::state::style::{CLOSE, tab};


const ACCENT: Color = Color::from_rgb(
    0x6F as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xE9 as f32 / 255.0,
);

const HIGHLIGHT: Color = Color::from_rgb(
    0xE3 as f32 / 255.0,
    0x8E as f32 / 255.0,
    0x21 as f32 / 255.0,
);

const PRIMARY: Color = Color::from_rgb(
    0x28 as f32 / 255.0,
    0x29 as f32 / 255.0,
    0x23 as f32 / 255.0,
);

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

pub const TEXT_COLOR: Color = Color::WHITE;


pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(TERTIARY)),
            ..container::Style::default()
        }
    }
}

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

pub struct Tab {
    pub is_focused: bool,
}

impl tab::StyleSheet for Tab {
    fn style(&self) -> tab::Style {
        tab::Style {
            background: Some(Background::Color(if self.is_focused {PRIMARY} else {SECONDARY})),
            border_width: 1,
            border_color: Color::TRANSPARENT,
            text_color: TEXT_COLOR,
            ..tab::Style::default()
        }
    }
}

pub struct Pane {
    pub is_focused: bool,
}

impl container::StyleSheet for Pane {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(PRIMARY)),
            border_width: 1,
            border_color: if self.is_focused { HIGHLIGHT } else { Color::BLACK },
            ..Default::default()
        }
    }
}

pub enum Button {
    Primary,
    Destructive,
    CloseTab,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        let (background, text_color, border_radius) = match self {
            Button::Primary => (Some(SECONDARY), TEXT_COLOR, 5),
            Button::Destructive => (Some(SECONDARY), TEXT_COLOR, 5),
            Button::CloseTab => (Some(Color::TRANSPARENT), TEXT_COLOR, 5),
        };

        button::Style {
            text_color,
            background: background.map(Background::Color),
            border_radius: border_radius,
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
            Button::Destructive | Button::CloseTab => Some(CLOSE),
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


pub struct Radio;

impl radio::StyleSheet for Radio {
    fn active(&self) -> radio::Style {
        radio::Style {
            background: Background::Color(PRIMARY),
            dot_color: ACTIVE,
            border_width: 1,
            border_color: ACTIVE,
        }
    }

    fn hovered(&self) -> radio::Style {
        radio::Style {
            background: Background::Color(Color { a: 0.5, ..PRIMARY }),
            ..self.active()
        }
    }
}

pub struct TextInput;

impl text_input::StyleSheet for TextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(PRIMARY),
            border_radius: 2,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            border_width: 1,
            border_color: ACCENT,
            ..self.active()
        }
    }

    fn hovered(&self) -> text_input::Style {
        text_input::Style {
            border_width: 1,
            border_color: Color { a: 0.3, ..ACCENT },
            ..self.focused()
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.4, 0.4, 0.4)
    }

    fn value_color(&self) -> Color {
        Color::WHITE
    }

    fn selection_color(&self) -> Color {
        ACTIVE
    }
}


pub struct Scrollable;

impl scrollable::StyleSheet for Scrollable {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(PRIMARY)),
            border_radius: 2,
            border_width: 0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: ACTIVE,
                border_radius: 2,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> scrollable::Scrollbar {
        let active = self.active();

        scrollable::Scrollbar {
            background: Some(Background::Color(Color {
                a: 0.9,
                ..PRIMARY
            })),
            scroller: scrollable::Scroller {
                color: HOVERED,
                ..active.scroller
            },
            ..active
        }
    }

    fn dragging(&self) -> scrollable::Scrollbar {
        let hovered = self.hovered();

        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                color: Color::from_rgb(0.85, 0.85, 0.85),
                ..hovered.scroller
            },
            ..hovered
        }
    }
}

pub struct Slider;

impl slider::StyleSheet for Slider {
    fn active(&self) -> slider::Style {
        slider::Style {
            rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 9 },
                color: ACTIVE,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> slider::Style {
        let active = self.active();

        slider::Style {
            handle: slider::Handle {
                color: HOVERED,
                ..active.handle
            },
            ..active
        }
    }

    fn dragging(&self) -> slider::Style {
        let active = self.active();

        slider::Style {
            handle: slider::Handle {
                color: Color::from_rgb(0.85, 0.85, 0.85),
                ..active.handle
            },
            ..active
        }
    }
}

pub struct ProgressBar;

impl progress_bar::StyleSheet for ProgressBar {
    fn style(&self) -> progress_bar::Style {
        progress_bar::Style {
            background: Background::Color(PRIMARY),
            bar: Background::Color(ACTIVE),
            border_radius: 10,
        }
    }
}

pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    fn active(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(if is_checked {
                ACTIVE
            } else {
                PRIMARY
            }),
            checkmark_color: Color::WHITE,
            border_radius: 2,
            border_width: 1,
            border_color: ACTIVE,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(Color {
                a: 0.8,
                ..if is_checked { ACTIVE } else { PRIMARY }
            }),
            ..self.active(is_checked)
        }
    }
}

// pub struct DropDown;

// impl crate::state::style::drop_down::StyleSheet for DropDown {
//     fn active(&self) -> drop_down::Style {
//         let (background, text_color) =  (Some(ACTIVE), TEXT_COLOR);

//         drop_down::Style {
//             text_color,
//             background: background.map(Background::Color),
//             border_radius: 5,
//             shadow_offset: Vector::new(0.0, 0.0),
//             ..drop_down::Style::default()
//         }
//     }

//     fn hovered(&self) -> drop_down::Style {
//         let active = self.active();

//         let background = Some(HOVERED);

//         drop_down::Style {
//             background: background.map(Background::Color),
//             ..active
//         }
//     }

//     fn pressed(&self) -> drop_down::Style {
//         drop_down::Style {
//             border_width: 1,
//             border_color: Color::WHITE,
//             ..self.hovered()
//         }
//     }
// }
