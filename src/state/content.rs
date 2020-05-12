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
use iced_native::{layout, mouse, Background, Color, Hasher, Layout,
        Point, Size, Widget,};
use iced_wgpu::{Defaults, Primitive, Renderer};

// Local Imports
use crate::action::Message;
use crate::state::style;


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
                style::Button::Primary,
            ))
            .push(button(
                split_vertically,
                "Split vertically",
                Message::Split(pane_grid::Axis::Vertical, pane),
                style::Button::Primary,
            ));

        if total_panes > 1 {
            controls = controls.push(button(
                close,
                "Close",
                Message::Close(pane),
                style::Button::Destructive,
            ));
        }

        let content = Scrollable::new(scroll)
            .width(Length::Fill)
            .spacing(10)
            .align_items(Align::Center)
            .push(Text::new(format!("Pane {}", id)).size(30))
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_y()
            .style(style::Pane {
                is_focused: focus.is_some(),
            })
            .into()
    }
}

impl<Message> Widget<Message, Renderer> for Content
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        unimplemented!();
    }

    fn hash_layout(&self, _state: &mut Hasher) {
        unimplemented!();
    }

    fn draw(
        &self,
        _renderer: &mut Renderer,
        _defaults: &Defaults,
        _layout: Layout<'_>,
        _cursor_position: Point,
    ) -> (Primitive, mouse::Interaction) {
        unimplemented!();
    }
}

impl<'a, Message> Into<Element<'a, Message>> for Content {
    fn into(self) -> Element<'a, Message> {
        Element::new(self)
    }
}
