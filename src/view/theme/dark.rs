// 3rd Party Imports
use iced::{
    button, checkbox, container, progress_bar, radio, scrollable, slider, text_input, Background,
    Color, Vector,
};

// Local Imports
use crate::view::theme::{
    tab, NodeGraphStyle, CLOSE, DARK_GREY, LIGHT_GREY, MEDIUM_GREY, ORANGE, TURQOISE,
};

const ACCENT: Color = Color::from_rgb(
    0x6F as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xE9 as f32 / 255.0,
);

const HIGHLIGHT: Color = ORANGE;

pub const PRIMARY: Color = DARK_GREY;

pub const SECONDARY: Color = MEDIUM_GREY;

pub const TERTIARY: Color = LIGHT_GREY;

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

pub const NODE_GRAPH_STYLE: NodeGraphStyle = NodeGraphStyle {
    border_color: Color::BLACK,
    border_width: 2.0,
    selected_color: HIGHLIGHT,
    working_color: TURQOISE,
    selection_box_color: Color { a: 0.5, ..TERTIARY },
    selection_box_border_color: SECONDARY,
    selection_box_border_width: 2.0,
};

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
            background: Some(Background::Color(if self.is_focused {
                PRIMARY
            } else {
                SECONDARY
            })),
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
            border_color: if self.is_focused {
                HIGHLIGHT
            } else {
                Color::BLACK
            },
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
            background: Some(Background::Color(Color { a: 0.9, ..PRIMARY })),
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
            background: Background::Color(if is_checked { ACTIVE } else { PRIMARY }),
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
