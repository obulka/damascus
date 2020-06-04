// 3rd Party Imports

use iced::{
    Align,
    Button,
    button,
    Column,
    Container,
    Element,
    HorizontalAlignment,
    Length,
    pane_grid,
    Row,
    Space,
    Text,
    VerticalAlignment,
};


// Local Imports
use crate::action::Message;
use crate::state::{
    Config,
    widget::{
        Tab,
        TabContent,
        TabType,
        tabs::{
            tab_content_from_type,
        },
    },
};
use crate::state::style;


pub struct Panel {
    split_horizontally: button::State,
    split_vertically: button::State,
    float_pane: button::State, // Not Implemented
    close: button::State,
    tabs: Vec<(String, button::State)>,
    tab_contents: Vec<Box<dyn TabContent>>,
    focused_tab: usize,
}

impl Panel {
    pub fn new() -> Self {
        Panel {
            split_horizontally: button::State::new(),
            split_vertically: button::State::new(),
            float_pane: button::State::new(),
            close: button::State::new(),
            tabs: Vec::new(),
            tab_contents: Vec::new(),
            focused_tab: 0,
        }
    }

    pub fn view(
        &mut self,
        pane: pane_grid::Pane,
        focus: Option<pane_grid::Focus>,
        config: &Config,
    ) -> Element<Message> {
        let Panel {
            split_horizontally,
            split_vertically,
            float_pane,
            close,
            tabs,
            tab_contents,
            focused_tab,
        } = self;

        let button = |state, label, message, style| {
            Button::new(
                state,
                Text::new(label)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center)
                    .size(8),
            )
            .width(Length::Fill)
            .padding(1)
            .on_press(message)
            .style(style)
        };
        let options = Column::new()
            .spacing(2)
            .width(Length::Shrink)
            .height(Length::Fill)
            .max_width(config.tab_bar_height)
            .max_height(config.tab_bar_height)
            .padding(3)
            .push(Row::new()
                .spacing(2)
                .width(Length::Fill)
                .height(Length::Fill)
                .max_width(config.tab_bar_height)
                .max_height(config.tab_bar_height)
                .push(button(
                    split_vertically,
                    "|",
                    Message::Split(pane_grid::Axis::Vertical, pane),
                    config.theme.button_style(
                        style::Button::Primary,
                    ),
                ))
                .push(button(
                    close,
                    "×",
                    Message::Close(pane),
                    config.theme.button_style(
                        style::Button::Destructive,
                    ),
                ))
            )
            .push(
                Row::new()
                    .spacing(2)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .max_width(config.tab_bar_height)
                    .max_height(config.tab_bar_height)
                    .push(button(
                        float_pane,
                        "+",
                        Message::OpenTabFocused(TabType::Viewer),
                        config.theme.button_style(
                            style::Button::Primary,
                        ),
                    ))
                    .push(button(
                        split_horizontally,
                        "─",
                        Message::Split(pane_grid::Axis::Horizontal, pane),
                        config.theme.button_style(
                            style::Button::Primary,
                        ),
                    ))
            );

        let tab_bar = Container::new(
            Row::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(Align::End)
                .spacing(1)
                .padding(0)
                .push(
                    Row::new()
                        .width(Length::Shrink)
                        .max_width(1)
                        .push(Space::with_width(Length::Fill)) // Hack to get space before tab
                )
                .push(
                    tabs.iter_mut().enumerate().fold(
                        Row::new().padding(0).spacing(0),
                        |row_of_tabs, (index, (tab_label, close_tab_state))| {
                            let focused = *focused_tab == index;
                            row_of_tabs.push(
                                Tab::new(
                                    Row::new().padding(7).spacing(5).max_width(150)
                                        .push(
                                            Text::new(tab_label.to_string())
                                                .width(Length::Shrink)
                                                .horizontal_alignment(HorizontalAlignment::Left)
                                                .vertical_alignment(VerticalAlignment::Center)
                                                .size(config.font_size)
                                                .color(config.theme.text_color())
                                        )
                                        .push(
                                            button(
                                                close_tab_state,
                                                "×",
                                                Message::CloseTab(pane, index),
                                                config.theme.button_style(
                                                    style::Button::CloseTab,
                                                ),
                                            )
                                            .width(Length::Shrink)
                                            .min_width(10)
                                        ),
                                )
                                .width(Length::Shrink)
                                .padding(1)
                                .on_press(Message::FocusTab((pane, index)))
                                .style(config.theme.tab_style(focused))
                            )
                        },
                    )
                )
                .push(Space::new(Length::Fill, Length::Fill))
                .push(options)
        )
            .width(Length::Fill)
            .max_height(config.tab_bar_height)
            .padding(0)
            .style(config.theme.tab_bar_style());

        let mut content = Column::new()
            .spacing(5)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(tab_bar);

        if let Some(tab_content) = tab_contents.get(*focused_tab) {
            content = content.push(tab_content.view(config));
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .center_y()
            .style(config.theme.pane_style(focus.is_some()))
            .into()
    }

    pub fn open_tab(&mut self, tab_type: TabType) {
        self.tabs.push((tab_type.clone().into(), button::State::new()));

        let tab = tab_content_from_type(tab_type);
        self.tab_contents.push(tab);
        self.focused_tab = self.tabs.len() - 1;
    }

    pub fn focus_tab(&mut self, index: usize) {
        self.focused_tab = index;
    }

    pub fn close_tab(&mut self, index: usize) -> usize {
        let current_focus = self.focused_tab;
        self.tabs.remove(index);
        self.tab_contents.remove(index);

        let mut new_focus = if current_focus > index {current_focus - 1} else {current_focus};
        while new_focus >= self.tabs.len() && new_focus >= 1 {
            new_focus -= 1;
        }
        new_focus
    }

    pub fn close_all_tabs(&mut self) {
        self.tabs.clear();
        self.tab_contents.clear();
    }
}
