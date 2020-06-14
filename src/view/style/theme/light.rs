use iced::{button, container, Background, Color, Vector};

use crate::view::style::{
    tab, NodeGraphStyle, CLOSE, DARK_GREY, LIGHT_GREY, OFF_WHITE, ORANGE, PURPLE,
};

const HIGHLIGHT: Color = ORANGE;

pub const PRIMARY: Color = OFF_WHITE;

pub const SECONDARY: Color = LIGHT_GREY;

pub const TERTIARY: Color = DARK_GREY;

pub const TEXT_COLOR: Color = Color::BLACK;

pub const NODE_GRAPH_STYLE: NodeGraphStyle = NodeGraphStyle {
    border_color: Color::BLACK,
    border_width: 2.0,
    selected_color: HIGHLIGHT,
    working_color: PURPLE,
    selection_box_color: Color { a: 0.5, ..TERTIARY },
    selection_box_border_color: SECONDARY,
    selection_box_border_width: 2.0,
};

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
            border_color: Color::BLACK,
            ..self.hovered()
        }
    }
}
