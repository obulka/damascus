//! Combo boxes display a dropdown list of searchable and selectable options.
//!
//! # Example
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! #
//! use iced::widget::dropdown;
//!
//! struct State {
//!    fruits: dropdown::State<Fruit>,
//!    favorite: Option<Fruit>,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Fruit {
//!     Apple,
//!     Orange,
//!     Strawberry,
//!     Tomato,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     FruitSelected(Fruit),
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     dropdown(
//!         &state.fruits,
//!         "Select your favorite fruit...",
//!         state.favorite.as_ref(),
//!         Message::FruitSelected
//!     )
//!     .into()
//! }
//!
//! fn update(state: &mut State, message: Message) {
//!     match message {
//!         Message::FruitSelected(fruit) => {
//!             state.favorite = Some(fruit);
//!         }
//!     }
//! }
//!
//! impl std::fmt::Display for Fruit {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         f.write_str(match self {
//!             Self::Apple => "Apple",
//!             Self::Orange => "Orange",
//!             Self::Strawberry => "Strawberry",
//!             Self::Tomato => "Tomato",
//!         })
//!     }
//! }
//! ```

use std::cell::RefCell;
use std::fmt::Display;

use iced;
use iced::overlay::menu;
use iced::time::Instant;
use iced::widget::button::{self, Button};
use iced_core::keyboard;
use iced_core::keyboard::key;
use iced_core::layout::{self, Layout};
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::text;
use iced_core::text::LineHeight;
use iced_core::widget::{self, Widget};
use iced_core::{
    Clipboard, Element, Event, Length, Padding, Pixels, Rectangle, Shell, Size, Theme, Vector,
};

use damascus::{Enum, Enumerator};

/// A widget for searching and selecting a single value from a list of options.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// #
/// use iced::widget::dropdown;
///
/// struct State {
///    fruits: dropdown::State<Fruit>,
///    favorite: Option<Fruit>,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Fruit {
///     Apple,
///     Orange,
///     Strawberry,
///     Tomato,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     FruitSelected(Fruit),
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     dropdown(
///         &state.fruits,
///         "Select your favorite fruit...",
///         state.favorite.as_ref(),
///         Message::FruitSelected
///     )
///     .into()
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::FruitSelected(fruit) => {
///             state.favorite = Some(fruit);
///         }
///     }
/// }
///
/// impl std::fmt::Display for Fruit {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.write_str(match self {
///             Self::Apple => "Apple",
///             Self::Orange => "Orange",
///             Self::Strawberry => "Strawberry",
///             Self::Tomato => "Tomato",
///         })
///     }
/// }
/// ```
pub struct Dropdown<'a, T, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    T: Enumerator,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    open: bool,
    button: Button<'a, Message, Theme, Renderer>,
    width: Length,
    state: Enum,
    font: Option<Renderer::Font>,
    on_selected: Box<dyn Fn(T) -> Message>,
    on_option_hovered: Option<Box<dyn Fn(T) -> Message>>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    on_input: Option<Box<dyn Fn(String) -> Message>>,
    padding: Padding,
    size: Option<f32>,
    text_shaping: text::Shaping,
    menu_class: <Theme as menu::Catalog>::Class<'a>,
    menu_height: Length,
}

impl<'a, T, Message, Theme, Renderer> Dropdown<'a, T, Message, Theme, Renderer>
where
    T: Enumerator,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    /// Creates a new [`Dropdown`] with the given list of options, a placeholder,
    /// the current selected value, and the message to produce when an option is
    /// selected.
    pub fn new(name: &str, enumerator: T, on_selected: impl Fn(T) -> Message + 'static) -> Self {
        let enumerator: Enum = Enum::from(enumerator);
        Self {
            open: false,
            button: Button::new(iced::widget::text(name)),
            width: Length::Fill,
            state: enumerator,
            font: None,
            on_selected: Box::new(on_selected),
            on_option_hovered: None,
            on_input: None,
            on_open: None,
            on_close: None,
            padding: button::DEFAULT_PADDING,
            size: None,
            text_shaping: text::Shaping::default(),
            menu_class: <Theme as Catalog>::default_menu(),
            menu_height: Length::Shrink,
        }
    }

    /// Sets the message that should be produced when some text is typed into
    /// the [`TextInput`] of the [`Dropdown`].
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'static) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Sets the message that will be produced when an option of the
    /// [`Dropdown`] is hovered using the arrow keys.
    pub fn on_option_hovered(mut self, on_option_hovered: impl Fn(T) -> Message + 'static) -> Self {
        self.on_option_hovered = Some(Box::new(on_option_hovered));
        self
    }

    /// Sets the message that will be produced when the  [`Dropdown`] is
    /// opened.
    pub fn on_open(mut self, message: Message) -> Self {
        self.on_open = Some(message);
        self
    }

    /// Sets the message that will be produced when the outside area
    /// of the [`Dropdown`] is pressed.
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// Sets the [`Padding`] of the [`Dropdown`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the [`Renderer::Font`] of the [`Dropdown`].
    ///
    /// [`Renderer::Font`]: text::Renderer
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the text size of the [`Dropdown`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into().0);

        self
    }

    /// Sets the width of the [`Dropdown`].
    pub fn width(self, width: impl Into<Length>) -> Self {
        Self {
            width: width.into(),
            ..self
        }
    }

    /// Sets the height of the menu of the [`Dropdown`].
    pub fn menu_height(mut self, menu_height: impl Into<Length>) -> Self {
        self.menu_height = menu_height.into();
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`Dropdown`].
    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// Sets the style of the menu of the [`Dropdown`].
    #[must_use]
    pub fn menu_style(mut self, style: impl Fn(&Theme) -> menu::Style + 'a) -> Self
    where
        <Theme as menu::Catalog>::Class<'a>: From<menu::StyleFn<'a, Theme>>,
    {
        self.menu_class = (Box::new(style) as menu::StyleFn<'a, Theme>).into();
        self
    }
}

struct Menu<T> {
    menu: menu::State,
    hovered_option: Option<usize>,
    new_selection: Option<T>,
}

#[derive(Debug, Clone)]
enum TextInputEvent {
    TextChanged(String),
}

impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Dropdown<'_, T, Message, Theme, Renderer>
where
    T: Enumerator + 'static,
    Message: Clone,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    fn size(&self) -> Size<Length> {
        self.button.size()
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.button.layout(tree, renderer, limits)
    }

    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<Menu<T>>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(Menu::<T> {
            menu: menu::State::new(),
            hovered_option: Some(0),
            new_selection: None,
        })
    }

    fn children(&self) -> Vec<widget::Tree> {
        vec![widget::Tree::new(&self.button as &dyn Widget<_, _, _>)]
    }

    fn diff(&self, _tree: &mut widget::Tree) {
        // do nothing so the children don't get cleared
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let menu = tree.state.downcast_mut::<Menu<T>>();

        let started_open: bool = self.open;

        let mut published_message_to_shell = false;

        if self.open {
            if !started_open && let Some(on_option_hovered) = &mut self.on_option_hovered {
                let hovered_option = menu.hovered_option.unwrap_or(0);

                if let Some(option) = self.state.variants.get(hovered_option) {
                    shell.publish(on_option_hovered(option.clone()));
                    published_message_to_shell = true;
                }
            }

            // if let Event::Keyboard(keyboard::Event::KeyPressed {
            //     key: keyboard::Key::Named(named_key),
            //     modifiers,
            //     ..
            // }) = event
            // {
            //     let shift_modifier = modifiers.shift();
            //     match (named_key, shift_modifier) {
            //         (key::Named::Enter, _) => {
            //             if let Some(index) = &menu.hovered_option
            //                 && let Some(option) =
            //                     state.filtered_options.options.get(*index)
            //             {
            //                 menu.new_selection = Some(option.clone());
            //             }

            //             shell.capture_event();
            //             shell.request_redraw();
            //         }
            //         (key::Named::ArrowUp, _) | (key::Named::Tab, true) => {
            //             if let Some(index) = &mut menu.hovered_option {
            //                 if *index == 0 {
            //                     *index = state
            //                         .filtered_options
            //                         .options
            //                         .len()
            //                         .saturating_sub(1);
            //                 } else {
            //                     *index = index.saturating_sub(1);
            //                 }
            //             } else {
            //                 menu.hovered_option = Some(0);
            //             }

            //             if let Some(on_option_hovered) =
            //                 &mut self.on_option_hovered
            //                 && let Some(option) =
            //                     menu.hovered_option.and_then(|index| {
            //                         state
            //                             .filtered_options
            //                             .options
            //                             .get(index)
            //                     })
            //             {
            //                 // Notify the selection
            //                 shell.publish((on_option_hovered)(
            //                     option.clone(),
            //                 ));
            //                 published_message_to_shell = true;
            //             }

            //             shell.capture_event();
            //             shell.request_redraw();
            //         }
            //         (key::Named::ArrowDown, _)
            //         | (key::Named::Tab, false)
            //             if !modifiers.shift() =>
            //         {
            //             if let Some(index) = &mut menu.hovered_option {
            //                 if *index
            //                     >= state
            //                         .filtered_options
            //                         .options
            //                         .len()
            //                         .saturating_sub(1)
            //                 {
            //                     *index = 0;
            //                 } else {
            //                     *index = index.saturating_add(1).min(
            //                         state
            //                             .filtered_options
            //                             .options
            //                             .len()
            //                             .saturating_sub(1),
            //                     );
            //                 }
            //             } else {
            //                 menu.hovered_option = Some(0);
            //             }

            //             if let Some(on_option_hovered) =
            //                 &mut self.on_option_hovered
            //                 && let Some(option) =
            //                     menu.hovered_option.and_then(|index| {
            //                         self.state
            //                             .variants
            //                             .get(index)
            //                     })
            //             {
            //                 // Notify the selection
            //                 shell.publish((on_option_hovered)(
            //                     option.clone(),
            //                 ));
            //                 published_message_to_shell = true;
            //             }

            //             shell.capture_event();
            //             shell.request_redraw();
            //         }
            //         _ => {}
            //     }
            // });
        }

        // If the overlay menu has selected something
        self.state.with_inner_mut(|state| {
            if let Some(selection) = menu.new_selection.take() {
                // Clear the value and reset the options and menu
                menu.menu = menu::State::default();

                // Notify the selection
                shell.publish((self.on_selected)(selection));
                published_message_to_shell = true;

                // Unfocus the input
                let mut local_messages = Vec::new();
                let mut local_shell = Shell::new(&mut local_messages);

                shell.request_input_method(local_shell.input_method());
            }
        });

        if started_open != self.open {
            // Focus changed, invalidate widget tree to force a fresh `view`
            shell.invalidate_widgets();

            if !published_message_to_shell {
                if self.open {
                    if let Some(on_open) = self.on_open.take() {
                        shell.publish(on_open);
                    }
                } else if let Some(on_close) = self.on_close.take() {
                    shell.publish(on_close);
                }
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.button
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.button
            .draw(&tree.children[0], renderer, theme, layout, cursor, viewport);
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        if self.open {
            let Menu {
                menu,
                hovered_option,
                ..
            } = tree.state.downcast_mut::<Menu<T>>();

            if self.state.variants.is_empty() {
                None
            } else {
                let bounds = layout.bounds();

                let mut menu = menu::Menu::new(
                    menu,
                    &self.state.variants,
                    hovered_option,
                    |selection| (self.on_selected)(selection),
                    self.on_option_hovered.as_deref(),
                    &self.menu_class,
                )
                .width(bounds.width)
                .padding(self.padding)
                .text_shaping(self.text_shaping);

                if let Some(font) = self.font {
                    menu = menu.font(font);
                }

                if let Some(size) = self.size {
                    menu = menu.text_size(size);
                }

                Some(menu.overlay(
                    layout.position() + translation,
                    *viewport,
                    bounds.height,
                    self.menu_height,
                ))
            }
        } else {
            None
        }
    }
}

impl<'a, T, Message, Theme, Renderer> From<Dropdown<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: Enumerator + 'static,
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(dropdown: Dropdown<'a, T, Message, Theme, Renderer>) -> Self {
        Self::new(dropdown)
    }
}

/// The theme catalog of a [`Dropdown`].
pub trait Catalog: iced::widget::text::Catalog + button::Catalog + menu::Catalog {
    /// The default class for the text input of the [`Dropdown`].
    fn default_input<'a>() -> <Self as button::Catalog>::Class<'a> {
        <Self as button::Catalog>::default()
    }

    /// The default class for the menu of the [`Dropdown`].
    fn default_menu<'a>() -> <Self as menu::Catalog>::Class<'a> {
        <Self as menu::Catalog>::default()
    }
}

impl Catalog for Theme {}

fn search<'a, T, A>(
    options: impl IntoIterator<Item = T> + 'a,
    option_matchers: impl IntoIterator<Item = &'a A> + 'a,
    query: &'a str,
) -> impl Iterator<Item = T> + 'a
where
    A: AsRef<str> + 'a,
{
    let query: Vec<String> = query
        .to_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .map(String::from)
        .collect();

    options
        .into_iter()
        .zip(option_matchers)
        // Make sure each part of the query is found in the option
        .filter_map(move |(option, matcher)| {
            if query.iter().all(|part| matcher.as_ref().contains(part)) {
                Some(option)
            } else {
                None
            }
        })
}

fn build_matchers<'a, T>(options: impl IntoIterator<Item = T> + 'a) -> Vec<String>
where
    T: Display + 'a,
{
    options.into_iter().map(build_matcher).collect()
}

fn build_matcher<T>(option: T) -> String
where
    T: Display,
{
    let mut matcher = option.to_string();
    matcher.retain(|c| c.is_ascii_alphanumeric());
    matcher.to_lowercase()
}
