//
// The following widget was adapted from iced_aw's TabBar:
// https://github.com/iced-rs/iced_aw/blob/v0.13.0/src/widget/tab_bar.rs
//

use std::marker::PhantomData;

use iced;

pub use crate::{icons::Icons, widgets::style::Style as DefaultStyle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Hovered,
    Pressed,
    Inactive,
    Focused,
    Selected,
}

pub type StyleFn<'a, Theme, Style> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub background: iced::Background,
    pub text_colour: iced::Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: iced::Background::Color(iced::Color::WHITE),
            text_colour: iced::Color::BLACK,
        }
    }
}

/// The primary theme of a [`TabBar`](crate::widget::tab_bar::TabBar).
#[must_use]
pub fn primary(theme: &iced::Theme, status: Status) -> Style {
    let mut base = Style::default();
    let palette = theme.extended_palette();

    base.text_colour = palette.background.base.text;

    match status {
        Status::Inactive => {
            base.text_colour = palette.background.weakest.text;
            base.background = iced::Background::Color(palette.background.weakest.color);
        }
        Status::Hovered => {
            base.text_colour = palette.background.weaker.text;
            base.background = iced::Background::Color(palette.background.weaker.color);
        }
        _ => {
            base.text_colour = palette.background.neutral.text;
            base.background = iced::Background::Color(palette.background.base.color);
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

impl Catalog for iced::Theme {
    type Class<'a> = StyleFn<'a, Self, Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

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
    tab_labels: Vec<String>,
    /// The vector containing the indices of the tabs.
    tab_indices: Vec<TabId>,
    /// The statuses of the tab and close_button
    tab_statuses: Vec<(Option<Status>, bool)>,
    /// The function that produces the message when a tab is selected.
    on_select: Box<dyn Fn(TabId) -> Message>,
    /// The function that produces the message when the close icon was pressed.
    on_close: Option<Box<dyn Fn(TabId) -> Message>>,
    /// The width of the [`TabBar`].
    width: iced::Length,
    /// The width of the tabs of the [`TabBar`].
    tab_width: iced::Length,
    /// The width of the [`TabBar`].
    height: iced::Length,
    /// The minimum height.
    minimum_height: f32,
    /// The text size.
    text_size: f32,
    /// The size of the close icon.
    close_size: f32,
    /// The padding of the tabs of the [`TabBar`].
    padding: iced::Padding,
    /// The spacing of the tabs of the [`TabBar`].
    spacing: iced::Pixels,
    /// The optional icon font of the [`TabBar`].
    font: Option<iced::Font>,
    /// The optional text font of the [`TabBar`].
    text_font: Option<iced::Font>,
    /// The style of the [`TabBar`].
    class: <Theme as Catalog>::Class<'a>,
    #[allow(clippy::missing_docs_in_private_items)]
    _renderer: PhantomData<Renderer>,
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

    /// Similar to [`new`](Self::new) but with a given Vector of the [`String`]s.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * a vector containing the [`String`]s of the [`TabBar`].
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn with_tab_labels<F>(tab_labels: Vec<(TabId, String)>, on_select: F) -> Self
    where
        F: 'static + Fn(TabId) -> Message,
    {
        let default_style = DefaultStyle::default();

        Self {
            active_tab: 0,
            tab_indices: tab_labels.iter().map(|(id, _)| id.clone()).collect(),
            tab_statuses: tab_labels.iter().map(|_| (None, false)).collect(),
            tab_labels: tab_labels.into_iter().map(|(_, label)| label).collect(),
            on_select: Box::new(on_select),
            on_close: None,
            width: iced::Length::Fill,
            tab_width: iced::Length::Fill,
            height: iced::Length::Shrink,
            minimum_height: (default_style.icon_size as f32).max(default_style.text_size),
            text_size: default_style.text_size,
            close_size: default_style.icon_size as f32,
            padding: default_style.padding.into(),
            spacing: default_style.spacing.into(),
            font: None,
            text_font: None,
            class: <Theme as Catalog>::default(),
            _renderer: PhantomData,
        }
    }

    /// Sets the size of the close icon of the
    /// tabs of the [`TabBar`].
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
    pub fn get_height(&self) -> iced::Length {
        self.height
    }

    /// Gets the width of the [`TabBar`].
    #[must_use]
    pub fn get_width(&self) -> iced::Length {
        self.width
    }

    /// Sets the height of the [`TabBar`].
    #[must_use]
    pub fn height(mut self, height: impl Into<iced::Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the font of the icons of the
    /// tabs of the [`TabBar`].
    #[must_use]
    pub fn icon_font(mut self, font: iced::Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the minimum height of the tabs of the [`TabBar`].
    #[must_use]
    pub fn minimum_height(mut self, minimum_height: f32) -> Self {
        self.minimum_height = minimum_height;
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
    pub fn padding(mut self, padding: impl Into<iced::Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Pushes a tab to the [`TabBar`].
    #[must_use]
    pub fn push(mut self, id: TabId, tab_label: String) -> Self {
        self.tab_labels.push(tab_label);
        self.tab_indices.push(id);
        self.tab_statuses.push((None, false));
        self
    }

    /// Gets the amount of tabs on the [`TabBar`].
    #[must_use]
    pub fn size(&self) -> usize {
        self.tab_indices.len()
    }

    /// Sets the spacing between the tabs of the [`TabBar`].
    #[must_use]
    pub fn spacing(mut self, spacing: impl Into<iced::Pixels>) -> Self {
        self.spacing = spacing.into();
        self
    }

    /// Sets the font of the text of the
    /// tabs of the [`TabBar`].
    #[must_use]
    pub fn text_font(mut self, text_font: iced::Font) -> Self {
        self.text_font = Some(text_font);
        self
    }

    /// Sets the text size of the tabs of the [`TabBar`].
    #[must_use]
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = text_size;
        self
    }

    /// Sets the width of a tab on the [`TabBar`].
    #[must_use]
    pub fn tab_width(mut self, width: iced::Length) -> Self {
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
    pub fn width(mut self, width: impl Into<iced::Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<Message, TabId, Theme, Renderer> iced::advanced::widget::Widget<Message, Theme, Renderer>
    for TabBar<'_, Message, TabId, Theme, Renderer>
where
    Renderer: iced_core::renderer::Renderer
        + iced_core::text::Renderer<Font = iced_core::Font>
        + iced_core::svg::Renderer,
    Theme: Catalog + iced::widget::svg::Catalog + iced::widget::text::Catalog,
    TabId: Eq + Clone,
{
    fn size(&self) -> iced::Size<iced::Length> {
        iced::Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let row = self
            .tab_labels
            .iter()
            .fold(
                iced::widget::Row::<Message, Theme, Renderer>::new(),
                |row, tab_label| {
                    let mut label_row = iced::widget::Row::new()
                        .push(
                            iced::widget::Text::<Theme, Renderer>::new(tab_label)
                                .size(self.text_size)
                                .font(self.text_font.unwrap_or_default())
                                .align_x(iced::alignment::Horizontal::Center)
                                .shaping(iced::widget::text::Shaping::Advanced)
                                .width(iced::Length::Shrink),
                        )
                        .align_y(iced::Alignment::Center)
                        .padding(self.padding)
                        .spacing(self.spacing)
                        .width(self.tab_width);

                    if self.on_close.is_some() {
                        label_row = label_row.push(
                            iced::widget::Row::new()
                                .width(iced::Length::Fixed(self.close_size + self.padding.x()))
                                .height(iced::Length::Fixed(
                                    (self.close_size + self.padding.y()).max(self.minimum_height),
                                ))
                                .align_y(iced::Alignment::Center),
                        );
                    }

                    row.push(label_row)
                },
            )
            .width(self.width)
            .height(self.height)
            .spacing(self.spacing)
            .align_y(iced::Alignment::Center);

        let mut element: iced::Element<'_, Message, Theme, Renderer> = iced::Element::new(row);
        let tab_tree = if let Some(child_tree) = tree.children.get_mut(0) {
            child_tree.diff(element.as_widget_mut());
            child_tree
        } else {
            let child_tree = iced::advanced::widget::Tree::new(element.as_widget());
            tree.children.insert(0, child_tree);
            &mut tree.children[0]
        };

        element
            .as_widget_mut()
            .layout(tab_tree, renderer, &limits.loose())
    }

    fn update(
        &mut self,
        _state: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: iced::advanced::layout::Layout<'_>,
        cursor: iced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) {
        match event {
            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left))
            | iced::Event::Touch(iced::touch::Event::FingerPressed { .. })
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
                                let close_button_layout = tab_layout
                                    .children()
                                    .nth(1)
                                    .expect("widget: Layout should have a close layout");

                                cursor
                                    .position()
                                    .is_some_and(|pos| close_button_layout.bounds().contains(pos))
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
            let Some((tab_status, close_button_hovered)) = self.tab_statuses.get_mut(i) else {
                continue;
            };

            let current_status = if i == active_idx {
                Status::Active
            } else if cursor.is_over(layout.bounds()) {
                Status::Hovered
            } else {
                Status::Inactive
            };

            let mut is_close_button_hovered = false;
            let mut children = layout.children();
            if let Some(close_button_layout) = children.next_back() {
                is_close_button_hovered = cursor.is_over(close_button_layout.bounds());
            }

            if let iced::Event::Window(iced::window::Event::RedrawRequested(_now)) = event {
                *tab_status = Some(current_status);
                *close_button_hovered = is_close_button_hovered;
            } else if tab_status.is_some_and(|status| status != current_status)
                || *close_button_hovered != is_close_button_hovered
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
        _state: &iced::advanced::widget::Tree,
        layout: iced::advanced::layout::Layout<'_>,
        cursor: iced::mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced::mouse::Interaction {
        let children = layout.children();
        let mut mouse_interaction = iced::mouse::Interaction::default();

        for layout in children {
            let is_mouse_over = cursor
                .position()
                .is_some_and(|pos| layout.bounds().contains(pos));
            let new_mouse_interaction = if is_mouse_over {
                iced::mouse::Interaction::Pointer
            } else {
                iced::mouse::Interaction::default()
            };

            if new_mouse_interaction > mouse_interaction {
                mouse_interaction = new_mouse_interaction;
            }
        }

        mouse_interaction
    }

    fn draw(
        &self,
        _state: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &iced_core::renderer::Style,
        layout: iced::advanced::layout::Layout<'_>,
        _cursor: iced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        for ((id, tab_label), layout) in self.tab_labels.iter().enumerate().zip(layout.children()) {
            let Some((tab_status, close_button_hovered)) = self.tab_statuses.get(id) else {
                continue;
            };

            let bounds = layout.bounds();

            let style = Catalog::style(theme, &self.class, tab_status.unwrap_or(Status::Inactive));

            if bounds.intersects(viewport) {
                renderer.fill_quad(
                    iced_core::renderer::Quad {
                        bounds,
                        ..iced_core::renderer::Quad::default()
                    },
                    style.background,
                );
            }

            let mut children = layout.children();

            if let Some(label_layout) = children.next() {
                let text_bounds: iced::Rectangle = label_layout.bounds();

                renderer.fill_text(
                    iced_core::text::Text {
                        content: tab_label.to_string(),
                        bounds: iced::Size::new(text_bounds.width, text_bounds.height),
                        size: self.text_size.into(),
                        font: self.text_font.unwrap_or_default(),
                        align_x: iced::widget::text::Alignment::Center,
                        align_y: iced::alignment::Vertical::Center,
                        line_height: iced::widget::text::LineHeight::Relative(1.),
                        shaping: iced_core::text::Shaping::Advanced,
                        wrapping: iced::widget::text::Wrapping::default(),
                    },
                    iced::Point::new(text_bounds.center_x(), text_bounds.center_y()),
                    style.text_colour,
                    text_bounds,
                );
            }

            if let Some(close_button_layout) = children.next() {
                let close_button_style = Catalog::style(theme, &self.class, Status::Inactive);

                let half_close_size: f32 = 0.5 * self.close_size;

                let mut close_button_bounds = close_button_layout.bounds();
                close_button_bounds.x = close_button_bounds.center_x() - half_close_size;
                close_button_bounds.y = close_button_bounds.center_y() - half_close_size;
                close_button_bounds.height = self.close_size;
                close_button_bounds.width = self.close_size;

                if close_button_bounds.is_within(viewport) {
                    renderer.draw_svg(Icons::Close.as_core_svg(), close_button_bounds, *viewport);

                    if *close_button_hovered {
                        close_button_bounds = close_button_bounds.expand(self.padding);

                        renderer.fill_quad(
                            iced_core::renderer::Quad {
                                bounds: close_button_bounds,
                                ..iced_core::renderer::Quad::default()
                            },
                            close_button_style.background,
                        );
                    }
                }
            }
        }
    }

    fn operate(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation<()>,
    ) {
        operation.container(None, layout.bounds());

        // Reconstruct the internal structure for operation traversal
        // This allows the simulator to find text in tab labels
        let row: iced::widget::Row<'_, Message, Theme, Renderer> = self.tab_labels.iter().fold(
            iced::widget::Row::<Message, Theme, Renderer>::new(),
            |row, tab_label| {
                let label_content: iced::Element<'_, Message, Theme, Renderer> =
                    iced::widget::Text::<Theme, Renderer>::new(tab_label.clone())
                        .size(self.text_size)
                        .font(self.text_font.unwrap_or_default())
                        .into();

                row.push(label_content)
            },
        );

        let mut element: iced::Element<'_, Message, Theme, Renderer> = iced::Element::new(row);
        let tab_tree = if let Some(child_tree) = tree.children.get_mut(0) {
            child_tree.diff(element.as_widget_mut());
            child_tree
        } else {
            let child_tree = iced::advanced::widget::Tree::new(element.as_widget());
            tree.children.insert(0, child_tree);
            &mut tree.children[0]
        };

        element
            .as_widget_mut()
            .operate(tab_tree, layout, renderer, operation);
    }
}

impl<'a, Message, TabId, Theme, Renderer> From<TabBar<'a, Message, TabId, Theme, Renderer>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a
        + iced_core::renderer::Renderer
        + iced_core::text::Renderer<Font = iced_core::Font>
        + iced_core::svg::Renderer,
    Theme: 'a + Catalog + iced::widget::text::Catalog + iced::widget::svg::Catalog,
    Message: 'a,
    TabId: 'a + Eq + Clone,
{
    fn from(tab_bar: TabBar<'a, Message, TabId, Theme, Renderer>) -> Self {
        iced::Element::new(tab_bar)
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

        let default_style = DefaultStyle::default();

        assert_eq!(tab_bar.active_tab, 0);
        assert_eq!(tab_bar.tab_labels.len(), 0);
        assert_eq!(tab_bar.tab_indices.len(), 0);
        assert_eq!(tab_bar.width, iced::Length::Fill);
        assert_eq!(tab_bar.height, iced::Length::Shrink);
        assert!((tab_bar.minimum_height - default_style.icon_size as f32).abs() < f32::EPSILON);
        assert!((tab_bar.text_size - default_style.text_size).abs() < f32::EPSILON);
        assert!((tab_bar.close_size - default_style.icon_size as f32).abs() < f32::EPSILON);
    }

    #[test]
    fn tab_bar_push_adds_tab() {
        let tab_bar =
            TestTabBar::new(TestMessage::TabSelected).push(TestTabId::One, "Tab 1".to_string());

        assert_eq!(tab_bar.tab_labels.len(), 1);
        assert_eq!(tab_bar.tab_indices.len(), 1);
        assert_eq!(tab_bar.tab_indices[0], TestTabId::One);
    }

    #[test]
    fn tab_bar_push_multiple_tabs() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected)
            .push(TestTabId::One, "Tab 1".to_string())
            .push(TestTabId::Two, "Tab 2".to_string())
            .push(TestTabId::Three, "Tab 3".to_string());

        assert_eq!(tab_bar.tab_labels.len(), 3);
        assert_eq!(tab_bar.tab_indices.len(), 3);
    }

    #[test]
    fn tab_bar_set_active_tab_sets_correct_index() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected)
            .push(TestTabId::One, "Tab 1".to_string())
            .push(TestTabId::Two, "Tab 2".to_string())
            .push(TestTabId::Three, "Tab 3".to_string())
            .set_active_tab(&TestTabId::Two);

        assert_eq!(tab_bar.active_tab, 1);
        assert_eq!(tab_bar.get_active_tab_id(), Some(&TestTabId::Two));
    }

    #[test]
    fn tab_bar_get_active_tab_idx_returns_index() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected)
            .push(TestTabId::One, "Tab 1".to_string())
            .push(TestTabId::Two, "Tab 2".to_string())
            .set_active_tab(&TestTabId::Two);

        assert_eq!(tab_bar.get_active_tab_idx(), 1);
    }

    #[test]
    fn tab_bar_size_returns_number_of_tabs() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected)
            .push(TestTabId::One, "Tab 1".to_string())
            .push(TestTabId::Two, "Tab 2".to_string())
            .push(TestTabId::Three, "Tab 3".to_string());

        assert_eq!(tab_bar.size(), 3);
    }

    #[test]
    fn tab_bar_width_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).width(200);
        assert_eq!(tab_bar.width, iced::Length::Fixed(200.0));
    }

    #[test]
    fn tab_bar_height_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).height(50);
        assert_eq!(tab_bar.height, iced::Length::Fixed(50.0));
    }

    #[test]
    fn tab_bar_minimum_height_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).minimum_height(24.0);
        assert!((tab_bar.minimum_height - 24.0).abs() < f32::EPSILON);
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
            (TestTabId::One, "Tab 1".to_string()),
            (TestTabId::Two, "Tab 2".to_string()),
        ];

        let tab_bar = TestTabBar::with_tab_labels(labels, TestMessage::TabSelected);

        assert_eq!(tab_bar.tab_labels.len(), 2);
        assert_eq!(tab_bar.tab_indices.len(), 2);
    }

    #[test]
    fn tab_bar_tab_width_sets_value() {
        let tab_bar =
            TestTabBar::new(TestMessage::TabSelected).tab_width(iced::Length::Fixed(100.0));
        assert_eq!(tab_bar.tab_width, iced::Length::Fixed(100.0));
    }

    #[test]
    fn tab_bar_padding_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).padding(10.0);
        assert_eq!(tab_bar.padding, iced::Padding::from(10.0));
    }

    #[test]
    fn tab_bar_spacing_sets_value() {
        let tab_bar = TestTabBar::new(TestMessage::TabSelected).spacing(5.0);
        assert_eq!(tab_bar.spacing, iced::Pixels::from(5.0));
    }
}
