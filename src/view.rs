use iced::{Align, Column, Container, Element, Length, PaneGrid, Row, canvas::{Cursor, Geometry},
    mouse, Rectangle,
};

pub mod renderer;
pub mod style;
pub mod widget;

pub use widget::*;
use crate::model::Config;
use crate::view::widget::panel::PanelView;
use crate::update::{handle_hotkey, BaseMessage};
use crate::Damascus;

pub trait View {
    fn view(&mut self, config: &Config) -> Element<BaseMessage>;
}

pub trait CanvasView: View {
    type Message;

    fn view<'a>(&'a mut self) -> Element<'a, Self::Message>;

    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>;
}

impl View for Damascus {
    fn view(&mut self, config: &Config) -> Element<BaseMessage> {
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
                .on_drag(BaseMessage::PaneDragged)
                .on_resize(10, BaseMessage::Resized)
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
