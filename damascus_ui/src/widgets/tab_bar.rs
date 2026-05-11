//
// The following widget was adapted from iced_aw's TabBar:
// https://github.com/iced-rs/iced_aw/blob/v0.13.0/src/widget/tab_bar.rs
//

/// Displays a [`TabBar`] to select the content to be displayed.
///
/// You have to manage the logic to show the contend by yourself or you may want
/// to use the [`Tabs`](super::tabs::Tabs) widget instead.
use std::marker::PhantomData;

use iced::{
    Alignment, Background, Border, Color, Element, Event, Font, Length, Padding, Pixels, Point,
    Rectangle, Shadow, Size, Theme,
    advanced::{
        layout::{Layout, Limits, Node},
        widget::{Operation, Tree, Widget},
    },
    alignment::{self, Vertical},
    border::Radius,
    mouse::{self, Cursor},
    touch,
    widget::{
        Column, Row, Text,
        text::{self, LineHeight, Wrapping},
    },
    window,
};

pub use crate::icons::Icons;

/// The default icon size.
const DEFAULT_ICON_SIZE: f32 = 16.0;
/// The default text size.
const DEFAULT_TEXT_SIZE: f32 = 16.0;
/// The default size of the close icon.
const DEFAULT_CLOSE_SIZE: f32 = 16.0;
/// The default padding between the tabs.
const DEFAULT_PADDING: Padding = Padding::new(5.0);
/// The default spacing around the tabs.
const DEFAULT_SPACING: Pixels = Pixels::ZERO;

/// Status Enum of an mouse Event.
///
/// The Status of a widget event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// can be pressed.
    Active,
    /// can be pressed and it is being hovered.
    Hovered,
    /// is being pressed.
    Pressed,
    /// cannot be pressed.
    Disabled,
    /// is focused.
    Focused,
    /// is Selected.
    Selected,
}

/// The style function of widget.
pub type StyleFn<'a, Theme, Style> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

/// The appearance of a [`TabBar`](crate::widget::tab_bar::TabBar).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the tab bar.
    pub background: Option<Background>,

    /// The border color of the tab bar.
    pub border_color: Option<Color>,

    /// The border width of the tab bar.
    pub border_width: f32,

    /// The border radius of a tab.
    pub tab_border_radius: Radius,

    /// The background of the tab labels.
    pub tab_label_background: Background,

    /// The border color of the tab labels.
    pub tab_label_border_color: Color,

    /// The border with of the tab labels.
    pub tab_label_border_width: f32,

    /// The icon color of the tab labels.
    pub icon_color: Color,

    /// The color of the closing icon border
    pub icon_background: Option<Background>,

    /// How soft/hard the corners of the icon border are
    pub icon_border_radius: Radius,

    /// The text color of the tab labels.
    pub text_color: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            border_color: None,
            border_width: 0.0,
            tab_border_radius: Radius::default(),
            tab_label_background: Background::Color([0.87, 0.87, 0.87].into()),
            tab_label_border_color: [0.7, 0.7, 0.7].into(),
            tab_label_border_width: 1.0,
            icon_color: Color::BLACK,
            icon_background: Some(Background::Color(Color::TRANSPARENT)),
            icon_border_radius: 4.0.into(),
            text_color: Color::BLACK,
        }
    }
}

/// The primary theme of a [`TabBar`](crate::widget::tab_bar::TabBar).
#[must_use]
pub fn primary(theme: &Theme, status: Status) -> Style {
    let mut base = Style::default();
    let palette = theme.extended_palette();

    base.text_color = palette.background.base.text;

    match status {
        Status::Disabled => {
            base.tab_label_background = Background::Color(palette.background.strong.color);
        }
        Status::Hovered => {
            base.tab_label_background = Background::Color(palette.primary.strong.color);
        }
        _ => {
            base.tab_label_background = Background::Color(palette.primary.base.color);
        }
    }

    base
}

/// The Catalog of a [`TabBar`](crate::widget::tab_bar::TabBar).
pub trait Catalog {
    ///Style for the trait to use.
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self, Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// A [`TabLabel`] showing an icon and/or a text on a tab
/// on a [`TabBar`](super::TabBar).
#[allow(missing_debug_implementations)]
#[derive(Clone, Hash)]
pub enum TabLabel {
    /// A [`TabLabel`] showing only an icon on the tab.
    Icon(char),

    /// A [`TabLabel`] showing only a text on the tab.
    Text(String),

    /// A [`TabLabel`] showing an icon and a text on the tab.
    IconText(char, String),
    // TODO: Support any element as a label.
}

impl From<char> for TabLabel {
    fn from(value: char) -> Self {
        Self::Icon(value)
    }
}

impl From<&str> for TabLabel {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<String> for TabLabel {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<(char, &str)> for TabLabel {
    fn from(value: (char, &str)) -> Self {
        Self::IconText(value.0, value.1.to_owned())
    }
}

impl From<(char, String)> for TabLabel {
    fn from(value: (char, String)) -> Self {
        Self::IconText(value.0, value.1)
    }
}

/// A tab bar to show tabs.
///
/// # Example
/// ```ignore
/// # use iced_aw::{TabLabel, TabBar};
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
///     TabSelected(TabId),
/// }
///
/// #[derive(PartialEq, Hash)]
/// enum TabId {
///    One,
///    Two,
///    Three,
/// }
///
/// let tab_bar = TabBar::new(
///     Message::TabSelected,
/// )
/// .push(TabId::One, TabLabel::Text(String::from("One")))
/// .push(TabId::Two, TabLabel::Text(String::from("Two")))
/// .push(TabId::Three, TabLabel::Text(String::from("Three")))
/// .set_active_tab(&TabId::One);
/// ```
#[allow(missing_debug_implementations)]
pub struct TabBar<
    'a,
    Message,
    TabId,
    Theme = iced::widget::Theme,
    Renderer = iced::widget::Renderer,
> where
    Renderer: iced_core::renderer::Renderer + iced_core::text::Renderer,
    Theme: Catalog,
    TabId: Eq + Clone,
{
    /// The index of the currently active tab.
    active_tab: usize,
    /// The vector containing the labels of the tabs.
    tab_labels: Vec<TabLabel>,
    /// The vector containing the indices of the tabs.
    tab_indices: Vec<TabId>,
    /// The statuses of the [`TabLabel`] and cross
    tab_statuses: Vec<(Option<Status>, Option<bool>)>,
    /// The function that produces the message when a tab is selected.
    on_select: Box<dyn Fn(TabId) -> Message>,
    /// The function that produces the message when the close icon was pressed.
    on_close: Option<Box<dyn Fn(TabId) -> Message>>,
    /// The width of the [`TabBar`].
    width: Length,
    /// The width of the tabs of the [`TabBar`].
    tab_width: Length,
    /// The width of the [`TabBar`].
    height: Length,
    /// The maximum height of the [`TabBar`].
    max_height: f32,
    /// The icon size.
    icon_size: f32,
    /// The text size.
    text_size: f32,
    /// The size of the close icon.
    close_size: f32,
    /// The padding of the tabs of the [`TabBar`].
    padding: Padding,
    /// The spacing of the tabs of the [`TabBar`].
    spacing: Pixels,
    /// The optional icon font of the [`TabBar`].
    font: Option<Font>,
    /// The optional text font of the [`TabBar`].
    text_font: Option<Font>,
    /// The style of the [`TabBar`].
    class: <Theme as Catalog>::Class<'a>,
    /// Where the icon is placed relative to text
    position: Position,
    #[allow(clippy::missing_docs_in_private_items)]
    _renderer: PhantomData<Renderer>,
}

#[derive(Clone, Copy, Default)]
/// The [`Position`] of the icon relative to text, this enum is only relative if [`TabLabel::IconText`] is used.
pub enum Position {
    /// Icon is placed above of the text.
    Top,
    /// Icon is placed right of the text.
    Right,
    /// Icon is placed below of the text.
    Bottom,
    #[default]
    /// Icon is placed left of the text, the default.
    Left,
}

impl<'a, Message, TabId, Theme, Renderer> TabBar<'a, Message, TabId, Theme, Renderer>
where
    Renderer: iced_core::renderer::Renderer
        + iced_core::text::Renderer<Font = iced_core::Font>
        + iced_core::svg::Renderer,
    Theme: Catalog,
    TabId: Eq + Clone,
{
    /// Creates a new [`TabBar`] with the index of the selected tab and a specified
    /// message which will be send when a tab is selected by the user.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn new<F>(on_select: F) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
    {
        Self::with_tab_labels(Vec::new(), on_select)
    }

    /// Similar to [`new`](Self::new) but with a given Vector of the [`TabLabel`]s.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * a vector containing the [`TabLabel`]s of the [`TabBar`].
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn with_tab_labels<F>(tab_labels: Vec<(TabId, TabLabel)>, on_select: F) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
    {
        Self {
            active_tab: 0,
            tab_indices: tab_labels.iter().map(|(id, _)| id.clone()).collect(),
            tab_statuses: tab_labels.iter().map(|_| (None, None)).collect(),
            tab_labels: tab_labels.into_iter().map(|(_, label)| label).collect(),
            on_select: Box::new(on_select),
            on_close: None,
            width: Length::Fill,
            tab_width: Length::Fill,
            height: Length::Shrink,
            max_height: u32::MAX as f32,
            icon_size: DEFAULT_ICON_SIZE,
            text_size: DEFAULT_TEXT_SIZE,
            close_size: DEFAULT_CLOSE_SIZE,
            padding: DEFAULT_PADDING,
            spacing: DEFAULT_SPACING,
            font: None,
            text_font: None,
            class: <Theme as Catalog>::default(),
            position: Position::default(),
            _renderer: PhantomData,
        }
    }

    /// Sets the size of the close icon of the
    /// [`TabLabel`]s of the [`TabBar`].
    #[must_use]
    pub fn close_size(mut self, close_size: f32) -> Self {
        self.close_size = close_size;
        self
    }

    /// Gets the id of the currently active tab on the [`TabBar`].
    #[must_use]
    pub fn get_active_tab_id(&self) -> Option<&TabId> {
        self.tab_indices.get(self.active_tab)
    }

    /// Gets the index of the currently active tab on the [`TabBar`].
    #[must_use]
    pub fn get_active_tab_idx(&self) -> usize {
        self.active_tab
    }

    /// Gets the width of the [`TabBar`].
    #[must_use]
    pub fn get_height(&self) -> Length {
        self.height
    }

    /// Gets the width of the [`TabBar`].
    #[must_use]
    pub fn get_width(&self) -> Length {
        self.width
    }

    /// Sets the height of the [`TabBar`].
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the font of the icons of the
    /// [`TabLabel`]s of the [`TabBar`].
    #[must_use]
    pub fn icon_font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the icon size of the [`TabLabel`]s of the [`TabBar`].
    #[must_use]
    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.icon_size = icon_size;
        self
    }

    /// Sets the maximum height of the [`TabBar`].
    #[must_use]
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }

    /// Sets the message that will be produced when the close icon of a tab
    /// on the [`TabBar`] is pressed.
    ///
    /// Setting this enables the drawing of a close icon on the tabs.
    #[must_use]
    pub fn on_close<F>(mut self, on_close: F) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
    {
        self.on_close = Some(Box::new(on_close));
        self
    }

    /// Sets the padding of the tabs of the [`TabBar`].
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Pushes a [`TabLabel`] to the [`TabBar`].
    #[must_use]
    pub fn push(mut self, id: TabId, tab_label: TabLabel) -> Self {
        self.tab_labels.push(tab_label);
        self.tab_indices.push(id);
        self.tab_statuses.push((None, None));
        self
    }

    /// Gets the amount of tabs on the [`TabBar`].
    #[must_use]
    pub fn size(&self) -> usize {
        self.tab_indices.len()
    }

    /// Sets the spacing between the tabs of the [`TabBar`].
    #[must_use]
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into();
        self
    }

    /// Sets the font of the text of the
    /// [`TabLabel`]s of the [`TabBar`].
    #[must_use]
    pub fn text_font(mut self, text_font: Font) -> Self {
        self.text_font = Some(text_font);
        self
    }

    /// Sets the text size of the [`TabLabel`]s of the [`TabBar`].
    #[must_use]
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = text_size;
        self
    }

    /// Sets the width of a tab on the [`TabBar`].
    #[must_use]
    pub fn tab_width(mut self, width: Length) -> Self {
        self.tab_width = width;
        self
    }

    /// Sets up the active tab on the [`TabBar`].
    #[must_use]
    pub fn set_active_tab(mut self, active_tab: &TabId) -> Self {
        self.active_tab = self
            .tab_indices
            .iter()
            .position(|id| id == active_tab)
            .map_or(0, |a| a);
        self
    }

    #[must_use]
    /// Sets the [`Position`] of the Icon next to Text, Only used in [`TabLabel::IconText`]
    pub fn set_position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// Sets the style of the [`TabBar`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Sets the class of the input of the [`TabBar`].
    #[must_use]
    pub fn class(mut self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the width of the [`TabBar`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<Message, TabId, Theme, Renderer> Widget<Message, Theme, Renderer>
    for TabBar<'_, Message, TabId, Theme, Renderer>
where
    Renderer: iced_core::renderer::Renderer
        + iced_core::text::Renderer<Font = iced_core::Font>
        + iced_core::svg::Renderer,
    Theme: Catalog + iced::widget::svg::Catalog + text::Catalog,
    TabId: Eq + Clone,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        fn layout_icon<Theme, Renderer>(
            icon: &char,
            size: f32,
            font: Option<Font>,
        ) -> Text<'_, Theme, Renderer>
        where
            Renderer: iced_core::text::Renderer,
            Renderer::Font: From<Font>,
            Theme: iced::widget::text::Catalog,
        {
            Text::<Theme, Renderer>::new(icon.to_string())
                .size(size)
                .font(font.unwrap_or_default())
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .shaping(iced_core::text::Shaping::Advanced)
                .width(Length::Shrink)
        }

        fn layout_text<Theme, Renderer>(
            text: &str,
            size: f32,
            font: Option<Font>,
        ) -> Text<'_, Theme, Renderer>
        where
            Renderer: iced_core::text::Renderer,
            Renderer::Font: From<Font>,
            Theme: iced::widget::text::Catalog,
        {
            Text::<Theme, Renderer>::new(text)
                .size(size)
                .font(font.unwrap_or_default())
                .align_x(alignment::Horizontal::Center)
                .shaping(text::Shaping::Advanced)
                .width(Length::Shrink)
        }

        let row = self
            .tab_labels
            .iter()
            .fold(Row::<Message, Theme, Renderer>::new(), |row, tab_label| {
                let mut label_row = Row::new()
                    .push(
                        match tab_label {
                            TabLabel::Icon(icon) => Column::new()
                                .align_x(Alignment::Center)
                                .push(layout_icon(icon, self.icon_size, self.font)),

                            TabLabel::Text(text) => Column::new()
                                .padding(5.0)
                                .align_x(Alignment::Center)
                                .push(layout_text(text, self.text_size, self.text_font)),

                            TabLabel::IconText(icon, text) => {
                                let mut column = Column::new().align_x(Alignment::Center);

                                match self.position {
                                    Position::Top => {
                                        column = column
                                            .push(layout_icon(icon, self.icon_size, self.font))
                                            .push(layout_text(
                                                text,
                                                self.text_size,
                                                self.text_font,
                                            ));
                                    }
                                    Position::Right => {
                                        column = column.push(
                                            Row::new()
                                                .align_y(Alignment::Center)
                                                .push(layout_text(
                                                    text,
                                                    self.text_size,
                                                    self.text_font,
                                                ))
                                                .push(layout_icon(icon, self.icon_size, self.font)),
                                        );
                                    }
                                    Position::Left => {
                                        column = column.push(
                                            Row::new()
                                                .align_y(Alignment::Center)
                                                .push(layout_icon(icon, self.icon_size, self.font))
                                                .push(layout_text(
                                                    text,
                                                    self.text_size,
                                                    self.text_font,
                                                )),
                                        );
                                    }
                                    Position::Bottom => {
                                        column = column
                                            .height(Length::Fill)
                                            .push(layout_text(text, self.text_size, self.text_font))
                                            .push(layout_icon(icon, self.icon_size, self.font));
                                    }
                                }

                                column
                            }
                        }
                        .width(self.tab_width)
                        .height(self.height),
                    )
                    .align_y(Alignment::Center)
                    .padding(self.padding)
                    .width(self.tab_width);

                if self.on_close.is_some() {
                    label_row = label_row.push(
                        Row::new()
                            .width(Length::Fixed(self.close_size))
                            .height(Length::Fixed(self.close_size))
                            .align_y(Alignment::Center),
                    );
                }

                row.push(label_row)
            })
            .width(self.width)
            .height(self.height)
            .spacing(self.spacing)
            .align_y(Alignment::Center);

        let mut element: Element<'_, Message, Theme, Renderer> = Element::new(row);
        let tab_tree = if let Some(child_tree) = tree.children.get_mut(0) {
            child_tree.diff(element.as_widget_mut());
            child_tree
        } else {
            let child_tree = Tree::new(element.as_widget());
            tree.children.insert(0, child_tree);
            &mut tree.children[0]
        };

        element
            .as_widget_mut()
            .layout(tab_tree, renderer, &limits.loose())
    }

    fn update(
        &mut self,
        _state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if cursor
                    .position()
                    .is_some_and(|pos| layout.bounds().contains(pos)) =>
            {
                let tabs_map: Vec<bool> = layout
                    .children()
                    .map(|layout| {
                        cursor
                            .position()
                            .is_some_and(|pos| layout.bounds().contains(pos))
                    })
                    .collect();

                if let Some(new_selected) = tabs_map.iter().position(|b| *b) {
                    shell.publish(
                        self.on_close
                            .as_ref()
                            .filter(|_on_close| {
                                let tab_layout = layout.children().nth(new_selected).expect(
                                    "widget: Layout should have a tab layout at the selected index",
                                );
                                let cross_layout = tab_layout
                                    .children()
                                    .nth(1)
                                    .expect("widget: Layout should have a close layout");

                                cursor
                                    .position()
                                    .is_some_and(|pos| cross_layout.bounds().contains(pos))
                            })
                            .map_or_else(
                                || (self.on_select)(self.tab_indices[new_selected].clone()),
                                |on_close| (on_close)(self.tab_indices[new_selected].clone()),
                            ),
                    );
                    shell.capture_event();
                }
            }
            _ => {}
        }

        let mut request_redraw = false;
        let children = layout.children();
        for ((i, _tab), layout) in self.tab_labels.iter().enumerate().zip(children) {
            let active_idx = self.get_active_tab_idx();
            let tab_status = self.tab_statuses.get_mut(i).expect("Should have a status.");

            let current_status = if cursor.is_over(layout.bounds()) {
                Status::Hovered
            } else if i == active_idx {
                Status::Active
            } else {
                Status::Disabled
            };

            let mut is_cross_hovered = None;
            let mut children = layout.children();
            if let Some(cross_layout) = children.next_back() {
                is_cross_hovered = Some(cursor.is_over(cross_layout.bounds()));
            }

            if let Event::Window(window::Event::RedrawRequested(_now)) = event {
                *tab_status = (Some(current_status), is_cross_hovered);
            } else if tab_status.0.is_some_and(|status| status != current_status)
                || tab_status.1 != is_cross_hovered
            {
                request_redraw = true;
            }
        }

        if request_redraw {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let children = layout.children();
        let mut mouse_interaction = mouse::Interaction::default();

        for layout in children {
            let is_mouse_over = cursor
                .position()
                .is_some_and(|pos| layout.bounds().contains(pos));
            let new_mouse_interaction = if is_mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            };

            if new_mouse_interaction > mouse_interaction {
                mouse_interaction = new_mouse_interaction;
            }
        }

        mouse_interaction
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &iced_core::renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let children = layout.children();
        let is_mouse_over = cursor.position().is_some_and(|pos| bounds.contains(pos));
        let style_sheet = if is_mouse_over {
            Catalog::style(theme, &self.class, Status::Hovered)
        } else {
            Catalog::style(theme, &self.class, Status::Disabled)
        };

        if bounds.intersects(viewport) {
            renderer.fill_quad(
                iced_core::renderer::Quad {
                    bounds,
                    border: Border {
                        radius: (0.0).into(),
                        width: style_sheet.border_width,
                        color: style_sheet.border_color.unwrap_or(Color::TRANSPARENT),
                    },
                    shadow: Shadow::default(),
                    ..iced_core::renderer::Quad::default()
                },
                style_sheet
                    .background
                    .unwrap_or_else(|| Color::TRANSPARENT.into()),
            );
        }

        for ((i, tab), layout) in self.tab_labels.iter().enumerate().zip(children) {
            let tab_status = self.tab_statuses.get(i).expect("Should have a status.");

            draw_tab(
                renderer,
                tab,
                tab_status,
                layout,
                self.position,
                theme,
                &self.class,
                cursor,
                (self.font.unwrap_or(iced::Font::DEFAULT), self.icon_size),
                (self.text_font.unwrap_or_default(), self.text_size),
                viewport,
            );
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        operation.container(None, layout.bounds());

        // Reconstruct the internal structure for operation traversal
        // This allows the simulator to find text in tab labels
        let row = self.tab_labels.iter().fold(
            Row::<Message, Theme, Renderer>::new(),
            |row, tab_label| {
                let label_content: Element<'_, Message, Theme, Renderer> = match tab_label {
                    TabLabel::Icon(icon) => Text::<Theme, Renderer>::new(icon.to_string())
                        .size(self.icon_size)
                        .font(self.font.unwrap_or_default())
                        .into(),
                    TabLabel::Text(text) => Text::<Theme, Renderer>::new(text.clone())
                        .size(self.text_size)
                        .font(self.text_font.unwrap_or_default())
                        .into(),
                    TabLabel::IconText(icon, text) => {
                        let mut column = Column::<Message, Theme, Renderer>::new();
                        match self.position {
                            Position::Top => {
                                column = column
                                    .push(
                                        Text::<Theme, Renderer>::new(icon.to_string())
                                            .size(self.icon_size)
                                            .font(self.font.unwrap_or_default()),
                                    )
                                    .push(
                                        Text::<Theme, Renderer>::new(text.clone())
                                            .size(self.text_size)
                                            .font(self.text_font.unwrap_or_default()),
                                    );
                            }
                            Position::Right => {
                                let inner_row = Row::<Message, Theme, Renderer>::new()
                                    .push(
                                        Text::<Theme, Renderer>::new(text.clone())
                                            .size(self.text_size)
                                            .font(self.text_font.unwrap_or_default()),
                                    )
                                    .push(
                                        Text::<Theme, Renderer>::new(icon.to_string())
                                            .size(self.icon_size)
                                            .font(self.font.unwrap_or_default()),
                                    );
                                column = column.push(inner_row);
                            }
                            Position::Left => {
                                let inner_row = Row::<Message, Theme, Renderer>::new()
                                    .push(
                                        Text::<Theme, Renderer>::new(icon.to_string())
                                            .size(self.icon_size)
                                            .font(self.font.unwrap_or_default()),
                                    )
                                    .push(
                                        Text::<Theme, Renderer>::new(text.clone())
                                            .size(self.text_size)
                                            .font(self.text_font.unwrap_or_default()),
                                    );
                                column = column.push(inner_row);
                            }
                            Position::Bottom => {
                                column = column
                                    .push(
                                        Text::<Theme, Renderer>::new(text.clone())
                                            .size(self.text_size)
                                            .font(self.text_font.unwrap_or_default()),
                                    )
                                    .push(
                                        Text::<Theme, Renderer>::new(icon.to_string())
                                            .size(self.icon_size)
                                            .font(self.font.unwrap_or_default()),
                                    );
                            }
                        }
                        column.into()
                    }
                };

                let mut label_row = Row::<Message, Theme, Renderer>::new().push(label_content);

                // Add close button if present
                if self.on_close.is_some() {
                    label_row = label_row.push(<iced::widget::Svg<'_, Theme> as Into<
                        iced::Element<'_, Message, Theme, Renderer>,
                    >>::into(
                        Icons::Close
                            .as_svg()
                            .width(self.icon_size)
                            .height(self.icon_size),
                    ));
                }

                row.push(label_row)
            },
        );

        let mut element: Element<'_, Message, Theme, Renderer> = Element::new(row);
        let tab_tree = if let Some(child_tree) = tree.children.get_mut(0) {
            child_tree.diff(element.as_widget_mut());
            child_tree
        } else {
            let child_tree = Tree::new(element.as_widget());
            tree.children.insert(0, child_tree);
            &mut tree.children[0]
        };

        element
            .as_widget_mut()
            .operate(tab_tree, layout, renderer, operation);
    }
}

/// Draws a tab.
#[allow(
    clippy::borrowed_box,
    clippy::too_many_lines,
    clippy::too_many_arguments
)]
fn draw_tab<Theme, Renderer>(
    renderer: &mut Renderer,
    tab: &TabLabel,
    tab_status: &(Option<Status>, Option<bool>),
    layout: Layout<'_>,
    position: Position,
    theme: &Theme,
    class: &<Theme as Catalog>::Class<'_>,
    _cursor: Cursor,
    icon_data: (Font, f32),
    text_data: (Font, f32),
    viewport: &Rectangle,
) where
    Renderer: iced_core::renderer::Renderer
        + iced_core::text::Renderer<Font = iced_core::Font>
        + iced_core::svg::Renderer,
    Theme: Catalog + text::Catalog + iced::widget::svg::Catalog,
{
    fn icon_bound_rectangle(item: Option<Layout<'_>>) -> Rectangle {
        item.expect("Graphics: Layout should have an icons layout for an IconText")
            .bounds()
    }

    fn text_bound_rectangle(item: Option<Layout<'_>>) -> Rectangle {
        item.expect("Graphics: Layout should have an texts layout for an IconText")
            .bounds()
    }

    let bounds = layout.bounds();

    let style = Catalog::style(theme, class, tab_status.0.unwrap_or(Status::Disabled));

    let mut children = layout.children();
    let label_layout = children
        .next()
        .expect("Graphics: Layout should have a label layout");
    let mut label_layout_children = label_layout.children();

    if bounds.intersects(viewport) {
        renderer.fill_quad(
            iced_core::renderer::Quad {
                bounds,
                border: Border {
                    radius: style.tab_border_radius,
                    width: style.tab_label_border_width,
                    color: style.tab_label_border_color,
                },
                shadow: Shadow::default(),
                ..iced_core::renderer::Quad::default()
            },
            style.tab_label_background,
        );
    }

    match tab {
        TabLabel::Icon(icon) => {
            let icon_bounds = icon_bound_rectangle(label_layout_children.next());

            renderer.fill_text(
                iced_core::text::Text {
                    content: icon.to_string(),
                    bounds: Size::new(icon_bounds.width, icon_bounds.height),
                    size: Pixels(icon_data.1),
                    font: icon_data.0,
                    align_x: text::Alignment::Center,
                    align_y: Vertical::Center,
                    line_height: LineHeight::Relative(1.3),
                    shaping: iced_core::text::Shaping::Advanced,
                    wrapping: Wrapping::default(),
                },
                Point::new(icon_bounds.center_x(), icon_bounds.center_y()),
                style.icon_color,
                icon_bounds,
            );
        }

        TabLabel::Text(text) => {
            let text_bounds = text_bound_rectangle(label_layout_children.next());

            renderer.fill_text(
                iced_core::text::Text {
                    content: text.clone(),
                    bounds: Size::new(text_bounds.width, text_bounds.height),
                    size: Pixels(text_data.1),
                    font: text_data.0,
                    align_x: text::Alignment::Center,
                    align_y: Vertical::Center,
                    line_height: LineHeight::Relative(1.3),
                    shaping: iced_core::text::Shaping::Advanced,
                    wrapping: Wrapping::default(),
                },
                Point::new(text_bounds.center_x(), text_bounds.center_y()),
                style.text_color,
                text_bounds,
            );
        }
        TabLabel::IconText(icon, text) => {
            let icon_bounds: Rectangle;
            let text_bounds: Rectangle;

            match position {
                Position::Top => {
                    icon_bounds = icon_bound_rectangle(label_layout_children.next());
                    text_bounds = text_bound_rectangle(label_layout_children.next());
                }
                Position::Right => {
                    let mut row_childern = label_layout_children
                        .next()
                        .expect("Graphics: Right Layout should have have a row with one child")
                        .children();
                    text_bounds = text_bound_rectangle(row_childern.next());
                    icon_bounds = icon_bound_rectangle(row_childern.next());
                }
                Position::Left => {
                    let mut row_childern = label_layout_children
                        .next()
                        .expect("Graphics: Left Layout should have have a row with one child")
                        .children();
                    icon_bounds = icon_bound_rectangle(row_childern.next());
                    text_bounds = text_bound_rectangle(row_childern.next());
                }
                Position::Bottom => {
                    text_bounds = text_bound_rectangle(label_layout_children.next());
                    icon_bounds = icon_bound_rectangle(label_layout_children.next());
                }
            }

            renderer.fill_text(
                iced_core::text::Text {
                    content: icon.to_string(),
                    bounds: Size::new(icon_bounds.width, icon_bounds.height),
                    size: Pixels(icon_data.1),
                    font: icon_data.0,
                    align_x: text::Alignment::Center,
                    align_y: Vertical::Center,
                    line_height: LineHeight::Relative(1.3),
                    shaping: iced_core::text::Shaping::Advanced,
                    wrapping: Wrapping::default(),
                },
                Point::new(icon_bounds.center_x(), icon_bounds.center_y()),
                style.icon_color,
                icon_bounds,
            );

            renderer.fill_text(
                iced_core::text::Text {
                    content: text.clone(),
                    bounds: Size::new(text_bounds.width, text_bounds.height),
                    size: Pixels(text_data.1),
                    font: text_data.0,
                    align_x: text::Alignment::Center,
                    align_y: Vertical::Center,
                    line_height: LineHeight::Relative(1.3),
                    shaping: iced_core::text::Shaping::Advanced,
                    wrapping: Wrapping::default(),
                },
                Point::new(text_bounds.center_x(), text_bounds.center_y()),
                style.text_color,
                text_bounds,
            );
        }
    }

    if let Some(cross_layout) = children.next() {
        let cross_bounds = cross_layout.bounds();
        let is_mouse_over_cross = tab_status.1.unwrap_or(false);

        renderer.draw_svg(
            Icons::Close.as_core_svg::<Theme>(),
            cross_bounds,
            cross_bounds,
        );

        if is_mouse_over_cross && cross_bounds.intersects(viewport) {
            renderer.fill_quad(
                iced_core::renderer::Quad {
                    bounds: cross_bounds,
                    border: Border {
                        radius: style.icon_border_radius,
                        width: style.border_width,
                        color: style.border_color.unwrap_or(Color::TRANSPARENT),
                    },
                    shadow: Shadow::default(),
                    ..iced_core::renderer::Quad::default()
                },
                style
                    .icon_background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }
    }
}

impl<'a, Message, TabId, Theme, Renderer> From<TabBar<'a, Message, TabId, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a
        + iced_core::renderer::Renderer
        + iced_core::text::Renderer<Font = iced_core::Font>
        + iced_core::svg::Renderer,
    Theme: 'a + Catalog + text::Catalog + iced::widget::svg::Catalog,
    Message: 'a,
    TabId: 'a + Eq + Clone,
{
    fn from(tab_bar: TabBar<'a, Message, TabId, Theme, Renderer>) -> Self {
        Element::new(tab_bar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum TestTabId {
        One,
        Two,
        Three,
    }

    #[derive(Clone)]
    #[allow(dead_code)]
    enum TestMessage {
        TabSelected(TestTabId),
        TabClosed(TestTabId),
    }

    type TestTabBar<'a> =
        TabBar<'a, TestMessage, TestTabId, iced::widget::Theme, iced::widget::Renderer>;

    #[test]
    fn tab_bar_new_has_default_values() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected);

        assert_eq!(tab_bar.active_tab, 0);
        assert_eq!(tab_bar.tab_labels.len(), 0);
        assert_eq!(tab_bar.tab_indices.len(), 0);
        assert_eq!(tab_bar.width, Length::Fill);
        assert_eq!(tab_bar.height, Length::Shrink);
        assert!((tab_bar.icon_size - DEFAULT_ICON_SIZE).abs() < f32::EPSILON);
        assert!((tab_bar.text_size - DEFAULT_TEXT_SIZE).abs() < f32::EPSILON);
        assert!((tab_bar.close_size - DEFAULT_CLOSE_SIZE).abs() < f32::EPSILON);
    }

    #[test]
    fn tab_bar_push_adds_tab() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected)
            .push(TestTabId::One, TabLabel::Text("Tab 1".to_owned()));

        assert_eq!(tab_bar.tab_labels.len(), 1);
        assert_eq!(tab_bar.tab_indices.len(), 1);
        assert_eq!(tab_bar.tab_indices[0], TestTabId::One);
    }

    #[test]
    fn tab_bar_push_multiple_tabs() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected)
            .push(TestTabId::One, TabLabel::Text("Tab 1".to_owned()))
            .push(TestTabId::Two, TabLabel::Text("Tab 2".to_owned()))
            .push(TestTabId::Three, TabLabel::Text("Tab 3".to_owned()));

        assert_eq!(tab_bar.tab_labels.len(), 3);
        assert_eq!(tab_bar.tab_indices.len(), 3);
    }

    #[test]
    fn tab_bar_set_active_tab_sets_correct_index() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected)
            .push(TestTabId::One, TabLabel::Text("Tab 1".to_owned()))
            .push(TestTabId::Two, TabLabel::Text("Tab 2".to_owned()))
            .push(TestTabId::Three, TabLabel::Text("Tab 3".to_owned()))
            .set_active_tab(&TestTabId::Two);

        assert_eq!(tab_bar.active_tab, 1);
        assert_eq!(tab_bar.get_active_tab_id(), Some(&TestTabId::Two));
    }

    #[test]
    fn tab_bar_get_active_tab_idx_returns_index() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected)
            .push(TestTabId::One, TabLabel::Text("Tab 1".to_owned()))
            .push(TestTabId::Two, TabLabel::Text("Tab 2".to_owned()))
            .set_active_tab(&TestTabId::Two);

        assert_eq!(tab_bar.get_active_tab_idx(), 1);
    }

    #[test]
    fn tab_bar_size_returns_number_of_tabs() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected)
            .push(TestTabId::One, TabLabel::Text("Tab 1".to_owned()))
            .push(TestTabId::Two, TabLabel::Text("Tab 2".to_owned()))
            .push(TestTabId::Three, TabLabel::Text("Tab 3".to_owned()));

        assert_eq!(tab_bar.size(), 3);
    }

    #[test]
    fn tab_bar_width_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).width(200);
        assert_eq!(tab_bar.width, Length::Fixed(200.0));
    }

    #[test]
    fn tab_bar_height_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).height(50);
        assert_eq!(tab_bar.height, Length::Fixed(50.0));
    }

    #[test]
    fn tab_bar_icon_size_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).icon_size(24.0);
        assert!((tab_bar.icon_size - 24.0).abs() < f32::EPSILON);
    }

    #[test]
    fn tab_bar_text_size_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).text_size(20.0);
        assert!((tab_bar.text_size - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn tab_bar_close_size_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).close_size(18.0);
        assert!((tab_bar.close_size - 18.0).abs() < f32::EPSILON);
    }

    #[test]
    fn tab_bar_on_close_enables_close_button() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).on_close(TestMessage::TabClosed);

        assert!(tab_bar.on_close.is_some());
    }

    #[test]
    fn tab_bar_with_tab_labels_creates_tabs() {
        let labels = vec![
            (TestTabId::One, TabLabel::Text("Tab 1".to_owned())),
            (TestTabId::Two, TabLabel::Text("Tab 2".to_owned())),
        ];

        let tab_bar = TestTabBar::with_tab_labels(labels, TestMessage::TabSelected);

        assert_eq!(tab_bar.tab_labels.len(), 2);
        assert_eq!(tab_bar.tab_indices.len(), 2);
    }

    #[test]
    fn tab_bar_tab_width_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).tab_width(Length::Fixed(100.0));
        assert_eq!(tab_bar.tab_width, Length::Fixed(100.0));
    }

    #[test]
    fn tab_bar_max_height_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).max_height(200.0);
        assert!((tab_bar.max_height - 200.0).abs() < f32::EPSILON);
    }

    #[test]
    fn tab_bar_padding_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).padding(10.0);
        assert_eq!(tab_bar.padding, Padding::from(10.0));
    }

    #[test]
    fn tab_bar_spacing_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).spacing(5.0);
        assert_eq!(tab_bar.spacing, Pixels::from(5.0));
    }
}
