// 3rd Party Imports

use iced::{
    button, pane_grid, Align, Button, Column, Container, Element, HorizontalAlignment, Length, Row,
    Space, Text, VerticalAlignment,
};

// Local Imports
use crate::action::{panel::Message, Message as DamascusMessage};
use crate::model::tabs::{tab_content_from_type, TabContent};
use crate::state::{
    style,
    widget::{Tab, TabType},
    Config,
};

pub struct State {
    split_horizontally: button::State,
    split_vertically: button::State,
    float_pane: button::State, // Not Implemented
    close: button::State,
    tabs: Vec<(String, button::State)>,
    tab_contents: Vec<Box<dyn TabContent>>,
    focused_tab: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
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
    ) -> Element<DamascusMessage> {
        let State {
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
            .push(
                Row::new()
                    .spacing(2)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .max_width(config.tab_bar_height)
                    .max_height(config.tab_bar_height)
                    .push(button(
                        split_vertically,
                        "|",
                        DamascusMessage::Split(pane_grid::Axis::Vertical, pane),
                        config.theme.button_style(style::Button::Primary),
                    ))
                    .push(button(
                        close,
                        "×",
                        DamascusMessage::Close(pane),
                        config.theme.button_style(style::Button::Destructive),
                    )),
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
                        DamascusMessage::FloatPane(pane),
                        config.theme.button_style(style::Button::Primary),
                    ))
                    .push(button(
                        split_horizontally,
                        "─",
                        DamascusMessage::Split(pane_grid::Axis::Horizontal, pane),
                        config.theme.button_style(style::Button::Primary),
                    )),
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
                        .push(Space::with_width(Length::Fill)), // Hack to get space before tab
                )
                .push(tabs.iter_mut().enumerate().fold(
                    Row::new().padding(0).spacing(0),
                    |row_of_tabs, (index, (tab_label, close_tab_state))| {
                        let focused = *focused_tab == index;
                        row_of_tabs.push(
                            Tab::new(
                                Row::new()
                                    .padding(7)
                                    .spacing(5)
                                    .max_width(150)
                                    .push(
                                        Text::new(tab_label.to_string())
                                            .width(Length::Shrink)
                                            .horizontal_alignment(HorizontalAlignment::Left)
                                            .vertical_alignment(VerticalAlignment::Center)
                                            .size(config.font_size)
                                            .color(config.theme.text_color()), // TODO: Move text into tab to get theme
                                    )
                                    .push(
                                        button(
                                            close_tab_state,
                                            "×",
                                            DamascusMessage::Panel(Message::CloseTab(pane, index)),
                                            config.theme.button_style(style::Button::CloseTab),
                                        )
                                        .width(Length::Shrink)
                                        .min_width(10),
                                    ),
                            )
                            .width(Length::Shrink)
                            .padding(1)
                            .on_press(DamascusMessage::Panel(Message::FocusTab((pane, index))))
                            .style(config.theme.tab_style(focused)),
                        )
                    },
                ))
                .push(Space::new(Length::Fill, Length::Fill))
                .push(options),
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

        if let Some(tab_content) = tab_contents.get_mut(*focused_tab) {
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
        self.tabs
            .push((tab_type.clone().into(), button::State::new()));
        self.tab_contents.push(tab_content_from_type(tab_type));

        self.focused_tab = self.tabs.len() - 1;
    }

    pub fn open_tab_with_content(
        &mut self,
        tab: (String, button::State),
        tab_content: Box<dyn TabContent>,
    ) {
        self.tabs.push(tab);
        self.tab_contents.push(tab_content);
        self.focused_tab = self.tabs.len() - 1;
    }

    pub fn focus_tab(&mut self, index: usize) {
        self.focused_tab = index;
    }

    pub fn close_tab(
        &mut self,
        index: usize,
    ) -> (usize, (String, button::State), Box<dyn TabContent>) {
        let current_focus = self.focused_tab;
        let tab = self.tabs.remove(index);
        let tab_content = self.tab_contents.remove(index);

        let mut new_focus = if current_focus > index {
            current_focus - 1
        } else {
            current_focus
        };
        while new_focus >= self.tabs.len() && new_focus >= 1 {
            new_focus -= 1;
        }
        (new_focus, tab, tab_content)
    }

    pub fn close_all_tabs(&mut self) {
        self.tabs.clear();
        self.tab_contents.clear();
    }

    pub fn index_of_tab_type(&self, tab_type: TabType) -> Option<usize> {
        let tab_string: String = tab_type.into();
        for (index, (label, _)) in self.tabs.iter().enumerate() {
            if *label == tab_string {
                return Some(index);
            }
        }
        None
    }

    pub fn get_focused_label(&self) -> Option<&String> {
        if let Some((focused_label, _)) = self.tabs.get(self.focused_tab) {
            return Some(focused_label);
        }
        None
    }

    pub fn get_focused_content(&self) -> Option<&Box<dyn TabContent>> {
        self.tab_contents.get(self.focused_tab)
    }

    pub fn get_mut_focused_content(&mut self) -> Option<&mut Box<dyn TabContent>> {
        self.tab_contents.get_mut(self.focused_tab)
    }
}
