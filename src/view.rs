use iced::{
    canvas::{Cursor, Frame, Geometry},
    Align, Column, Container, Element, Length, PaneGrid, Rectangle, Row,
};

pub mod panel;
pub mod tabs;
pub mod theme;
mod widget;

pub use theme::Theme;
pub use widget::*;
pub use tabs::node_graph::node;

use crate::update::{handle_hotkey, Message};
use crate::Damascus;
use panel::PanelView;

pub trait View {
    fn view(&mut self, config: &Config) -> Element<Message>;
}

pub trait CanvasView: View {
    type Message;

    fn view<'a>(&'a mut self) -> Element<'a, Self::Message>;

    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>;
}

pub trait CanvasItemView {
    fn draw(
        &self,
        frame: &mut Frame,
        bounds: &Rectangle,
    );
}

#[derive(Debug, Clone)]
pub struct Config {
    pub font_size: u16,
    pub tab_bar_height: u32,
    pub theme: Theme,
    pub title: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            font_size: 13,
            tab_bar_height: 36,
            theme: Theme::default(),
            title: "Damascus".to_string(),
        }
    }
}

impl View for Damascus {
    fn view(&mut self, config: &Config) -> Element<Message> {
        let app_content = Column::new()
            .push(
                // Toolbar
                Row::new()
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .max_height(config.tab_bar_height)
                    .align_items(Align::End)
                    .spacing(1)
                    .padding(0),
            )
            .push(
                // Panes
                PaneGrid::new(&mut self.panes, |pane, content, focus| {
                    PanelView::view(content, pane, focus, config)
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(0) // Space between panes
                .on_drag(Message::PaneDragged)
                .on_resize(10, Message::Resized)
                .on_key_press(handle_hotkey),
            );

        Container::new(app_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0) // Space between panes and window edge
            .style(config.theme)
            .into()
    }
}
