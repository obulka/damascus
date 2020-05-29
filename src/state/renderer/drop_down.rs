use iced_wgpu::{
    Defaults,
    defaults,
    Primitive,
};
use iced_native::{
    Background,
    Color,
    Element,
    Layout,
    mouse,
    Point,
    Rectangle,
    Vector,
};
use crate::state::{
    widget::tab::Renderer,
    style::drop_down::StyleSheet,
};

impl Renderer for iced_wgpu::Renderer {
    const DEFAULT_PADDING: u16 = 5;

    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        defaults: &Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        is_disabled: bool,
        is_pressed: bool,
        is_open: bool,
        style: &Box<dyn StyleSheet>,
        content: &Element<'_, Message, Self>,
        menu: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        let bounds = layout.bounds();
        let is_mouse_over = bounds.contains(cursor_position);

        let styling = if is_disabled {
            style.disabled()
        } else if is_mouse_over {
            if is_pressed {
                style.pressed()
            } else {
                style.hovered()
            }
        } else {
            style.active()
        };

        let (content, _) = content.draw(
            self,
            &Defaults {
                text: defaults::Text {
                    color: styling.text_color,
                },
            },
            content_layout,
            cursor_position,
        );

        let mut mouse_interaction = mouse::Interaction::default();
        if is_mouse_over {
            mouse_interaction = mouse::Interaction::Pointer
        }

        if styling.background.is_some() || styling.border_width > 0 {
            let mut widget_primitives = vec![content];
            let background = Primitive::Quad {
                bounds,
                background: styling
                    .background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
                border_radius: styling.border_radius,
                border_width: styling.border_width,
                border_color: styling.border_color,
            };

            if styling.shadow_offset == Vector::default() {
                widget_primitives.insert(0, background);
            } else {
                // TODO: Implement proper shadow support
                let shadow = Primitive::Quad {
                    bounds: Rectangle {
                        x: bounds.x + styling.shadow_offset.x,
                        y: bounds.y + styling.shadow_offset.y,
                        ..bounds
                    },
                    background: Background::Color(
                        [0.0, 0.0, 0.0, 0.5].into(),
                    ),
                    border_radius: styling.border_radius,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                };

                widget_primitives.insert(0, background);
                widget_primitives.insert(0, shadow);
            }
            if is_open {
                let (primitive, new_mouse_interaction) =
                    menu.draw(self, defaults, layout, cursor_position);
                widget_primitives.insert(0, primitive);
                if new_mouse_interaction > mouse_interaction {
                    mouse_interaction = new_mouse_interaction;
                }
            }
            (Primitive::Group{primitives: widget_primitives}, mouse_interaction)
        } else {
            (content, mouse_interaction)
        }
    }
}
