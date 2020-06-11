use iced::{
    canvas::{Frame, Path, Text},
    Color, HorizontalAlignment, Point, Rectangle, Vector, VerticalAlignment,
};

mod viewer;
pub use viewer::Viewer;

pub trait Node {
    fn label(&self) -> &str;

    fn color(&self) -> Color;

    fn text_color(&self) -> Color;

    fn text_size(&self) -> f32;

    fn translate(&mut self, translation: Vector);

    fn get_position(&self) -> Point {
        self.rect().position()
    }

    fn rect(&self) -> Rectangle;

    fn draw(&self, frame: &mut Frame, bounds: &Rectangle, draw_text: bool) {
        if let Some(rect) = bounds.intersection(&self.rect()) {
            let node = Path::rectangle(rect.position(), rect.size());
            frame.with_save(|frame| {
                frame.translate(self.get_position() - Point::ORIGIN);
                frame.fill(&node, self.color());

                if draw_text && bounds.contains(self.rect().center()) {
                    frame.translate(Vector::new(rect.center_x(), rect.center_y()));
                    frame.fill_text(Text {
                        content: (*self.label()).to_string(),
                        color: self.text_color(),
                        size: self.text_size(),
                        horizontal_alignment: HorizontalAlignment::Center,
                        vertical_alignment: VerticalAlignment::Center,
                        ..Text::default()
                    })
                }
            });
        }
    }
}
