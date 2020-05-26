//! Distribute content vertically.


use iced_native::{
    column,
    Column,
    Element,
    Event,
    Hasher,
    Layout,
    layout,
    mouse,
    Point,
    Clipboard,
    Widget,
};
use iced_core::{
    Length,
};


/// The local state of a [`DropDownMenu`].
///
/// [`DropDownMenu`]: struct.DropDownMenu.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_pressed: bool,
    is_open: bool,
}

impl State {
    /// Creates a new [`State`].
    ///
    /// [`State`]: struct.State.html
    pub fn new() -> State {
        State::default()
    }
}


/// A container that distributes its contents vertically.
///
/// A [`Column`] will try to fill the horizontal space of its container.
///
/// [`Column`]: struct.Column.html
#[allow(missing_debug_implementations)]
pub struct DropDownMenu<'a, Message, Renderer> {
    state: &'a mut State,
    menu: Column<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> DropDownMenu<'a, Message, Renderer>
where
    Renderer: self::column::Renderer,
{
    /// Creates an empty [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn new(state: &'a mut State, menu: Column<'a, Message, Renderer>) -> Self {
        DropDownMenu {
            state: state,
            menu: menu,
        }
    }

    /// Adds an element to the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn push<E>(mut self, child: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.menu = self.menu.push(child);
        self
    }
}


impl<'a, Message, Renderer> Widget<Message, Renderer>
    for DropDownMenu<'a, Message, Renderer>
where
    Renderer: self::column::Renderer,
{
    fn width(&self) -> Length {
        <Column<'a, Message, Renderer> as Widget<Message, Renderer>>::width(&self.menu)
    }

    fn height(&self) -> Length {
        <Column<'a, Message, Renderer> as Widget<Message, Renderer>>::height(&self.menu)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.menu.layout(renderer, limits)
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
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                let bounds = layout.bounds();
                self.state.is_pressed = bounds.contains(cursor_position);
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)) => {
                let bounds = layout.bounds();

                let is_clicked = self.state.is_pressed
                    && bounds.contains(cursor_position);

                self.state.is_pressed = false;

                if self.state.is_open {
                    self.state.is_open = false;
                }
                else if is_clicked {
                    self.state.is_open = true;
                }
                println!("Open? {:?}", self.state.is_open);
            }
            _ => {}
        }
        self.menu.on_event(
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
        self.menu.draw(renderer, defaults, layout, cursor_position)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.menu.hash_layout(state);
    }
}


// /// The renderer of a [`Column`].
// ///
// /// Your [renderer] will need to implement this trait before being
// /// able to use a [`Column`] in your user interface.
// ///
// /// [`Column`]: struct.Column.html
// /// [renderer]: ../../renderer/index.html
// pub trait Renderer: iced_native::Renderer + Sized {
//     /// Draws a [`Column`].
//     ///
//     /// It receives:
//     /// - the children of the [`Column`]
//     /// - the [`Layout`] of the [`Column`] and its children
//     /// - the cursor position
//     ///
//     /// [`Column`]: struct.Column.html
//     /// [`Layout`]: ../layout/struct.Layout.html
//     fn draw<Message>(
//         &mut self,
//         defaults: &Self::Defaults,
//         menu: &Column<'_, Message, Self>,
//         layout: Layout<'_>,
//         cursor_position: Point,
//     ) -> Self::Output;
// }



impl<'a, Message, Renderer> From<DropDownMenu<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::column::Renderer,
    Message: 'a,
{
    fn from(
        drop_down_menu: DropDownMenu<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(drop_down_menu.menu)
    }
}
