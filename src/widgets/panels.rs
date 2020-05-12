// use iced_native::{
//     Background,
//     Clipboard,
//     Color,
//     Element,
//     Event,
//     Hasher,
//     Layout,
//     layout,
//     Length,
//     mouse,
//     Point,
//     Size,
//     Widget,
// };
// use iced::{pane_grid::{Axis, Pane}, PaneGrid};
// use iced_wgpu::{Defaults, Primitive};


// use crate::action::Message;


// trait Panel
// {

//     fn on_event(
//         &mut self,
//         event: Event,
//         layout: Layout<'_>,
//         cursor_position: Point,
//         messages: &mut Vec<Message>,
//         renderer: &Renderer,
//         clipboard: Option<&dyn Clipboard>,
//     ) {
//         self.on_event(
//             event: Event,
//             layout,
//             cursor_position,
//             messages,
//             renderer,
//             clipboard,
//         );
//     }
// }

// impl<'a> Panel for PaneGrid<'a>
// {
//     // fn on_event(
//     //     &mut self,
//     //     event: Event,
//     //     layout: Layout<'_>,
//     //     cursor_position: Point,
//     //     messages: &mut Vec<Message>,
//     //     renderer: &Renderer,
//     //     clipboard: Option<&dyn Clipboard>,
//     // )
//     // {
//     //     super::on_event(self);
//     //     println!("fuck");
//     // }

// }


// pub struct Panels {
//     radius: u16,
// }

// impl Panels {
//     pub fn new(radius: u16) -> Self {
//         Self { radius }
//     }
// }

// impl<Message> Widget<Message, Renderer> for Panels {
//     fn width(&self) -> Length {
//         Length::Shrink
//     }

//     fn height(&self) -> Length {
//         Length::Shrink
//     }

//     fn layout(
//         &self,
//         _renderer: &Renderer,
//         _limits: &layout::Limits,
//     ) -> layout::Node {
//         layout::Node::new(Size::new(
//             f32::from(self.radius) * 2.0,
//             f32::from(self.radius) * 2.0,
//         ))
//     }

//     fn hash_layout(&self, state: &mut Hasher) {
//         use std::hash::Hash;

//         self.radius.hash(state);
//     }

//     fn draw(
//         &self,
//         _renderer: &mut Renderer,
//         _defaults: &Defaults,
//         layout: Layout<'_>,
//         _cursor_position: Point,
//     ) -> (Primitive, mouse::Interaction) {
//         (
//             Primitive::Quad {
//                 bounds: layout.bounds(),
//                 background: Background::Color(Color::BLACK),
//                 border_radius: self.radius,
//                 border_width: 0,
//                 border_color: Color::TRANSPARENT,
//             },
//             mouse::Interaction::default(),
//         )
//     }
// }

// impl<'a, Message> Into<Element<'a, Message, Renderer>> for Panels {
//     fn into(self) -> Element<'a, Message, Renderer> {
//         Element::new(self)
//     }
// }