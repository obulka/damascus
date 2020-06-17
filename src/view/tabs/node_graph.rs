// Standard Imports
use std::f32::consts::PI;

// 3rd Party Imports
use iced::{
    canvas::{Canvas, Cursor, Geometry, LineCap, Path, Stroke},
    Container, Element, Length, Point, Rectangle, Vector,
};

// Local Imports
pub mod node;

use crate::model::tabs::NodeGraph;
use crate::update::{
    tabs::{node_graph::NodeGraphMessage, TabContentMessage},
    Message,
};
use crate::view::{theme::NodeGraphStyle, CanvasView, Config, View};

impl View for NodeGraph {
    fn view(&mut self, _config: &Config) -> Element<Message> {
        let id = self.id.clone();
        let content = CanvasView::view(self)
            .map(move |message| TabContentMessage::NodeGraph((Some(id.clone()), message)).into());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(1)
            .into()
    }
}

impl CanvasView<NodeGraphMessage> for NodeGraph {
    fn view<'a>(&'a mut self) -> Element<'a, NodeGraphMessage> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut geometry: Vec<Geometry> = Vec::new();

        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);
        let node_graph_style: &NodeGraphStyle = &self.config.theme.into();

        if !self.lower_lod() && self.show_lines {
            let grid = self.grid_cache.draw(bounds.size(), |frame| {
                frame.translate(center);
                frame.scale(self.scaling);
                frame.translate(self.translation);
                frame.scale(self.grid_size);

                let region = self.visible_region(frame.size());
                let rows = region.rows();
                let columns = region.columns();
                let width = 1.0 / self.grid_size;
                let color = self.config.theme.secondary_color();

                frame.translate(Vector::new(-width / 2.0, -width / 2.0));

                for row in region.rows() {
                    let line = Path::line(
                        Point::new(*columns.start() as f32, row as f32),
                        Point::new(*columns.end() as f32, row as f32),
                    );
                    frame.stroke(
                        &line,
                        Stroke {
                            width: 1.0,
                            color: color,
                            ..Stroke::default()
                        },
                    );
                }

                for column in region.columns() {
                    let line = Path::line(
                        Point::new(column as f32, *rows.start() as f32),
                        Point::new(column as f32, *rows.end() as f32),
                    );
                    frame.stroke(
                        &line,
                        Stroke {
                            width: 1.0,
                            color: color,
                            ..Stroke::default()
                        },
                    );
                }
            });

            geometry.push(grid);
        }

        if !self.nodes.is_empty() {
            let disconnected = Path::line(Point::ORIGIN, Point::new(2.0, 0.0));

            let style: NodeGraphStyle = self.config.theme.into();
            let connection_stroke = Stroke {
                width: style.connection_width,
                color: style.connection_color,
                line_cap: LineCap::Butt,
                ..Stroke::default()
            };

            let connections = self.connection_cache.draw(bounds.size(), |frame| {
                frame.with_save(|frame| {
                    frame.translate(center);
                    frame.scale(self.scaling);
                    frame.translate(self.translation);
                    frame.scale(self.grid_size);
                    let width = 1.0 / self.grid_size;
                    frame.translate(Vector::new(-width / 2.0, -width / 2.0));

                    let _visible_bounds: Rectangle = self.visible_region(frame.size()).into();

                    for (_, node) in self.nodes.iter() {
                        let num_disconnected = node.num_disconnected();
                        if num_disconnected > 0 {
                            let rotation = -PI / 4.0;
                            let rotation_step = if num_disconnected > 1 {
                                -PI / (2.0 * (num_disconnected - 1) as f32)
                            } else {
                                0.0
                            };
                            let node_position = node.translated_rect().center();
                            let input_location = Vector::new(node_position.x, node_position.y);
                            frame.with_save(|frame| {
                                frame.translate(input_location);
                                frame.rotate(rotation);
                                for _ in 0..num_disconnected {
                                    frame.stroke(&disconnected, connection_stroke);
                                    frame.rotate(rotation_step);
                                }
                            })
                        }
                    }
                })
            });
            geometry.push(connections);

            let nodes = self.node_cache.draw(bounds.size(), |frame| {
                frame.with_save(|frame| {
                    frame.translate(center);
                    frame.scale(self.scaling);
                    frame.translate(self.translation);
                    frame.scale(self.grid_size);
                    let width = 1.0 / self.grid_size;
                    frame.translate(Vector::new(-width / 2.0, -width / 2.0));

                    let visible_bounds: Rectangle = self.visible_region(frame.size()).into();

                    for (_, node) in self.nodes.iter() {
                        node.draw(frame, &visible_bounds, !self.lower_lod(), &self.config);
                    }
                })
            });
            geometry.push(nodes);
        }

        if let Some(selection_box) = self.selection_box {
            let selection_box_geo = self.selection_box_cache.draw(bounds.size(), |frame| {
                frame.with_save(|frame| {
                    frame.translate(center);
                    frame.scale(self.scaling);
                    frame.translate(self.translation);
                    frame.scale(self.grid_size);
                    let width = 1.0 / self.grid_size;
                    frame.translate(Vector::new(-width / 2.0, -width / 2.0));
                    let selection_box_path =
                        Path::rectangle(selection_box.position(), selection_box.size());
                    frame.fill(&selection_box_path, (*node_graph_style).selection_box_color);
                    frame.stroke(
                        &selection_box_path,
                        Stroke {
                            width: (*node_graph_style).selection_box_border_width,
                            color: (*node_graph_style).selection_box_border_color,
                            ..Stroke::default()
                        },
                    );
                })
            });
            geometry.push(selection_box_geo);
        }
        geometry
    }
}
