use iced_native::{
    layout, mouse, Clipboard, Element, Event, Hasher, Layout, Length, Point, Rectangle, Widget,
};
use std::hash::Hash;

#[allow(missing_debug_implementations)]
pub struct Tab<'a, Message, Renderer: self::Renderer> {
    content: Element<'a, Message, Renderer>,
    on_press: Option<Message>,
    width: Length,
    height: Length,
    min_width: u32,
    min_height: u32,
    padding: u16,
    style: Renderer::Style,
}

impl<'a, Message, Renderer> Tab<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    /// Creates a new [`Tab`] with the given
    /// content.
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn new<E>(content: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        Tab {
            content: content.into(),
            on_press: None,
            width: Length::Shrink,
            height: Length::Shrink,
            min_width: 0,
            min_height: 0,
            padding: Renderer::DEFAULT_PADDING,
            style: Renderer::Style::default(),
        }
    }

    /// Sets the width of the [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the minimum width of the [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn min_width(mut self, min_width: u32) -> Self {
        self.min_width = min_width;
        self
    }

    /// Sets the minimum height of the [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn min_height(mut self, min_height: u32) -> Self {
        self.min_height = min_height;
        self
    }

    /// Sets the padding of the [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the message that will be produced when the [`Tab`] is pressed.
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    /// Sets the style of the [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Tab<'a, Message, Renderer>
where
    Renderer: self::Renderer,
    Message: Clone,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

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

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>,
    ) {
        // Allows the close tab Tab to be pressed
        if let Some(child_layout) = layout.children().next() {
            self.content.on_event(
                event.clone(),
                child_layout,
                cursor_position,
                messages,
                renderer,
                clipboard,
            );
            // Do not focus if close Tab clicked
            if let Some(close_layout) = child_layout.children().last() {
                match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                        if let Some(on_press) = self.on_press.clone() {
                            if layout.bounds().contains(cursor_position)
                                && !close_layout.bounds().contains(cursor_position)
                            {
                                messages.push(on_press);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
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

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.content.hash_layout(state);
    }
}

/// The renderer of a [`Tab`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`Tab`] in your user interface.
///
/// [`Tab`]: struct.Tab.html
/// [renderer]: ../../renderer/index.html
pub trait Renderer: iced_native::Renderer + Sized {
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

impl<'a, Message, Renderer> From<Tab<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a + Clone,
{
    fn from(tab: Tab<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(tab)
    }
}
