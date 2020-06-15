// Standard Imports
use std::hash::Hash;

// 3rd Party Imports
use iced_native::{
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point, Widget,
};

use crate::model::WidgetModel;
use crate::view::{renderer::tab::TabRenderer, WidgetView};
use crate::update::WidgetUpdate;

#[allow(missing_debug_implementations)]
pub struct Tab<'a, Message, Renderer: TabRenderer> {
    pub content: Element<'a, Message, Renderer>,
    pub on_press: Option<Message>,
    pub width: Length,
    pub height: Length,
    pub min_width: u32,
    pub min_height: u32,
    pub padding: u16,
    pub style: Renderer::Style,
}

impl<'a, Message, Renderer> WidgetModel<Message, Renderer> for Tab<'a, Message, Renderer>
where
    Renderer: TabRenderer,
    Message: Clone,
{}

impl<'a, Message, Renderer> Tab<'a, Message, Renderer>
where
    Renderer: TabRenderer,
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
    Renderer: TabRenderer,
    Message: Clone,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        WidgetView::layout(self, renderer, limits)
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
        WidgetUpdate::on_event(
            self,
            event,
            layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        WidgetView::draw(
            self,
            renderer,
            defaults,
            layout,
            cursor_position,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.content.hash_layout(state);
    }
}

impl<'a, Message, Renderer> From<Tab<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + TabRenderer,
    Message: 'a + Clone,
{
    fn from(tab: Tab<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(tab)
    }
}
