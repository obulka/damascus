//
// The following widget was adapted from iced's TextInput:
// https://github.com/iced-rs/iced/blob/0.14.0/widget/src/text_input.rs
//

// Copyright 2019 Héctor Ramón, Iced contributors

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Text inputs display fields that can be filled with text.
//!
//! # Example
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! #
//! use iced_core::widget::text_input;
//!
//! struct State {
//!    content: String,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     ContentChanged(String)
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     text_input("Type something here...", &state.content)
//!         .on_input(Message::ContentChanged)
//!         .into()
//! }
//!
//! fn update(state: &mut State, message: Message) {
//!     match message {
//!         Message::ContentChanged(content) => {
//!             state.content = content;
//!         }
//!     }
//! }
//! ```

use iced_core;
use iced_core::text::paragraph::Paragraph as _;

use super::style::Style as DefaultStyle;

pub mod cursor;
pub mod editor;
pub mod value;

use cursor::Cursor;
use editor::Editor;
use value::Value;

/// A field that can be filled with text.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// #
/// use iced_core::widget::text_input;
///
/// struct State {
///    content: String,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     ContentChanged(String)
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     text_input("Type something here...", &state.content)
///         .on_input(Message::ContentChanged)
///         .into()
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::ContentChanged(content) => {
///             state.content = content;
///         }
///     }
/// }
/// ```
pub struct NumberInput<'a, Message, Theme = iced_core::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog,
    Renderer: iced_core::text::Renderer,
{
    id: Option<iced_core::widget::Id>,
    value: Value,
    font: Option<Renderer::Font>,
    width: iced_core::Length,
    padding: iced_core::Padding,
    size: Option<iced_core::Pixels>,
    line_height: iced_core::text::LineHeight,
    alignment: iced_core::alignment::Horizontal,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_paste: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    class: Theme::Class<'a>,
    last_status: Option<Status>,
}

impl<'a, Message, Theme, Renderer> NumberInput<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog,
    Renderer: iced_core::text::Renderer,
{
    /// Creates a new [`NumberInput`] with its current value.
    pub fn new(value: &str) -> Self {
        let default_style = DefaultStyle::default();
        Self {
            id: None,
            value: Value::new(value),
            font: None,
            width: iced_core::Length::Fill,
            padding: default_style.padding.into(),
            size: Some(default_style.text_size.into()),
            line_height: iced_core::text::LineHeight::default(),
            alignment: iced_core::alignment::Horizontal::Left,
            on_input: None,
            on_paste: None,
            on_submit: None,
            class: Theme::default(),
            last_status: None,
        }
    }

    /// Sets the [`iced_core::widget::Id`] of the [`NumberInput`].
    pub fn id(mut self, id: impl Into<iced_core::widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the message that should be produced when some text is typed into
    /// the [`NumberInput`].
    ///
    /// If this method is not called, the [`NumberInput`] will be disabled.
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Sets the message that should be produced when some text is typed into
    /// the [`NumberInput`], if `Some`.
    ///
    /// If `None`, the [`NumberInput`] will be disabled.
    pub fn on_input_maybe(mut self, on_input: Option<impl Fn(String) -> Message + 'a>) -> Self {
        self.on_input = on_input.map(|f| Box::new(f) as _);
        self
    }

    /// Sets the message that should be produced when the [`NumberInput`] is
    /// focused and the enter key is pressed.
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Sets the message that should be produced when the [`NumberInput`] is
    /// focused and the enter key is pressed, if `Some`.
    pub fn on_submit_maybe(mut self, on_submit: Option<Message>) -> Self {
        self.on_submit = on_submit;
        self
    }

    /// Sets the message that should be produced when some text is pasted into
    /// the [`NumberInput`].
    pub fn on_paste(mut self, on_paste: impl Fn(String) -> Message + 'a) -> Self {
        self.on_paste = Some(Box::new(on_paste));
        self
    }

    /// Sets the message that should be produced when some text is pasted into
    /// the [`NumberInput`], if `Some`.
    pub fn on_paste_maybe(mut self, on_paste: Option<impl Fn(String) -> Message + 'a>) -> Self {
        self.on_paste = on_paste.map(|f| Box::new(f) as _);
        self
    }

    /// Sets the [`Font`] of the [`NumberInput`].
    ///
    /// [`Font`]: iced_core::text::Renderer::Font
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the width of the [`NumberInput`].
    pub fn width(mut self, width: impl Into<iced_core::Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the [`Padding`] of the [`NumberInput`].
    pub fn padding<P: Into<iced_core::Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the text size of the [`NumberInput`].
    pub fn size(mut self, size: impl Into<iced_core::Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Sets the [`iced_core::text::LineHeight`] of the [`NumberInput`].
    pub fn line_height(mut self, line_height: impl Into<iced_core::text::LineHeight>) -> Self {
        self.line_height = line_height.into();
        self
    }

    /// Sets the horizontal alignment of the [`NumberInput`].
    pub fn align_x(mut self, alignment: impl Into<iced_core::alignment::Horizontal>) -> Self {
        self.alignment = alignment.into();
        self
    }

    /// Sets the style of the [`NumberInput`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`NumberInput`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Lays out the [`NumberInput`], overriding its [`Value`] if provided.
    ///
    /// [`Renderer`]: iced_core::text::Renderer
    pub fn layout(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        renderer: &Renderer,
        limits: &iced_core::layout::Limits,
        value: Option<&Value>,
    ) -> iced_core::layout::Node {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();
        let value = value.unwrap_or(&self.value);

        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let text_size = self.size.unwrap_or_else(|| renderer.default_size());
        let padding = self.padding.fit(iced_core::Size::ZERO, limits.max());
        let height = self.line_height.to_absolute(text_size);

        let limits = limits.width(self.width).shrink(padding);
        let text_bounds = limits.resolve(self.width, height, iced_core::Size::ZERO);

        let _ = state.value.update(iced_core::text::Text {
            font,
            line_height: self.line_height,
            content: &value.to_string(),
            bounds: iced_core::Size::new(f32::INFINITY, text_bounds.height),
            size: text_size,
            align_x: iced_core::text::Alignment::Default,
            align_y: iced_core::alignment::Vertical::Center,
            shaping: iced_core::text::Shaping::Advanced,
            wrapping: iced_core::text::Wrapping::default(),
        });

        let text = iced_core::layout::Node::new(text_bounds)
            .move_to(iced_core::Point::new(padding.left, padding.top));

        iced_core::layout::Node::with_children(text_bounds.expand(padding), vec![text])
    }

    fn input_method<'b>(
        &self,
        state: &'b State<Renderer::Paragraph>,
        layout: iced_core::Layout<'_>,
        value: &Value,
    ) -> iced_core::InputMethod<&'b str> {
        let Some(Focus {
            is_window_focused: true,
            ..
        }) = &state.is_focused
        else {
            return iced_core::InputMethod::Disabled;
        };

        let text_bounds = layout.children().next().unwrap().bounds();

        let caret_index = match state.cursor.state(value) {
            cursor::State::Index(position) => position,
            cursor::State::Selection { start, end } => start.min(end),
        };

        let text = state.value.raw();
        let (cursor_x, scroll_offset) =
            measure_cursor_and_scroll_offset(text, text_bounds, caret_index);

        let alignment_offset =
            alignment_offset(text_bounds.width, text.min_width(), self.alignment);

        let x = (text_bounds.x + cursor_x).floor() - scroll_offset + alignment_offset;

        iced_core::InputMethod::Enabled {
            cursor: iced_core::Rectangle::new(
                iced_core::Point::new(x, text_bounds.y),
                iced_core::Size::new(1.0, text_bounds.height),
            ),
            purpose: iced_core::input_method::Purpose::Normal,
            preedit: state
                .preedit
                .as_ref()
                .map(iced_core::input_method::Preedit::as_ref),
        }
    }

    /// Draws the [`NumberInput`] with the given [`Renderer`], overriding its
    /// [`Value`] if provided.
    ///
    /// [`Renderer`]: iced_core::text::Renderer
    pub fn draw(
        &self,
        tree: &iced_core::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        layout: iced_core::Layout<'_>,
        _cursor: iced_core::mouse::Cursor,
        value: Option<&Value>,
        viewport: &iced_core::Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();
        let value = value.unwrap_or(&self.value);
        let is_disabled = self.on_input.is_none();

        let bounds = layout.bounds();

        let mut children_layout = layout.children();
        let text_bounds = children_layout.next().unwrap().bounds();

        let style = theme.style(&self.class, self.last_status.unwrap_or(Status::Disabled));

        renderer.fill_quad(
            iced_core::renderer::Quad {
                bounds,
                border: style.border,
                ..iced_core::renderer::Quad::default()
            },
            style.background,
        );

        let (cursor, offset, is_selecting) = if let Some(focus) = state
            .is_focused
            .as_ref()
            .filter(|focus| focus.is_window_focused)
        {
            match state.cursor.state(value) {
                cursor::State::Index(position) => {
                    let (text_value_width, offset) =
                        measure_cursor_and_scroll_offset(state.value.raw(), text_bounds, position);

                    let is_cursor_visible = !is_disabled
                        && ((focus.now - focus.updated_at).as_millis()
                            / CURSOR_BLINK_INTERVAL_MILLIS)
                            .is_multiple_of(2);

                    let cursor = if is_cursor_visible {
                        Some((
                            iced_core::renderer::Quad {
                                bounds: iced_core::Rectangle {
                                    x: (text_bounds.x + text_value_width).floor(),
                                    y: text_bounds.y,
                                    width: 1.0,
                                    height: text_bounds.height,
                                },
                                ..iced_core::renderer::Quad::default()
                            },
                            style.value,
                        ))
                    } else {
                        None
                    };

                    (cursor, offset, false)
                }
                cursor::State::Selection { start, end } => {
                    let left = start.min(end);
                    let right = end.max(start);

                    let (left_position, left_offset) =
                        measure_cursor_and_scroll_offset(state.value.raw(), text_bounds, left);

                    let (right_position, right_offset) =
                        measure_cursor_and_scroll_offset(state.value.raw(), text_bounds, right);

                    let width = right_position - left_position;

                    (
                        Some((
                            iced_core::renderer::Quad {
                                bounds: iced_core::Rectangle {
                                    x: text_bounds.x + left_position,
                                    y: text_bounds.y,
                                    width,
                                    height: text_bounds.height,
                                },
                                ..iced_core::renderer::Quad::default()
                            },
                            style.selection,
                        )),
                        if end == right {
                            right_offset
                        } else {
                            left_offset
                        },
                        true,
                    )
                }
            }
        } else {
            (None, 0.0, false)
        };

        let draw = |renderer: &mut Renderer, viewport| {
            let paragraph = state.value.raw();

            let alignment_offset =
                alignment_offset(text_bounds.width, paragraph.min_width(), self.alignment);

            if let Some((cursor, color)) = cursor {
                renderer.with_translation(
                    iced_core::Vector::new(alignment_offset - offset, 0.0),
                    |renderer| {
                        renderer.fill_quad(cursor, color);
                    },
                );
            } else {
                renderer.with_translation(iced_core::Vector::ZERO, |_| {});
            }

            renderer.fill_paragraph(
                paragraph,
                text_bounds.anchor(
                    paragraph.min_bounds(),
                    iced_core::Alignment::Start,
                    iced_core::Alignment::Center,
                ) + iced_core::Vector::new(alignment_offset - offset, 0.0),
                style.value,
                viewport,
            );
        };

        if is_selecting {
            renderer.with_layer(text_bounds, |renderer| draw(renderer, *viewport));
        } else {
            draw(renderer, text_bounds);
        }
    }
}

impl<Message, Theme, Renderer> iced_core::Widget<Message, Theme, Renderer>
    for NumberInput<'_, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog,
    Renderer: iced_core::text::Renderer,
{
    fn tag(&self) -> iced_core::widget::tree::Tag {
        iced_core::widget::tree::Tag::of::<State<Renderer::Paragraph>>()
    }

    fn state(&self) -> iced_core::widget::tree::State {
        iced_core::widget::tree::State::new(State::<Renderer::Paragraph>::new())
    }

    fn diff(&self, tree: &mut iced_core::widget::Tree) {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        // Stop pasting if input becomes disabled
        if self.on_input.is_none() {
            state.is_pasting = None;
        }
    }

    fn size(&self) -> iced_core::Size<iced_core::Length> {
        iced_core::Size {
            width: self.width,
            height: iced_core::Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        renderer: &Renderer,
        limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        self.layout(tree, renderer, limits, None)
    }

    fn operate(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        layout: iced_core::Layout<'_>,
        _renderer: &Renderer,
        operation: &mut dyn iced_core::widget::operation::Operation,
    ) {
        let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

        operation.text_input(self.id.as_ref(), layout.bounds(), state);
        operation.focusable(self.id.as_ref(), layout.bounds(), state);
    }

    fn update(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        event: &iced_core::Event,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced_core::clipboard::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        _viewport: &iced_core::Rectangle,
    ) {
        let update_cache = |state, value| {
            replace_paragraph(
                renderer,
                state,
                layout,
                value,
                self.font,
                self.size,
                self.line_height,
            );
        };

        match &event {
            iced_core::Event::Mouse(iced_core::mouse::Event::ButtonPressed(
                iced_core::mouse::Button::Left,
            ))
            | iced_core::Event::Touch(iced_core::touch::Event::FingerPressed { .. }) => {
                let state = state::<Renderer>(tree);
                let cursor_before = state.cursor;

                let click_position = cursor.position_over(layout.bounds());

                state.is_focused = if click_position.is_some() {
                    let now = iced_core::time::Instant::now();

                    Some(Focus {
                        updated_at: now,
                        now,
                        is_window_focused: true,
                    })
                } else {
                    None
                };

                if let Some(cursor_position) = click_position {
                    let text_layout = layout.children().next().unwrap();

                    let target = {
                        let text_bounds = text_layout.bounds();

                        let alignment_offset = alignment_offset(
                            text_bounds.width,
                            state.value.raw().min_width(),
                            self.alignment,
                        );

                        cursor_position.x - text_bounds.x - alignment_offset
                    };

                    let click = iced_core::mouse::Click::new(
                        cursor_position,
                        iced_core::mouse::Button::Left,
                        state.last_click,
                    );

                    match click.kind() {
                        iced_core::mouse::click::Kind::Single => {
                            let position = if target > 0.0 {
                                find_cursor_position(
                                    text_layout.bounds(),
                                    &self.value,
                                    state,
                                    target,
                                )
                            } else {
                                None
                            }
                            .unwrap_or(0);

                            if state.keyboard_modifiers.shift() {
                                state
                                    .cursor
                                    .select_range(state.cursor.start(&self.value), position);
                            } else {
                                state.cursor.move_to(position);
                            }

                            state.is_dragging = Some(Drag::Select);
                        }
                        iced_core::mouse::click::Kind::Double => {
                            let position = find_cursor_position(
                                text_layout.bounds(),
                                &self.value,
                                state,
                                target,
                            )
                            .unwrap_or(0);

                            state.cursor.select_range(
                                self.value.previous_start_of_word(position),
                                self.value.next_end_of_word(position),
                            );

                            state.is_dragging = Some(Drag::SelectWords { anchor: position });
                        }
                        iced_core::mouse::click::Kind::Triple => {
                            state.cursor.select_all(&self.value);
                            state.is_dragging = None;
                        }
                    }

                    state.last_click = Some(click);

                    if cursor_before != state.cursor {
                        shell.request_redraw();
                    }

                    shell.capture_event();
                }
            }
            iced_core::Event::Mouse(iced_core::mouse::Event::ButtonReleased(
                iced_core::mouse::Button::Left,
            ))
            | iced_core::Event::Touch(iced_core::touch::Event::FingerLifted { .. })
            | iced_core::Event::Touch(iced_core::touch::Event::FingerLost { .. }) => {
                state::<Renderer>(tree).is_dragging = None;
            }
            iced_core::Event::Mouse(iced_core::mouse::Event::CursorMoved { position })
            | iced_core::Event::Touch(iced_core::touch::Event::FingerMoved { position, .. }) => {
                let state = state::<Renderer>(tree);

                if let Some(is_dragging) = &state.is_dragging {
                    let text_layout = layout.children().next().unwrap();

                    let target = {
                        let text_bounds = text_layout.bounds();

                        let alignment_offset = alignment_offset(
                            text_bounds.width,
                            state.value.raw().min_width(),
                            self.alignment,
                        );

                        position.x - text_bounds.x - alignment_offset
                    };

                    let position =
                        find_cursor_position(text_layout.bounds(), &self.value, state, target)
                            .unwrap_or(0);

                    let selection_before = state.cursor.selection(&self.value);

                    match is_dragging {
                        Drag::Select => {
                            state
                                .cursor
                                .select_range(state.cursor.start(&self.value), position);
                        }
                        Drag::SelectWords { anchor } => {
                            if position < *anchor {
                                state.cursor.select_range(
                                    self.value.previous_start_of_word(position),
                                    self.value.next_end_of_word(*anchor),
                                );
                            } else {
                                state.cursor.select_range(
                                    self.value.previous_start_of_word(*anchor),
                                    self.value.next_end_of_word(position),
                                );
                            }
                        }
                    }

                    if let Some(focus) = &mut state.is_focused {
                        focus.updated_at = iced_core::time::Instant::now();
                    }

                    if selection_before != state.cursor.selection(&self.value) {
                        shell.request_redraw();
                    }

                    shell.capture_event();
                }
            }
            iced_core::Event::Keyboard(iced_core::keyboard::Event::KeyPressed {
                key,
                text,
                modified_key,
                physical_key,
                ..
            }) => {
                let state = state::<Renderer>(tree);

                if let Some(focus) = &mut state.is_focused {
                    let modifiers = state.keyboard_modifiers;

                    match key.to_latin(*physical_key) {
                        Some('c') if state.keyboard_modifiers.command() => {
                            if let Some((start, end)) = state.cursor.selection(&self.value) {
                                clipboard.write(
                                    iced_core::clipboard::Kind::Standard,
                                    self.value.select(start, end).to_string(),
                                );
                            }

                            shell.capture_event();
                            return;
                        }
                        Some('x') if state.keyboard_modifiers.command() => {
                            let Some(on_input) = &self.on_input else {
                                return;
                            };

                            if let Some((start, end)) = state.cursor.selection(&self.value) {
                                clipboard.write(
                                    iced_core::clipboard::Kind::Standard,
                                    self.value.select(start, end).to_string(),
                                );
                            }

                            let mut editor = Editor::new(&mut self.value, &mut state.cursor);
                            editor.delete();

                            let message = (on_input)(editor.contents());
                            shell.publish(message);
                            shell.capture_event();

                            focus.updated_at = iced_core::time::Instant::now();
                            update_cache(state, &self.value);
                            return;
                        }
                        Some('v')
                            if state.keyboard_modifiers.command()
                                && !state.keyboard_modifiers.alt() =>
                        {
                            let Some(on_input) = &self.on_input else {
                                return;
                            };

                            let content = match state.is_pasting.take() {
                                Some(content) => content,
                                None => {
                                    let content: String = clipboard
                                        .read(iced_core::clipboard::Kind::Standard)
                                        .unwrap_or_default()
                                        .chars()
                                        .filter(|c| !c.is_control())
                                        .collect();

                                    Value::new(&content)
                                }
                            };

                            let mut editor = Editor::new(&mut self.value, &mut state.cursor);
                            editor.paste(content.clone());

                            let message = if let Some(paste) = &self.on_paste {
                                (paste)(editor.contents())
                            } else {
                                (on_input)(editor.contents())
                            };
                            shell.publish(message);
                            shell.capture_event();

                            state.is_pasting = Some(content);
                            focus.updated_at = iced_core::time::Instant::now();
                            update_cache(state, &self.value);
                            return;
                        }
                        Some('a') if state.keyboard_modifiers.command() => {
                            let cursor_before = state.cursor;

                            state.cursor.select_all(&self.value);

                            if cursor_before != state.cursor {
                                focus.updated_at = iced_core::time::Instant::now();

                                shell.request_redraw();
                            }

                            shell.capture_event();
                            return;
                        }
                        _ => {}
                    }

                    if let Some(text) = text {
                        let Some(on_input) = &self.on_input else {
                            return;
                        };

                        state.is_pasting = None;

                        if let Some(c) = text.chars().next().filter(|c| !c.is_control()) {
                            let mut editor = Editor::new(&mut self.value, &mut state.cursor);

                            editor.insert(c);

                            let message = (on_input)(editor.contents());
                            shell.publish(message);
                            shell.capture_event();

                            focus.updated_at = iced_core::time::Instant::now();
                            update_cache(state, &self.value);
                            return;
                        }
                    }

                    #[cfg(target_os = "macos")]
                    let macos_shortcut = crate::text_editor::convert_macos_shortcut(key, modifiers);

                    #[cfg(target_os = "macos")]
                    let modified_key = macos_shortcut.as_ref().unwrap_or(modified_key);

                    match modified_key.as_ref() {
                        iced_core::keyboard::Key::Named(iced_core::keyboard::key::Named::Enter) => {
                            if let Some(on_submit) = self.on_submit.clone() {
                                shell.publish(on_submit);
                                shell.capture_event();
                            }
                        }
                        iced_core::keyboard::Key::Named(
                            iced_core::keyboard::key::Named::Backspace,
                        ) => {
                            let Some(on_input) = &self.on_input else {
                                return;
                            };

                            if state.cursor.selection(&self.value).is_none() {
                                if modifiers.macos_command() {
                                    state
                                        .cursor
                                        .select_range(state.cursor.start(&self.value), 0);
                                } else if modifiers.jump() {
                                    state.cursor.select_left_by_words(&self.value);
                                }
                            }

                            let mut editor = Editor::new(&mut self.value, &mut state.cursor);
                            editor.backspace();

                            let message = (on_input)(editor.contents());
                            shell.publish(message);
                            shell.capture_event();

                            focus.updated_at = iced_core::time::Instant::now();
                            update_cache(state, &self.value);
                        }
                        iced_core::keyboard::Key::Named(
                            iced_core::keyboard::key::Named::Delete,
                        ) => {
                            let Some(on_input) = &self.on_input else {
                                return;
                            };

                            if state.cursor.selection(&self.value).is_none() {
                                if modifiers.macos_command() {
                                    state.cursor.select_range(
                                        state.cursor.start(&self.value),
                                        self.value.len(),
                                    );
                                } else if modifiers.jump() {
                                    state.cursor.select_right_by_words(&self.value);
                                }
                            }

                            let mut editor = Editor::new(&mut self.value, &mut state.cursor);
                            editor.delete();

                            let message = (on_input)(editor.contents());
                            shell.publish(message);
                            shell.capture_event();

                            focus.updated_at = iced_core::time::Instant::now();
                            update_cache(state, &self.value);
                        }
                        iced_core::keyboard::Key::Named(iced_core::keyboard::key::Named::Home) => {
                            let cursor_before = state.cursor;

                            if modifiers.shift() {
                                state
                                    .cursor
                                    .select_range(state.cursor.start(&self.value), 0);
                            } else {
                                state.cursor.move_to(0);
                            }

                            if cursor_before != state.cursor {
                                focus.updated_at = iced_core::time::Instant::now();

                                shell.request_redraw();
                            }

                            shell.capture_event();
                        }
                        iced_core::keyboard::Key::Named(iced_core::keyboard::key::Named::End) => {
                            let cursor_before = state.cursor;

                            if modifiers.shift() {
                                state.cursor.select_range(
                                    state.cursor.start(&self.value),
                                    self.value.len(),
                                );
                            } else {
                                state.cursor.move_to(self.value.len());
                            }

                            if cursor_before != state.cursor {
                                focus.updated_at = iced_core::time::Instant::now();

                                shell.request_redraw();
                            }

                            shell.capture_event();
                        }
                        iced_core::keyboard::Key::Named(
                            iced_core::keyboard::key::Named::ArrowLeft,
                        ) => {
                            let cursor_before = state.cursor;

                            if modifiers.macos_command() {
                                if modifiers.shift() {
                                    state
                                        .cursor
                                        .select_range(state.cursor.start(&self.value), 0);
                                } else {
                                    state.cursor.move_to(0);
                                }
                            } else if modifiers.jump() {
                                if modifiers.shift() {
                                    state.cursor.select_left_by_words(&self.value);
                                } else {
                                    state.cursor.move_left_by_words(&self.value);
                                }
                            } else if modifiers.shift() {
                                state.cursor.select_left(&self.value);
                            } else {
                                state.cursor.move_left(&self.value);
                            }

                            if cursor_before != state.cursor {
                                focus.updated_at = iced_core::time::Instant::now();

                                shell.request_redraw();
                            }

                            shell.capture_event();
                        }
                        iced_core::keyboard::Key::Named(
                            iced_core::keyboard::key::Named::ArrowRight,
                        ) => {
                            let cursor_before = state.cursor;

                            if modifiers.macos_command() {
                                if modifiers.shift() {
                                    state.cursor.select_range(
                                        state.cursor.start(&self.value),
                                        self.value.len(),
                                    );
                                } else {
                                    state.cursor.move_to(self.value.len());
                                }
                            } else if modifiers.jump() {
                                if modifiers.shift() {
                                    state.cursor.select_right_by_words(&self.value);
                                } else {
                                    state.cursor.move_right_by_words(&self.value);
                                }
                            } else if modifiers.shift() {
                                state.cursor.select_right(&self.value);
                            } else {
                                state.cursor.move_right(&self.value);
                            }

                            if cursor_before != state.cursor {
                                focus.updated_at = iced_core::time::Instant::now();

                                shell.request_redraw();
                            }

                            shell.capture_event();
                        }
                        iced_core::keyboard::Key::Named(
                            iced_core::keyboard::key::Named::Escape,
                        ) => {
                            state.is_focused = None;
                            state.is_dragging = None;
                            state.is_pasting = None;

                            state.keyboard_modifiers = iced_core::keyboard::Modifiers::default();

                            shell.capture_event();
                        }
                        _ => {}
                    }
                }
            }
            iced_core::Event::Keyboard(iced_core::keyboard::Event::KeyReleased { key, .. }) => {
                let state = state::<Renderer>(tree);

                if state.is_focused.is_some()
                    && let iced_core::keyboard::Key::Character("v") = key.as_ref()
                {
                    state.is_pasting = None;

                    shell.capture_event();
                }

                state.is_pasting = None;
            }
            iced_core::Event::Keyboard(iced_core::keyboard::Event::ModifiersChanged(modifiers)) => {
                let state = state::<Renderer>(tree);

                state.keyboard_modifiers = *modifiers;
            }
            iced_core::Event::InputMethod(event) => match event {
                iced_core::input_method::Event::Opened | iced_core::input_method::Event::Closed => {
                    let state = state::<Renderer>(tree);

                    state.preedit = matches!(event, iced_core::input_method::Event::Opened)
                        .then(iced_core::input_method::Preedit::new);

                    shell.request_redraw();
                }
                iced_core::input_method::Event::Preedit(content, selection) => {
                    let state = state::<Renderer>(tree);

                    if state.is_focused.is_some() {
                        state.preedit = Some(iced_core::input_method::Preedit {
                            content: content.to_owned(),
                            selection: selection.clone(),
                            text_size: self.size,
                        });

                        shell.request_redraw();
                    }
                }
                iced_core::input_method::Event::Commit(text) => {
                    let state = state::<Renderer>(tree);

                    if let Some(focus) = &mut state.is_focused {
                        let Some(on_input) = &self.on_input else {
                            return;
                        };

                        let mut editor = Editor::new(&mut self.value, &mut state.cursor);
                        editor.paste(Value::new(text));

                        focus.updated_at = iced_core::time::Instant::now();
                        state.is_pasting = None;

                        let message = (on_input)(editor.contents());
                        shell.publish(message);
                        shell.capture_event();

                        update_cache(state, &self.value);
                    }
                }
            },
            iced_core::Event::Window(iced_core::window::Event::Unfocused) => {
                let state = state::<Renderer>(tree);

                if let Some(focus) = &mut state.is_focused {
                    focus.is_window_focused = false;
                }
            }
            iced_core::Event::Window(iced_core::window::Event::Focused) => {
                let state = state::<Renderer>(tree);

                if let Some(focus) = &mut state.is_focused {
                    focus.is_window_focused = true;
                    focus.updated_at = iced_core::time::Instant::now();

                    shell.request_redraw();
                }
            }
            iced_core::Event::Window(iced_core::window::Event::RedrawRequested(now)) => {
                let state = state::<Renderer>(tree);

                if let Some(focus) = &mut state.is_focused
                    && focus.is_window_focused
                {
                    if matches!(state.cursor.state(&self.value), cursor::State::Index(_)) {
                        focus.now = *now;

                        let millis_until_redraw = CURSOR_BLINK_INTERVAL_MILLIS
                            - (*now - focus.updated_at).as_millis() % CURSOR_BLINK_INTERVAL_MILLIS;

                        shell.request_redraw_at(
                            *now + iced_core::time::Duration::from_millis(
                                millis_until_redraw as u64,
                            ),
                        );
                    }

                    shell.request_input_method(&self.input_method(state, layout, &self.value));
                }
            }
            _ => {}
        }

        let state = state::<Renderer>(tree);
        let is_disabled = self.on_input.is_none();

        let status = if is_disabled {
            Status::Disabled
        } else if state.is_focused() {
            Status::Focused {
                is_hovered: cursor.is_over(layout.bounds()),
            }
        } else if cursor.is_over(layout.bounds()) {
            Status::Hovered
        } else {
            Status::Active
        };

        if let iced_core::Event::Window(iced_core::window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(status);
        } else if self
            .last_status
            .is_some_and(|last_status| status != last_status)
        {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &iced_core::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &iced_core::renderer::Style,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
    ) {
        self.draw(tree, renderer, theme, layout, cursor, None, viewport);
    }

    fn mouse_interaction(
        &self,
        _tree: &iced_core::widget::Tree,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        _viewport: &iced_core::Rectangle,
        _renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            if self.on_input.is_none() {
                iced_core::mouse::Interaction::Idle
            } else {
                iced_core::mouse::Interaction::Text
            }
        } else {
            iced_core::mouse::Interaction::default()
        }
    }
}

impl<'a, Message, Theme, Renderer> From<NumberInput<'a, Message, Theme, Renderer>>
    for iced_core::Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::text::Renderer + 'a,
{
    fn from(
        text_input: NumberInput<'a, Message, Theme, Renderer>,
    ) -> iced_core::Element<'a, Message, Theme, Renderer> {
        iced_core::Element::new(text_input)
    }
}

/// The side of a [`NumberInput`].
#[derive(Debug, Clone)]
pub enum Side {
    /// The left side of a [`NumberInput`].
    Left,
    /// The right side of a [`NumberInput`].
    Right,
}

/// The state of a [`NumberInput`].
#[derive(Debug, Default, Clone)]
pub struct State<P: iced_core::text::Paragraph> {
    value: iced_core::text::paragraph::Plain<P>,
    is_focused: Option<Focus>,
    is_dragging: Option<Drag>,
    is_pasting: Option<Value>,
    preedit: Option<iced_core::input_method::Preedit>,
    last_click: Option<iced_core::mouse::Click>,
    cursor: Cursor,
    keyboard_modifiers: iced_core::keyboard::Modifiers,
    // TODO: Add stateful horizontal scrolling offset
}

fn state<Renderer: iced_core::text::Renderer>(
    tree: &mut iced_core::widget::Tree,
) -> &mut State<Renderer::Paragraph> {
    tree.state.downcast_mut::<State<Renderer::Paragraph>>()
}

#[derive(Debug, Clone)]
struct Focus {
    updated_at: iced_core::time::Instant,
    now: iced_core::time::Instant,
    is_window_focused: bool,
}

#[derive(Debug, Clone)]
enum Drag {
    Select,
    SelectWords { anchor: usize },
}

impl<P: iced_core::text::Paragraph> State<P> {
    /// Creates a new [`State`], representing an unfocused [`NumberInput`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether the [`NumberInput`] is currently focused or not.
    pub fn is_focused(&self) -> bool {
        self.is_focused.is_some()
    }

    /// Returns the [`Cursor`] of the [`NumberInput`].
    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    /// Focuses the [`NumberInput`].
    pub fn focus(&mut self) {
        let now = iced_core::time::Instant::now();

        self.is_focused = Some(Focus {
            updated_at: now,
            now,
            is_window_focused: true,
        });

        self.move_cursor_to_end();
    }

    /// Unfocuses the [`NumberInput`].
    pub fn unfocus(&mut self) {
        self.is_focused = None;
    }

    /// Moves the [`Cursor`] of the [`NumberInput`] to the front of the input text.
    pub fn move_cursor_to_front(&mut self) {
        self.cursor.move_to(0);
    }

    /// Moves the [`Cursor`] of the [`NumberInput`] to the end of the input text.
    pub fn move_cursor_to_end(&mut self) {
        self.cursor.move_to(usize::MAX);
    }

    /// Moves the [`Cursor`] of the [`NumberInput`] to an arbitrary location.
    pub fn move_cursor_to(&mut self, position: usize) {
        self.cursor.move_to(position);
    }

    /// Selects all the content of the [`NumberInput`].
    pub fn select_all(&mut self) {
        self.cursor.select_range(0, usize::MAX);
    }

    /// Selects the given range of the content of the [`NumberInput`].
    pub fn select_range(&mut self, start: usize, end: usize) {
        self.cursor.select_range(start, end);
    }
}

impl<P: iced_core::text::Paragraph> iced_core::widget::operation::Focusable for State<P> {
    fn is_focused(&self) -> bool {
        State::is_focused(self)
    }

    fn focus(&mut self) {
        State::focus(self);
    }

    fn unfocus(&mut self) {
        State::unfocus(self);
    }
}

impl<P: iced_core::text::Paragraph> iced_core::widget::operation::TextInput for State<P> {
    fn text(&self) -> &str {
        self.value.content()
    }

    fn move_cursor_to_front(&mut self) {
        State::move_cursor_to_front(self);
    }

    fn move_cursor_to_end(&mut self) {
        State::move_cursor_to_end(self);
    }

    fn move_cursor_to(&mut self, position: usize) {
        State::move_cursor_to(self, position);
    }

    fn select_all(&mut self) {
        State::select_all(self);
    }

    fn select_range(&mut self, start: usize, end: usize) {
        State::select_range(self, start, end);
    }
}

fn offset<P: iced_core::text::Paragraph>(
    text_bounds: iced_core::Rectangle,
    value: &Value,
    state: &State<P>,
) -> f32 {
    if state.is_focused() {
        let cursor = state.cursor();

        let focus_position = match cursor.state(value) {
            cursor::State::Index(i) => i,
            cursor::State::Selection { end, .. } => end,
        };

        let (_, offset) =
            measure_cursor_and_scroll_offset(state.value.raw(), text_bounds, focus_position);

        offset
    } else {
        0.0
    }
}

fn measure_cursor_and_scroll_offset(
    paragraph: &impl iced_core::text::Paragraph,
    text_bounds: iced_core::Rectangle,
    cursor_index: usize,
) -> (f32, f32) {
    let grapheme_position = paragraph
        .grapheme_position(0, cursor_index)
        .unwrap_or(iced_core::Point::ORIGIN);

    let offset = ((grapheme_position.x + 5.0) - text_bounds.width).max(0.0);

    (grapheme_position.x, offset)
}

/// Computes the position of the text cursor at the given X coordinate of
/// a [`NumberInput`].
fn find_cursor_position<P: iced_core::text::Paragraph>(
    text_bounds: iced_core::Rectangle,
    value: &Value,
    state: &State<P>,
    x: f32,
) -> Option<usize> {
    let offset = offset(text_bounds, value, state);
    let value = value.to_string();

    let char_offset = state
        .value
        .raw()
        .hit_test(iced_core::Point::new(x + offset, text_bounds.height / 2.0))
        .map(iced_core::text::Hit::cursor)?;

    Some(
        unicode_segmentation::UnicodeSegmentation::graphemes(
            &value[..char_offset.min(value.len())],
            true,
        )
        .count(),
    )
}

fn replace_paragraph<Renderer>(
    renderer: &Renderer,
    state: &mut State<Renderer::Paragraph>,
    layout: iced_core::Layout<'_>,
    value: &Value,
    font: Option<Renderer::Font>,
    text_size: Option<iced_core::Pixels>,
    line_height: iced_core::text::LineHeight,
) where
    Renderer: iced_core::text::Renderer,
{
    let font = font.unwrap_or_else(|| renderer.default_font());
    let text_size = text_size.unwrap_or_else(|| renderer.default_size());

    let mut children_layout = layout.children();
    let text_bounds = children_layout.next().unwrap().bounds();

    state.value = iced_core::text::paragraph::Plain::new(iced_core::text::Text {
        font,
        line_height,
        content: value.to_string(),
        bounds: iced_core::Size::new(f32::INFINITY, text_bounds.height),
        size: text_size,
        align_x: iced_core::text::Alignment::Default,
        align_y: iced_core::alignment::Vertical::Center,
        shaping: iced_core::text::Shaping::Advanced,
        wrapping: iced_core::text::Wrapping::default(),
    });
}

const CURSOR_BLINK_INTERVAL_MILLIS: u128 = 500;

/// The possible status of a [`NumberInput`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`NumberInput`] can be interacted with.
    Active,
    /// The [`NumberInput`] is being hovered.
    Hovered,
    /// The [`NumberInput`] is focused.
    Focused {
        /// Whether the [`NumberInput`] is hovered, while focused.
        is_hovered: bool,
    },
    /// The [`NumberInput`] cannot be interacted with.
    Disabled,
}

/// The appearance of a text input.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The [`Background`] of the text input.
    pub background: iced_core::Background,
    /// The [`Border`] of the text input.
    pub border: iced_core::Border,
    /// The [`Color`] of the value of the text input.
    pub value: iced_core::Color,
    /// The [`Color`] of the selection of the text input.
    pub selection: iced_core::Color,
}

/// The theme catalog of a [`NumberInput`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`NumberInput`].
///
/// This is just a boxed closure: `Fn(&Theme, Status) -> Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for iced_core::Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`NumberInput`].
pub fn default(theme: &iced_core::Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let active = Style {
        background: iced_core::Background::Color(palette.background.base.color),
        border: iced_core::Border {
            radius: 2.0.into(),
            width: 1.0,
            color: palette.background.strong.color,
        },
        value: palette.background.base.text,
        selection: palette.primary.weak.color,
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            border: iced_core::Border {
                color: palette.background.base.text,
                ..active.border
            },
            ..active
        },
        Status::Focused { .. } => Style {
            border: iced_core::Border {
                color: palette.primary.strong.color,
                ..active.border
            },
            ..active
        },
        Status::Disabled => Style {
            background: iced_core::Background::Color(palette.background.weak.color),
            value: palette.secondary.base.color,
            ..active
        },
    }
}

fn alignment_offset(
    text_bounds_width: f32,
    text_min_width: f32,
    alignment: iced_core::alignment::Horizontal,
) -> f32 {
    if text_min_width > text_bounds_width {
        0.0
    } else {
        match alignment {
            iced_core::alignment::Horizontal::Left => 0.0,
            iced_core::alignment::Horizontal::Center => (text_bounds_width - text_min_width) / 2.0,
            iced_core::alignment::Horizontal::Right => text_bounds_width - text_min_width,
        }
    }
}
