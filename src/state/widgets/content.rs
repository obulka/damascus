// 3rd Party Imports
use iced::{
    Align,
    Button,
    button,
    Column,
    Container,
    Element,
    HorizontalAlignment,
    Length,
    pane_grid,
    Scrollable,
    scrollable,
    Text,
};

// Local Imports
use crate::action::Message;
use crate::state::style::{self, Theme};


pub struct Content {
    id: usize,
    scroll: scrollable::State,
    split_horizontally: button::State,
    split_vertically: button::State,
    close: button::State,
}

impl Content {
    pub fn new(id: usize) -> Self {
        Content {
            id,
            scroll: scrollable::State::new(),
            split_horizontally: button::State::new(),
            split_vertically: button::State::new(),
            close: button::State::new(),
        }
    }

    pub fn view(
        &mut self,
        pane: pane_grid::Pane,
        focus: Option<pane_grid::Focus>,
        total_panes: usize,
        theme: Theme,
    ) -> Element<Message> {
        let Content {
            id,
            scroll,
            split_horizontally,
            split_vertically,
            close,
        } = self;

        let button = |state, label, message, style| {
            Button::new(
                state,
                Text::new(label)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(16),
            )
            .width(Length::Fill)
            .padding(8)
            .on_press(message)
            .style(style)
        };

        let mut controls = Column::new()
            .spacing(5)
            .max_width(150)
            .push(button(
                split_horizontally,
                "Split horizontally",
                Message::Split(pane_grid::Axis::Horizontal, pane),
                theme.button_style(
                    style::Button::Primary,
                ),
            ))
            .push(button(
                split_vertically,
                "Split vertically",
                Message::Split(pane_grid::Axis::Vertical, pane),
                theme.button_style(
                    style::Button::Primary,
                ),
            ));

        if total_panes > 1 {
            controls = controls.push(button(
                close,
                "Close",
                Message::Close(pane),
                theme.button_style(
                    style::Button::Destructive,
                ),
            ));
        }

        let content = Scrollable::new(scroll)
            .width(Length::Fill)
            .spacing(10)
            .align_items(Align::Center)
            .push(Text::new(format!("Pane {}", id)).size(30))
            .push(controls)
            .style(theme);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_y()
            .style(theme.pane_style(focus.is_some()))
            .into()
    }
}
