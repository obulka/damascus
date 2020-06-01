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
    widget::tab::{Tab},
};
use crate::state::style::{self, Theme};


pub struct Panel {
    split_horizontally: button::State,
    split_vertically: button::State,
    float_pane: button::State, // Not Implemented
    close: button::State,
    tabs: Vec<(String, button::State)>,
    focused_tab: usize,
}

// Next step - ability to add tabs as buttons - then close them - then swap between panels
// they need to then send the message to open a specific Canvas/ UI Element

impl Panel {
    pub fn new() -> Self {
        Panel {
            split_horizontally: button::State::new(),
            split_vertically: button::State::new(),
            float_pane: button::State::new(),
            close: button::State::new(),
            tabs: Vec::new(),
            focused_tab: 0,
        }
    }

    pub fn view(
        &mut self,
        pane: pane_grid::Pane,
        focus: Option<pane_grid::Focus>,
        total_panes: usize,
        theme: Theme,
        config: &Config,
    ) -> Element<Message> {
        let Panel {
            split_horizontally,
            split_vertically,
            float_pane,
            close,
            tabs,
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
                    theme.button_style(
                        style::Button::Primary,
                    ),
                ))
                .push(button(
                    close,
                    "×",
                    Message::Close(pane),
                    theme.button_style(
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
                        Message::OpenTabFocused(format!("Fook{}", total_panes)),
                        theme.button_style(
                            style::Button::Primary,
                        ),
                    ))
                    .push(button(
                        split_horizontally,
                        "─",
                        Message::Split(pane_grid::Axis::Horizontal, pane),
                        theme.button_style(
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
                            row_of_tabs.push(
                                Tab::new(
                                    Row::new().padding(7).spacing(5).max_width(150)
                                        .push(
                                            Text::new(tab_label.to_string())
                                                .width(Length::Shrink)
                                                .horizontal_alignment(HorizontalAlignment::Left)
                                                .vertical_alignment(VerticalAlignment::Center)
                                                .size(config.font_size)
                                                .color(theme.text_color())
                                        )
                                        .push(
                                            button(
                                                close_tab_state,
                                                "×",
                                                Message::CloseTab(pane, index),
                                                theme.button_style(
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
                                .style(theme.tab_style(*focused_tab == index))
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
            .style(theme.tab_bar_style());

        let content = Column::new()
            .spacing(5)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(tab_bar);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .center_y()
            .style(theme.pane_style(focus.is_some()))
            .into()
    }

    pub fn open_tab(&mut self, label: &String) {
        self.tabs.push((label.to_string(), button::State::new()));
        self.focused_tab = self.tabs.len() - 1;
    }

    pub fn focus_tab(&mut self, index: usize) {
        self.focused_tab = index;
    }

    pub fn close_tab(&mut self, index: usize) -> usize {
        let current_focus = self.focused_tab;
        self.tabs.remove(index);

        let mut new_focus = if current_focus > index {current_focus - 1} else {current_focus};
        while new_focus >= self.tabs.len() && new_focus >= 1 {
            new_focus -= 1;
        }
        new_focus
    }

    pub fn close_all_tabs(&mut self) {
        self.tabs.clear();
    }
}
