// 3rd Party Imports
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{layout, mouse, Background, Color, Element, Layout, Point, Rectangle};

// Local Imports
pub use crate::view::{style::tab::StyleSheet, WidgetView};

pub type Tab<'a, Message, Backend> = crate::model::Tab<'a, Message, Renderer<Backend>>;

impl<'a, Message, Renderer> WidgetView<Message, Renderer> for crate::model::Tab<'a, Message, Renderer>
where
    Renderer: TabRenderer,
    Message: Clone,
{
    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let padding = f32::from(self.padding);
        let limits = limits
            .min_width(self.min_width)
            .min_height(self.min_height)
            .width(self.width)
            .height(self.height)
            .pad(padding);

        let mut content = self.content.layout(renderer, &limits);
        content.move_to(Point::new(padding, padding));

        let size = limits.resolve(content.size()).pad(padding);

        layout::Node::with_children(size, vec![content])
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        renderer.draw(
            defaults,
            layout.bounds(),
            cursor_position,
            &self.style,
            &self.content,
            layout.children().next().unwrap(),
        )
    }
}

/// The renderer of a [`Tab`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`Tab`] in your user interface.
///
/// [`Tab`]: struct.Tab.html
/// [renderer]: ../../renderer/index.html
pub trait TabRenderer: iced_native::Renderer + Sized {
    /// The default padding of a [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    const DEFAULT_PADDING: u16;

    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        style: &Self::Style,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output;
}


impl<B> TabRenderer for Renderer<B>
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
