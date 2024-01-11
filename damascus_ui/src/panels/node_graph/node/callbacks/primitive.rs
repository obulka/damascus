use egui_node_graph::NodeId;

use damascus_core::geometry;

use super::{DamascusGraph, DamascusValueType, NodeCallbacks, UIInput};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct PrimitiveCallbacks;

impl NodeCallbacks for PrimitiveCallbacks {
    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String) {
        if !["shape", "repetition", "blend_type", "hollow", "elongate"]
            .contains(&input_name.as_str())
        {
            return;
        }
        if let Some(node) = graph.nodes.get(node_id) {
            let mut to_hide = vec![];
            let mut to_show = vec![];
            if let Ok(input_id) = node.get_input(input_name) {
                if let Some(input_param) = graph.inputs.get(input_id) {
                    match input_name.as_str() {
                        "shape" => {
                            to_hide.extend([
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
                            ]);
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
                        "repetition" => match input_param.value() {
                            DamascusValueType::ComboBox { ref value } => {
                                match value.as_enum::<geometry::Repetition>() {
                                    Ok(geometry::Repetition::Finite) => {
                                        to_show.push("repetitions");
                                        to_show.push("spacing");
                                    }
                                    Ok(geometry::Repetition::Infinite) => {
                                        to_hide.push("repetitions");
                                        to_show.push("spacing");
                                    }
                                    _ => {
                                        to_hide.push("repetitions");
                                        to_hide.push("spacing");
                                    }
                                }
                            }
                            _ => {}
                        },
                        "blend_type" => match input_param.value() {
                            DamascusValueType::ComboBox { ref value } => {
                                if let Ok(blend_type) = value.as_enum::<geometry::BlendType>() {
                                    if blend_type >= geometry::BlendType::SmoothUnion {
                                        to_show.push("blend_strength");
                                    } else {
                                        to_hide.push("blend_strength");
                                    }
                                }
                            }
                            _ => {}
                        },
                        "hollow" => match input_param.value() {
                            DamascusValueType::Bool { ref value } => {
                                if *value.value() {
                                    to_show.push("wall_thickness");
                                } else {
                                    to_hide.push("wall_thickness");
                                }
                            }
                            _ => {}
                        },
                        "elongate" => match input_param.value() {
                            DamascusValueType::Bool { ref value } => {
                                if *value.value() {
                                    to_show.push("elongation");
                                } else {
                                    to_hide.push("elongation");
                                }
                            }
                            _ => {}
                        },
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
