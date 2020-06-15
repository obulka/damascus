// 3rd Party Imports
use iced::mouse;
use iced_native::{Clipboard, Event, Layout, Point};

// Local Imports
use crate::model::Tab;
use crate::view::renderer::tab::TabRenderer;
use crate::update::WidgetUpdate;

impl<'a, Message, Renderer> WidgetUpdate<Message, Renderer> for Tab<'a, Message, Renderer>
where
    Renderer: TabRenderer,
    Message: Clone,
{
    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>,
    ) {
        // Allows the close tab Tab to be pressed
        if let Some(child_layout) = layout.children().next() {
            self.content.on_event(
                event.clone(),
                child_layout,
                cursor_position,
                messages,
                renderer,
                clipboard,
            );
            // Do not focus if close Tab clicked
            if let Some(close_layout) = child_layout.children().last() {
                match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                        if let Some(on_press) = self.on_press.clone() {
                            if layout.bounds().contains(cursor_position)
                                && !close_layout.bounds().contains(cursor_position)
                            {
                                messages.push(on_press);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}