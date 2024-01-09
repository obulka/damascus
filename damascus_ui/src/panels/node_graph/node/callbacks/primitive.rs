use egui_node_graph::NodeId;

use damascus_core::geometry;

use super::{DamascusGraph, DamascusValueType, NodeCallbacks};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct PrimitiveCallbacks;

impl NodeCallbacks for PrimitiveCallbacks {
    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String) {
        if input_name != "shape" {
            return;
        }
        if let Some(node) = graph.nodes.get(node_id) {
            let to_hide = vec![
                "radius",
                "radii",
                "height",
                "thickness",
                "hollow_radius",
                "hollow_height",
                "solid_angle",
                "width",
                "depth",
                "corner_radius",
                "base",
                "normal",
                "negative_height",
                "positive_height",
                "angle",
                "lower_radius",
                "upper_radius",
                "ring_radius",
                "tube_radius",
                "cap_angle",
                "radial_extent",
                "power",
                "iterations",
                "max_square_radius",
                "scale",
                "min_square_radius",
                "folding_limit",
            ];
            let mut to_show = vec![];
            if let Ok(input_id) = node.get_input(input_name) {
                if let Some(input_param) = graph.inputs.get(input_id) {
                    match input_param.value() {
                        DamascusValueType::ComboBox { ref value } => {
                            match value.as_enum::<geometry::Shapes>() {
                                Ok(geometry::Shapes::Sphere) => {
                                    to_show.push("radius");
                                }
                                Ok(geometry::Shapes::Ellipsoid) => {
                                    to_show.push("radii");
                                }
                                Ok(geometry::Shapes::CutSphere) => {
                                    to_show.push("radius");
                                    to_show.push("height");
                                }
                                Ok(geometry::Shapes::HollowSphere) => {
                                    to_show.push("radius");
                                    to_show.push("height");
                                    to_show.push("thickness");
                                }
                                Ok(geometry::Shapes::DeathStar) => {
                                    to_show.push("radius");
                                    to_show.push("hollow_radius");
                                    to_show.push("hollow_height");
                                }
                                Ok(geometry::Shapes::SolidAngle) => {
                                    to_show.push("radius");
                                    to_show.push("solid_angle");
                                }
                                Ok(geometry::Shapes::RectangularPrism) => {
                                    to_show.push("width");
                                    to_show.push("height");
                                    to_show.push("depth");
                                }
                                Ok(geometry::Shapes::RectangularPrismFrame) => {
                                    to_show.push("width");
                                    to_show.push("height");
                                    to_show.push("depth");
                                    to_show.push("thickness");
                                }
                                Ok(geometry::Shapes::Rhombus) => {
                                    to_show.push("width");
                                    to_show.push("height");
                                    to_show.push("depth");
                                    to_show.push("corner_radius");
                                }
                                Ok(geometry::Shapes::TriangularPrism) => {
                                    to_show.push("base");
                                    to_show.push("depth");
                                }
                                Ok(geometry::Shapes::Cylinder) => {
                                    to_show.push("radius");
                                    to_show.push("height");
                                }

                                Ok(geometry::Shapes::InfiniteCylinder) => {
                                    to_show.push("radius");
                                }
                                Ok(geometry::Shapes::Plane) => {
                                    to_show.push("normal");
                                }
                                Ok(geometry::Shapes::Capsule) => {
                                    to_show.push("radius");
                                    to_show.push("negative_height");
                                    to_show.push("positive_height");
                                }
                                Ok(geometry::Shapes::Cone) => {
                                    to_show.push("angle");
                                    to_show.push("height");
                                }
                                Ok(geometry::Shapes::InfiniteCone) => {
                                    to_show.push("angle");
                                }
                                Ok(geometry::Shapes::CappedCone)
                                | Ok(geometry::Shapes::RoundedCone) => {
                                    to_show.push("height");
                                    to_show.push("lower_radius");
                                    to_show.push("upper_radius");
                                }
                                Ok(geometry::Shapes::Torus) => {
                                    to_show.push("ring_radius");
                                    to_show.push("tube_radius");
                                }
                                Ok(geometry::Shapes::CappedTorus) => {
                                    to_show.push("ring_radius");
                                    to_show.push("tube_radius");
                                    to_show.push("cap_angle");
                                }
                                Ok(geometry::Shapes::Link) => {
                                    to_show.push("ring_radius");
                                    to_show.push("tube_radius");
                                    to_show.push("height");
                                }
                                Ok(geometry::Shapes::HexagonalPrism) => {
                                    to_show.push("height");
                                    to_show.push("depth");
                                }
                                Ok(geometry::Shapes::Octahedron) => {
                                    to_show.push("radial_extent");
                                }
                                Ok(geometry::Shapes::Mandelbulb) => {
                                    to_show.push("power");
                                    to_show.push("iterations");
                                    to_show.push("max_square_radius");
                                }
                                Ok(geometry::Shapes::Mandelbox) => {
                                    to_show.push("scale");
                                    to_show.push("iterations");
                                    to_show.push("min_square_radius");
                                    to_show.push("folding_limit");
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }

            for input_name in to_hide.iter() {
                if let Ok(input_id) = node.get_input(input_name) {
                    if let Some(input_param) = graph.inputs.get_mut(input_id) {
                        self.hide_input(&mut input_param.value)
                    }
                }
            }
            for input_name in to_show.iter() {
                if let Ok(input_id) = node.get_input(input_name) {
                    if let Some(input_param) = graph.inputs.get_mut(input_id) {
                        self.show_input(&mut input_param.value)
                    }
                }
            }
        }
    }
}
