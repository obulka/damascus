//
// The following widget was adapted from iced's PickList:
// https://github.com/iced-rs/iced/blob/0.14.0/widget/src/pick_list.rs
//

use std::borrow::Borrow;

use iced;
use iced_core;

use super::style::Style as DefaultStyle;

pub struct DropdownMenu<'a, T, L, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a,
    Theme: Catalog,
    Renderer: iced_core::text::Renderer,
{
    on_select: Box<dyn Fn(T) -> Message + 'a>,
    on_open: Option<Message>,
    on_close: Option<Message>,
    options: L,
    menu_title: String,
    icon: Option<iced_core::Svg>,
    width: iced::Length,
    padding: iced::Padding,
    spacing: f32,
    icon_size: u32,
    text_size: iced::Pixels,
    text_line_height: iced_core::text::LineHeight,
    text_shaping: iced_core::text::Shaping,
    font: Option<Renderer::Font>,
    class: <Theme as Catalog>::Class<'a>,
    menu_class: <Theme as iced::widget::overlay::menu::Catalog>::Class<'a>,
    last_status: Option<Status>,
    menu_height: iced::Length,
}

impl<'a, T, L, Message, Theme, Renderer> DropdownMenu<'a, T, L, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]> + 'a,
    Message: Clone,
    Theme: Catalog,
    Renderer: iced_core::text::Renderer,
{
    /// Creates a new [`DropdownMenu`] with the given list of options, the current
    /// selected value, and the message to produce when an option is selected.
    pub fn new(
        options: L,
        menu_title: String,
        icon: Option<iced_core::Svg>,
        on_select: impl Fn(T) -> Message + 'a,
    ) -> Self {
        let default_style = DefaultStyle::default();
        Self {
            on_select: Box::new(on_select),
            on_open: None,
            on_close: None,
            options,
            menu_title,
            icon,
            width: iced::Length::Shrink,
            padding: default_style.padding.into(),
            spacing: default_style.padding,
            icon_size: default_style.icon_size,
            text_size: default_style.text_size.into(),
            text_line_height: iced_core::text::LineHeight::default(),
            text_shaping: iced_core::text::Shaping::default(),
            font: None,
            class: <Theme as Catalog>::default(),
            menu_class: <Theme as Catalog>::default_menu(),
            last_status: None,
            menu_height: iced::Length::Shrink,
        }
    }

    /// Sets the menu_title of the [`DropdownMenu`].
    pub fn menu_title(mut self, menu_title: impl Into<String>) -> Self {
        self.menu_title = menu_title.into();
        self
    }

    /// Sets the icon of the [`DropdownMenu`].
    pub fn icon(mut self, icon: iced_core::Svg) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the icon_size of the [`DropdownMenu`].
    pub fn icon_size(mut self, icon_size: u32) -> Self {
        self.icon_size = icon_size;
        self
    }

    /// Sets the spacing of the [`DropdownMenu`].
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the width of the [`DropdownMenu`].
    pub fn width(mut self, width: impl Into<iced::Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Menu`].
    pub fn menu_height(mut self, menu_height: impl Into<iced::Length>) -> Self {
        self.menu_height = menu_height.into();
        self
    }

    /// Sets the [`Padding`] of the [`DropdownMenu`].
    pub fn padding<P: Into<iced::Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the text size of the [`DropdownMenu`].
    pub fn text_size(mut self, size: impl Into<iced::Pixels>) -> Self {
        self.text_size = size.into();
        self
    }

    /// Sets the text [`iced_core::text::LineHeight`] of the [`DropdownMenu`].
    pub fn text_line_height(mut self, line_height: impl Into<iced_core::text::LineHeight>) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    /// Sets the [`iced_core::text::Shaping`] strategy of the [`DropdownMenu`].
    pub fn text_shaping(mut self, shaping: iced_core::text::Shaping) -> Self {
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
    pub fn menu_style(
        mut self,
        style: impl Fn(&Theme) -> iced::widget::overlay::menu::Style + 'a,
    ) -> Self
    where
        <Theme as iced::widget::overlay::menu::Catalog>::Class<'a>:
            From<iced::widget::overlay::menu::StyleFn<'a, Theme>>,
    {
        self.menu_class =
            (Box::new(style) as iced::widget::overlay::menu::StyleFn<'a, Theme>).into();
        self
    }
}

impl<'a, T, L, Message, Theme, Renderer> iced_core::Widget<Message, Theme, Renderer>
    for DropdownMenu<'a, T, L, Message, Theme, Renderer>
where
    T: Clone + ToString + PartialEq + 'a,
    L: Borrow<[T]>,
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::text::Renderer + iced_core::svg::Renderer + 'a,
{
    fn tag(&self) -> iced_core::widget::tree::Tag {
        iced_core::widget::tree::Tag::of::<State<Renderer::Paragraph>>()
    }

    fn state(&self) -> iced_core::widget::tree::State {
        iced_core::widget::tree::State::new(State::<Renderer::Paragraph>::new())
    }

    fn size(&self) -> iced::Size<iced::Length> {
        iced::Size {
            width: self.width,
            height: iced::Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut iced_core::widget::tree::Tree,
        renderer: &Renderer,
        limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let options = self.options.borrow();

        state.options.resize_with(options.len(), Default::default);

        let option_text = iced_core::text::Text {
            content: "",
            bounds: iced::Size::new(
                f32::INFINITY,
                self.text_line_height.to_absolute(self.text_size).into(),
            ),
            size: self.text_size,
            line_height: self.text_line_height,
            font,
            align_x: iced_core::text::Alignment::Default,
            align_y: iced_core::alignment::Vertical::Center,
            shaping: self.text_shaping,
            wrapping: iced_core::text::Wrapping::default(),
        };

        for (option, paragraph) in options.iter().zip(state.options.iter_mut()) {
            let label = option.to_string();

            let _ = paragraph.update(iced_core::text::Text {
                content: &label,
                ..option_text
            });
        }

        if !self.menu_title.is_empty() {
            let _ = state.menu_title.update(iced_core::text::Text {
                content: &self.menu_title,
                ..option_text
            });
        }

        let max_width = match self.width {
            iced::Length::Shrink => {
                state.menu_title.min_width()
                    + if self.icon.is_some() {
                        self.icon_size as f32
                    } else {
                        0.
                    }
                    + if self.menu_title.is_empty() || self.icon.is_none() {
                        0.
                    } else {
                        self.spacing
                    }
            }
            _ => 0.0,
        };

        let size = {
            let intrinsic = iced::Size::new(
                max_width.ceil(),
                f32::from(self.text_line_height.to_absolute(self.text_size))
                    .max(self.icon_size as f32),
            );

            limits
                .resolve(self.width, iced::Length::Shrink, intrinsic)
                .expand(self.padding)
        };

        iced_core::layout::Node::new(size)
    }

    fn update(
        &mut self,
        tree: &mut iced_core::widget::tree::Tree,
        event: &iced::Event,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        match event {
            iced::Event::Mouse(iced_core::mouse::Event::ButtonPressed(
                iced_core::mouse::Button::Left,
            ))
            | iced::Event::Touch(iced_core::touch::Event::FingerPressed { .. }) => {
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
            iced::Event::Keyboard(iced_core::keyboard::Event::ModifiersChanged(modifiers)) => {
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

        if let iced::Event::Window(iced_core::window::Event::RedrawRequested(_now)) = event {
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
        _tree: &iced_core::widget::tree::Tree,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        let bounds = layout.bounds();
        let is_mouse_over = cursor.is_over(bounds);

        if is_mouse_over {
            iced_core::mouse::Interaction::Pointer
        } else {
            iced_core::mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        _tree: &iced_core::widget::tree::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &iced_core::renderer::Style,
        layout: iced_core::Layout<'_>,
        _cursor: iced_core::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let style = Catalog::style(
            theme,
            &self.class,
            self.last_status.unwrap_or(Status::Active),
        );

        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let mut bounds: iced::Rectangle = layout.bounds();
        if self.menu_title.is_empty() && self.icon.is_some() {
            bounds.height = bounds.width;
        }

        renderer.fill_quad(
            iced_core::renderer::Quad {
                bounds,
                border: style.border,
                ..iced_core::renderer::Quad::default()
            },
            style.background,
        );

        if !self.menu_title.is_empty() {
            renderer.fill_text(
                iced_core::text::Text {
                    content: self.menu_title.clone(),
                    size: self.text_size,
                    line_height: self.text_line_height,
                    font,
                    bounds: iced::Size::new(
                        bounds.width - self.padding.x(),
                        f32::from(self.text_line_height.to_absolute(self.text_size)),
                    ),
                    align_x: iced_core::text::Alignment::Default,
                    align_y: iced_core::alignment::Vertical::Center,
                    shaping: self.text_shaping,
                    wrapping: iced_core::text::Wrapping::default(),
                },
                iced::Point::new(bounds.x + self.padding.left, bounds.center_y()),
                style.text_colour,
                *viewport,
            );
        }

        if let Some(ref icon) = self.icon {
            let icon_bounds = iced::Rectangle {
                x: bounds.x + bounds.width - self.icon_size as f32 - self.padding.right,
                y: bounds.center_y() - self.icon_size as f32 * 0.5,
                width: self.icon_size as f32,
                height: self.icon_size as f32,
            };
            renderer.draw_svg(icon.clone(), icon_bounds, icon_bounds);
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced_core::widget::tree::Tree,
        layout: iced_core::Layout<'_>,
        renderer: &Renderer,
        viewport: &iced::Rectangle,
        translation: iced::Vector,
    ) -> Option<iced_core::overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
        let font = self.font.unwrap_or_else(|| renderer.default_font());

        if state.is_open {
            let max_width = match self.width {
                iced::Length::Shrink => state.options.iter().fold(0.0, |width, paragraph| {
                    f32::max(width, paragraph.min_width())
                }),
                _ => 0.0,
            };

            let bounds = layout.bounds();

            let on_select = &self.on_select;

            let mut menu = iced::widget::overlay::menu::Menu::new(
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

            menu = menu.text_size(self.text_size);

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
    for iced::Element<'a, Message, Theme, Renderer>
where
    T: Clone + ToString + PartialEq + 'a,
    L: Borrow<[T]> + 'a,
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::text::Renderer + iced_core::svg::Renderer + 'a,
{
    fn from(dropdown_menu: DropdownMenu<'a, T, L, Message, Theme, Renderer>) -> Self {
        Self::new(dropdown_menu)
    }
}

#[derive(Debug)]
struct State<P: iced_core::text::Paragraph> {
    menu: iced::widget::overlay::menu::State,
    keyboard_modifiers: iced_core::keyboard::Modifiers,
    is_open: bool,
    hovered_option: Option<usize>,
    options: Vec<iced_core::text::paragraph::Plain<P>>,
    menu_title: iced_core::text::paragraph::Plain<P>,
}

impl<P: iced_core::text::Paragraph> State<P> {
    /// Creates a new [`State`] for a [`DropdownMenu`].
    fn new() -> Self {
        Self {
            menu: iced::widget::overlay::menu::State::default(),
            keyboard_modifiers: iced_core::keyboard::Modifiers::default(),
            is_open: bool::default(),
            hovered_option: Option::default(),
            options: Vec::new(),
            menu_title: iced_core::text::paragraph::Plain::default(),
        }
    }
}

impl<P: iced_core::text::Paragraph> Default for State<P> {
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
    /// The menu_title [`Color`] of the pick list.
    pub text_colour: iced::Color,
    /// The [`Background`] of the pick list.
    pub background: iced_core::Background,
    /// The [`Border`] of the pick list.
    pub border: iced::Border,
}

/// The theme catalog of a [`DropdownMenu`].
pub trait Catalog: iced::widget::overlay::menu::Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> <Self as Catalog>::Class<'a>;

    /// The default class for the menu of the [`DropdownMenu`].
    fn default_menu<'a>() -> <Self as iced::widget::overlay::menu::Catalog>::Class<'a> {
        <Self as iced::widget::overlay::menu::Catalog>::default()
    }

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`DropdownMenu`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for iced::Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(default)
    }

    fn style(&self, class: &StyleFn<'_, Self>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of the field of a [`DropdownMenu`].
pub fn default(theme: &iced::Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let base = Style {
        background: iced::Color::TRANSPARENT.into(),
        border: iced::border::rounded(2),
        text_colour: palette.background.weakest.text,
    };

    match status {
        Status::Active => base,
        Status::Opened { is_hovered: _ } | Status::Hovered => Style {
            background: iced::Background::Color(palette.background.weaker.color),
            ..base
        },
    }
}
