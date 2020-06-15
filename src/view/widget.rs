// 3rd Party Imports
use iced_native::{layout, Layout, Point};

pub mod renderer;
pub mod style;

pub trait WidgetView<Message, Renderer: iced_native::Renderer + Sized> {
    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node;

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output;
}
