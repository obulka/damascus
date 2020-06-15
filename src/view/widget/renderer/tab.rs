// 3rd Party Imports
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::mouse;
use iced_native::{Background, Color, Element, Layout, Point, Rectangle};

// Local Imports
pub use crate::view::style::tab::StyleSheet;

pub type Tab<'a, Message, Backend> = crate::model::tabs::Tab<'a, Message, Renderer<Backend>>;

impl<B> crate::model::tabs::tab::Renderer for Renderer<B>
where
    B: Backend,
{
    const DEFAULT_PADDING: u16 = 5;

    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        defaults: &Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        style: &Box<dyn StyleSheet>,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let styling = style.style();

        let (content, _) = content.draw(self, defaults, content_layout, cursor_position);

        (
            if styling.background.is_some() || styling.border_width > 0 {
                let background = Primitive::Quad {
                    bounds,
                    background: styling
                        .background
                        .unwrap_or(Background::Color(Color::TRANSPARENT)),
                    border_radius: styling.border_radius,
                    border_width: styling.border_width,
                    border_color: styling.border_color,
                };

                Primitive::Group {
                    primitives: vec![background, content],
                }
            } else {
                content
            },
            if is_mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            },
        )
    }
}
