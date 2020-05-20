//! Distribute content vertically.


use iced_native::{
    column,
    Column,
    Element,
    Event,
    Hasher,
    Layout,
    layout,
    Point,
    Clipboard,
    Widget,
};
use iced_core::{
    Length,
};


/// A container that distributes its contents vertically.
///
/// A [`Column`] will try to fill the horizontal space of its container.
///
/// [`Column`]: struct.Column.html
#[allow(missing_debug_implementations)]
pub struct DropDownMenu<'a, Message, Renderer: self::column::Renderer> {
    pub column: Column<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> DropDownMenu<'a, Message, Renderer>
where
    Renderer: self::column::Renderer,
{
    /// Creates an empty [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn new(column: Column<'a, Message, Renderer>) -> Self {
        DropDownMenu {
            column: column,
        }
    }

    /// Adds an element to the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn push<E>(mut self, child: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.column = self.column.push(child);
        self
    }
}


impl<'a, Message, Renderer> Widget<Message, Renderer>
    for DropDownMenu<'a, Message, Renderer>
where
    Renderer: self::column::Renderer,
{
    fn width(&self) -> Length {
        <Column<'a, Message, Renderer> as Widget<Message, Renderer>>::width(&self.column)
    }

    fn height(&self) -> Length {
        <Column<'a, Message, Renderer> as Widget<Message, Renderer>>::height(&self.column)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.column.layout(renderer, limits)
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
        self.column.on_event(
            event,
            layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        self.column.draw(renderer, defaults, layout, cursor_position)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.column.hash_layout(state);
    }
}


impl<'a, Message, Renderer> From<DropDownMenu<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::column::Renderer,
    Message: 'a,
{
    fn from(
        drop_down_menu: DropDownMenu<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(drop_down_menu.column)
    }
}
