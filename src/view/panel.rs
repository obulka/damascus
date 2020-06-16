// 3rd Party Imports
use iced::{
    pane_grid, Align, Button, Column, Container, Element, HorizontalAlignment, Length, Row, Space,
    Text, VerticalAlignment,
};

// Local Imports
use crate::model::{panel::Panel, Tab};
use crate::update::{
    panel::{PanelMessage, PanelUpdate},
    Message,
};
use crate::view::{theme, Config, View};

pub trait PanelView: View {
    fn view(
        &mut self,
        pane: pane_grid::Pane,
        focus: Option<pane_grid::Focus>,
        config: &Config,
    ) -> Element<Message>;
}

impl PanelView for Panel {
    fn view(
        &mut self,
        pane: pane_grid::Pane,
        focus: Option<pane_grid::Focus>,
        config: &Config,
    ) -> Element<Message> {
        self.update_view_state(pane, focus);
        View::view(self, config)
    }
}

impl View for Panel {
    fn view(&mut self, config: &Config) -> Element<Message> {
        let Panel {
            pane,
            focus,
            split_horizontally,
            split_vertically,
            float_pane,
            close,
            tabs,
            tab_contents,
            focused_tab,
        } = self;
        let pane = pane.unwrap();

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
                        Message::Split(pane_grid::Axis::Vertical, pane),
                        config.theme.button_style(theme::Button::Primary),
                    ))
                    .push(button(
                        close,
                        "×",
                        Message::Close(pane),
                        config.theme.button_style(theme::Button::Destructive),
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
                        Message::FloatPane(pane),
                        config.theme.button_style(theme::Button::Primary),
                    ))
                    .push(button(
                        split_horizontally,
                        "─",
                        Message::Split(pane_grid::Axis::Horizontal, pane),
                        config.theme.button_style(theme::Button::Primary),
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
                        .push(Space::with_width(Length::Fill)), // Add space before tab
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
                                            .color(
                                                config.theme.tab_style(focused).style().text_color,
                                            ),
                                    )
                                    .push(
                                        button(
                                            close_tab_state,
                                            "×",
                                            PanelMessage::CloseTab(pane, index).into(),
                                            config.theme.button_style(theme::Button::CloseTab),
                                        )
                                        .width(Length::Shrink)
                                        .min_width(10),
                                    ),
                            )
                            .width(Length::Shrink)
                            .padding(1)
                            .on_press(PanelMessage::FocusTab((pane, index)).into())
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
            .style(config.theme.pane_style(*focus))
            .into()
    }
}
