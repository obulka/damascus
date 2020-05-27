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
use std::collections::HashMap;

// Local Imports
use crate::action::Message;
use crate::state::Config;
use crate::state::style::{self, Theme};


pub struct Panel {
    split_horizontally: button::State,
    split_vertically: button::State,
    float_pane: button::State, // Not Implemented
    close: button::State,
    tabs: HashMap<String, button::State>,
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
            tabs: HashMap::new(),
        }
    }

    pub fn view(
        &mut self,
        pane: pane_grid::Pane,
        focus: Option<pane_grid::Focus>,
        _total_panes: usize,
        theme: Theme,
        config: &Config,
    ) -> Element<Message> {
        let Panel {
            split_horizontally,
            split_vertically,
            float_pane,
            close,
            tabs,
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
                .width(Length::Shrink)
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
                    "X",
                    Message::Close(pane),
                    theme.button_style(
                        style::Button::Destructive,
                    ),
                ))
            )
            .push(
                Row::new()
                    .spacing(2)
                    .width(Length::Shrink)
                    .height(Length::Fill)
                    .max_width(config.tab_bar_height)
                    .max_height(config.tab_bar_height)
                    .push(button(
                        float_pane,
                        "+",
                        // Message::ThemeChanged(
                        //     match theme {
                        //         Theme::Dark => Theme::Light,
                        //         Theme::Light => Theme::Dark,
                        //     }
                        // ),
                        Message::AddTabFocused("Fook".to_string()),
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
                .spacing(5)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(Align::Center)
                .padding(0)
                .push(
                    tabs.iter_mut().fold(
                        Row::new().padding(0).spacing(1),
                        |row_of_tabs, tab| {
                            row_of_tabs.push(
                                Button::new(
                                    tab.1,
                                    Text::new(tab.0)
                                        .width(Length::Shrink)
                                        .horizontal_alignment(HorizontalAlignment::Left)
                                        .vertical_alignment(VerticalAlignment::Center)
                                        .size(10),
                                )
                                .width(Length::Shrink)
                                .padding(1)
                                .on_press(Message::ThemeChanged(
                                    match theme {
                                        Theme::Dark => Theme::Light,
                                        Theme::Light => Theme::Dark,
                                    }
                                ))
                                .style(theme.button_style(
                                    style::Button::Primary,
                                ))
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

    pub fn add_tab(&mut self, label: String) {
        self.tabs.insert(label, button::State::new());
    }
}
