//
// The following widget was adapted from iced's PickList:
// https://github.com/iced-rs/iced/blob/0.14.0/widget/src/pick_list.rs
//

use std::{borrow::Borrow, f32};

use iced;
use iced::widget::overlay::menu::{self, Menu};
use iced_core::alignment;
use iced_core::keyboard;
use iced_core::layout;
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::text::paragraph;
use iced_core::text::{self, Text};
use iced_core::touch;
use iced_core::widget::tree::{self, Tree};
use iced_core::window;
use iced_core::{
    Background, Border, Clipboard, Color, Element, Event, Layout, Length, Padding, Pixels, Point,
    Rectangle, Shell, Size, Theme, Vector, Widget,
};

use crate::widgets::style;

pub struct DropdownMenu<'a, T, L, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    on_select: Box<dyn Fn(T) -> Message + 'a>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    options: L,
    menu_title: String,
    width: Length,
    padding: Padding,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    text_shaping: text::Shaping,
    font: Option<Renderer::Font>,
    class: <Theme as Catalog>::Class<'a>,
    menu_class: <Theme as menu::Catalog>::Class<'a>,
    last_status: Option<Status>,
    menu_height: Length,
}

impl<'a, T, L, Message, Theme, Renderer> DropdownMenu<'a, T, L, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a,
    Message: Clone,
    Theme: Catalog,
    Renderer: text::Renderer,
{
    /// Creates a new [`DropdownMenu`] with the given list of options, the current
    /// selected value, and the message to produce when an option is selected.
    pub fn new(options: L, menu_title: String, on_select: impl Fn(T) -> Message + 'a) -> Self {
        Self {
            on_select: Box::new(on_select),
            on_open: None,
            on_close: None,
            options,
            menu_title,
            width: Length::Shrink,
            padding: iced::widget::button::DEFAULT_PADDING,
            text_size: None,
            text_line_height: text::LineHeight::default(),
            text_shaping: text::Shaping::default(),
            font: None,
            class: <Theme as Catalog>::default(),
            menu_class: <Theme as Catalog>::default_menu(),
            last_status: None,
            menu_height: Length::Shrink,
        }
    }

    /// Sets the menu_title of the [`DropdownMenu`].
    pub fn menu_title(mut self, menu_title: impl Into<String>) -> Self {
        self.menu_title = menu_title.into();
        self
    }

    /// Sets the width of the [`DropdownMenu`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Menu`].
    pub fn menu_height(mut self, menu_height: impl Into<Length>) -> Self {
        self.menu_height = menu_height.into();
        self
    }

    /// Sets the [`Padding`] of the [`DropdownMenu`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the text size of the [`DropdownMenu`].
    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    /// Sets the text [`text::LineHeight`] of the [`DropdownMenu`].
    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`DropdownMenu`].
    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    /// Sets the font of the [`DropdownMenu`].
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the message that will be produced when the [`DropdownMenu`] is opened.
    pub fn on_open(mut self, on_open: Message) -> Self {
        self.on_open = Some(on_open);
        self
    }

    /// Sets the message that will be produced when the [`DropdownMenu`] is closed.
    pub fn on_close(mut self, on_close: Message) -> Self {
        self.on_close = Some(on_close);
        self
    }

    /// Sets the style of the [`DropdownMenu`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style of the [`Menu`].
    #[must_use]
    pub fn menu_style(mut self, style: impl Fn(&Theme) -> menu::Style + 'a) -> Self
    where
        <Theme as menu::Catalog>::Class<'a>: From<menu::StyleFn<'a, Theme>>,
    {
        self.menu_class = (Box::new(style) as menu::StyleFn<'a, Theme>).into();
        self
    }
}

impl<'a, T, L, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for DropdownMenu<'a, T, L, Message, Theme, Renderer>
where
    T: Clone + ToString + PartialEq + 'a,
    L: Borrow<[T]>,
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Renderer::Paragraph>::new())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());
        let options = self.options.borrow();

        state.options.resize_with(options.len(), Default::default);

        let option_text = Text {
            content: "",
            bounds: Size::new(
                f32::INFINITY,
                self.text_line_height.to_absolute(text_size).into(),
            ),
            size: text_size,
            line_height: self.text_line_height,
            font,
            align_x: text::Alignment::Default,
            align_y: alignment::Vertical::Center,
            shaping: self.text_shaping,
            wrapping: text::Wrapping::default(),
        };

        for (option, paragraph) in options.iter().zip(state.options.iter_mut()) {
            let label = option.to_string();

            let _ = paragraph.update(Text {
                content: &label,
                ..option_text
            });
        }

        if !self.menu_title.is_empty() {
            let _ = state.menu_title.update(Text {
                content: &self.menu_title,
                ..option_text
            });
        }

        let max_width = match self.width {
            Length::Shrink => state.menu_title.min_width(),
            _ => 0.0,
        };

        let size = {
            let intrinsic = Size::new(
                max_width.ceil(),
                f32::from(self.text_line_height.to_absolute(text_size)),
            );

            limits
                .width(self.width)
                .shrink(self.padding)
                .resolve(self.width, Length::Shrink, intrinsic)
                .expand(self.padding)
        };

        layout::Node::new(size)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if state.is_open {
                    // Event wasn't processed by overlay, so cursor was clicked either outside its
                    // bounds or on the drop-down, either way we close the overlay.
                    state.is_open = false;

                    if let Some(on_close) = &self.on_close {
                        shell.publish(on_close.clone());
                    }

                    shell.capture_event();
                } else if cursor.is_over(layout.bounds()) {
                    state.is_open = true;
                    state.hovered_option = None;

                    if let Some(on_open) = &self.on_open {
                        shell.publish(on_open.clone());
                    }

                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.keyboard_modifiers = *modifiers;
            }
            _ => {}
        };

        let status = {
            let is_hovered = cursor.is_over(layout.bounds());

            if state.is_open {
                Status::Opened { is_hovered }
            } else if is_hovered {
                Status::Hovered
            } else {
                Status::Active
            }
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(status);
        } else if self
            .last_status
            .is_some_and(|last_status| last_status != status)
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();
        let is_mouse_over = cursor.is_over(bounds);

        if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let bounds = layout.bounds();

        let style = Catalog::style(
            theme,
            &self.class,
            self.last_status.unwrap_or(Status::Active),
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                ..renderer::Quad::default()
            },
            style.background,
        );

        if !self.menu_title.is_empty() {
            let text_size = self.text_size.unwrap_or_else(|| renderer.default_size());

            renderer.fill_text(
                Text {
                    content: self.menu_title.clone(),
                    size: text_size,
                    line_height: self.text_line_height,
                    font,
                    bounds: Size::new(
                        bounds.width - self.padding.x(),
                        f32::from(self.text_line_height.to_absolute(text_size)),
                    ),
                    align_x: text::Alignment::Default,
                    align_y: alignment::Vertical::Center,
                    shaping: self.text_shaping,
                    wrapping: text::Wrapping::default(),
                },
                Point::new(bounds.x + self.padding.left, bounds.center_y()),
                style.menu_title_color,
                *viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
        let font = self.font.unwrap_or_else(|| renderer.default_font());

        if state.is_open {
            let max_width = match self.width {
                Length::Shrink => state.options.iter().fold(0.0, |width, paragraph| {
                    f32::max(width, paragraph.min_width())
                }),
                _ => 0.0,
            };

            let bounds = layout.bounds();

            let on_select = &self.on_select;

            let mut menu = Menu::new(
                &mut state.menu,
                self.options.borrow(),
                &mut state.hovered_option,
                |option| {
                    state.is_open = false;

                    (on_select)(option)
                },
                None,
                &self.menu_class,
            )
            .width(f32::max(
                bounds.width,
                (max_width + self.padding.left + self.padding.right).ceil(),
            ))
            .padding(self.padding)
            .font(font)
            .text_shaping(self.text_shaping);

            if let Some(text_size) = self.text_size {
                menu = menu.text_size(text_size);
            }

            Some(menu.overlay(
                layout.position() + translation,
                *viewport,
                bounds.height,
                self.menu_height,
            ))
        } else {
            None
        }
    }
}

impl<'a, T, L, Message, Theme, Renderer> From<DropdownMenu<'a, T, L, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    T: Clone + ToString + PartialEq + 'a,
    L: Borrow<[T]> + 'a,
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(dropdown_menu: DropdownMenu<'a, T, L, Message, Theme, Renderer>) -> Self {
        Self::new(dropdown_menu)
    }
}

#[derive(Debug)]
struct State<P: text::Paragraph> {
    menu: menu::State,
    keyboard_modifiers: keyboard::Modifiers,
    is_open: bool,
    hovered_option: Option<usize>,
    options: Vec<paragraph::Plain<P>>,
    menu_title: paragraph::Plain<P>,
}

impl<P: text::Paragraph> State<P> {
    /// Creates a new [`State`] for a [`DropdownMenu`].
    fn new() -> Self {
        Self {
            menu: menu::State::default(),
            keyboard_modifiers: keyboard::Modifiers::default(),
            is_open: bool::default(),
            hovered_option: Option::default(),
            options: Vec::new(),
            menu_title: paragraph::Plain::default(),
        }
    }
}

impl<P: text::Paragraph> Default for State<P> {
    fn default() -> Self {
        Self::new()
    }
}

/// The possible status of a [`DropdownMenu`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`DropdownMenu`] can be interacted with.
    Active,
    /// The [`DropdownMenu`] is being hovered.
    Hovered,
    /// The [`DropdownMenu`] is open.
    Opened {
        /// Whether the [`DropdownMenu`] is hovered, while open.
        is_hovered: bool,
    },
}

/// The appearance of a pick list.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The text [`Color`] of the pick list.
    pub text_color: Color,
    /// The menu_title [`Color`] of the pick list.
    pub menu_title_color: Color,
    /// The [`Background`] of the pick list.
    pub background: Background,
    /// The [`Border`] of the pick list.
    pub border: Border,
}

/// The theme catalog of a [`DropdownMenu`].
pub trait Catalog: menu::Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> <Self as Catalog>::Class<'a>;

    /// The default class for the menu of the [`DropdownMenu`].
    fn default_menu<'a>() -> <Self as menu::Catalog>::Class<'a> {
        <Self as menu::Catalog>::default()
    }

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`DropdownMenu`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(default)
    }

    fn style(&self, class: &StyleFn<'_, Self>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of the field of a [`DropdownMenu`].
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let base = Style {
        background: palette.background.weakest.color.into(),
        text_color: palette.background.weakest.text,
        border: iced::border::rounded(style::Style::default().border_width),
        menu_title_color: palette.background.weakest.text,
    };

    match status {
        Status::Active => base,
        Status::Opened { is_hovered: _ } | Status::Hovered => Style {
            background: iced::Background::Color(palette.background.weaker.color),
            ..base
        },
    }
}

/// The default style of the field of a [`DropdownMenu`].
pub fn from_style(theme: &Theme, status: Status, style: &style::Style) -> Style {
    let base: Style = default(theme, status);

    Style {
        border: iced::border::rounded(style.border_width),
        ..base
    }
}
