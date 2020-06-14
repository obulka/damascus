use iced::{
    canvas::{Canvas, Cursor, Geometry, Path, Stroke},
    Container, Element, Length, Point, Rectangle, Vector,
};
// Security not important

use crate::model::{tabs::NodeGraph, Config};
use crate::update::{tabs::node_graph::NodeGraphMessage, Message};
use crate::view::{style::NodeGraphStyle, CanvasView, View};

impl View for NodeGraph {
    fn view(&mut self, _config: &Config) -> Element<Message> {
        let content = CanvasView::view(self).map(|message| message.into());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(1)
            .into()
    }
}

impl CanvasView for NodeGraph {
    type Message = NodeGraphMessage;
    fn view<'a>(&'a mut self) -> Element<'a, Self::Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut geometry: Vec<Geometry> = Vec::new();

        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);
        let lower_lod = self.scaling < 0.6;
        let node_graph_style: &NodeGraphStyle = &self.config.theme.into();

        if !lower_lod && self.show_lines {
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
            let nodes = self.node_cache.draw(bounds.size(), |frame| {
                frame.with_save(|frame| {
                    frame.translate(center);
                    frame.scale(self.scaling);
                    frame.translate(self.translation);
                    frame.scale(self.grid_size);
                    let width = 1.0 / self.grid_size;
                    frame.translate(Vector::new(-width / 2.0, -width / 2.0));

                    let visible_bounds: Rectangle = self.visible_region(frame.size()).into();

                    let font_size = self.config.font_size as f32;
                    let node_graph_style = &self.config.theme.into();
                    for (label, node) in self.nodes.iter() {
                        node.draw(
                            frame,
                            &visible_bounds,
                            if lower_lod { None } else { Some(label) },
                            self.selected_nodes.contains(label),
                            false,
                            font_size,
                            node_graph_style,
                        );
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