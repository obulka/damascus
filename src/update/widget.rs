use iced_native::{Clipboard, Event as NativeEvent, Layout, Point};

pub mod tab;

pub trait WidgetUpdate<EmittedMessage, Renderer> {
    fn on_event(
        &mut self,
        event: NativeEvent,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<EmittedMessage>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>,
    );
}
