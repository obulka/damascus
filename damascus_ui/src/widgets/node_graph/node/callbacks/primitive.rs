// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use egui_node_graph::NodeId;

use damascus::geometry::{self, primitives};

use super::{
    super::{Graph, NodeGraphResponse},
    NodeCallbacks, NodeGraph, NodeValueType, UIInput,
};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct PrimitiveCallbacks;

impl NodeCallbacks for PrimitiveCallbacks {
    fn input_value_changed(
        &self,
        node_graph: &mut NodeGraph,
        node_id: NodeId,
        input_name: &String,
    ) -> Vec<NodeGraphResponse> {
        if ![
            "bounding_volume",
            "shape",
            "repetition",
            "hollow",
            "elongate",
        ]
        .contains(&input_name.as_str())
        {
            return Vec::new();
        }
        let graph: &mut Graph = node_graph.graph_mut();
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
                                NodeValueType::ComboBox { value } => {
                                    match value.value().as_enumerator() {
                                        primitives::Shapes::CappedCone
                                        | primitives::Shapes::RoundedCone => {
                                            to_show.push("height");
                                            to_show.push("lower_radius");
                                            to_show.push("upper_radius");
                                        }
                                        primitives::Shapes::CappedTorus => {
                                            to_show.push("ring_radius");
                                            to_show.push("tube_radius");
                                            to_show.push("cap_angle");
                                        }
                                        primitives::Shapes::Capsule => {
                                            to_show.push("radius");
                                            to_show.push("negative_height");
                                            to_show.push("positive_height");
                                        }
                                        primitives::Shapes::Cone => {
                                            to_show.push("angle");
                                            to_show.push("height");
                                        }
                                        primitives::Shapes::CutSphere => {
                                            to_show.push("radius");
                                            to_show.push("height");
                                        }
                                        primitives::Shapes::Cylinder => {
                                            to_show.push("radius");
                                            to_show.push("height");
                                        }
                                        primitives::Shapes::DeathStar => {
                                            to_show.push("radius");
                                            to_show.push("hollow_radius");
                                            to_show.push("hollow_height");
                                        }
                                        primitives::Shapes::Ellipsoid => {
                                            to_show.push("radii");
                                        }
                                        primitives::Shapes::HexagonalPrism => {
                                            to_show.push("height");
                                            to_show.push("depth");
                                        }
                                        primitives::Shapes::HollowSphere => {
                                            to_show.push("radius");
                                            to_show.push("height");
                                            to_show.push("thickness");
                                        }
                                        primitives::Shapes::InfiniteCone => {
                                            to_show.push("angle");
                                        }
                                        primitives::Shapes::InfiniteCylinder => {
                                            to_show.push("radius");
                                        }
                                        primitives::Shapes::Link => {
                                            to_show.push("ring_radius");
                                            to_show.push("tube_radius");
                                            to_show.push("height");
                                        }
                                        primitives::Shapes::Mandelbox => {
                                            to_show.push("scale");
                                            to_show.push("iterations");
                                            to_show.push("min_square_radius");
                                            to_show.push("folding_limit");
                                        }
                                        primitives::Shapes::Mandelbulb => {
                                            to_show.push("power");
                                            to_show.push("iterations");
                                            to_show.push("max_square_radius");
                                        }
                                        primitives::Shapes::Octahedron => {
                                            to_show.push("radial_extent");
                                        }
                                        primitives::Shapes::Plane => {
                                            to_show.push("normal");
                                        }
                                        primitives::Shapes::RectangularPrism => {
                                            to_show.push("width");
                                            to_show.push("height");
                                            to_show.push("depth");
                                        }
                                        primitives::Shapes::RectangularPrismFrame => {
                                            to_show.push("width");
                                            to_show.push("height");
                                            to_show.push("depth");
                                            to_show.push("thickness");
                                        }
                                        primitives::Shapes::Rhombus => {
                                            to_show.push("width");
                                            to_show.push("height");
                                            to_show.push("depth");
                                            to_show.push("corner_radius");
                                        }
                                        primitives::Shapes::SolidAngle => {
                                            to_show.push("radius");
                                            to_show.push("solid_angle");
                                        }
                                        primitives::Shapes::Sphere => {
                                            to_show.push("radius");
                                        }
                                        primitives::Shapes::Torus => {
                                            to_show.push("ring_radius");
                                            to_show.push("tube_radius");
                                        }
                                        primitives::Shapes::TriangularPrism => {
                                            to_show.push("base");
                                            to_show.push("depth");
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        "repetition" => match input_param.value() {
                            NodeValueType::ComboBox { value } => {
                                match value.value().as_enumerator() {
                                    geometry::Repetition::Finite => {
                                        to_show.push("negative_repetitions");
                                        to_show.push("positive_repetitions");
                                        to_show.push("spacing");
                                    }
                                    geometry::Repetition::Infinite => {
                                        to_hide.push("negative_repetitions");
                                        to_hide.push("positive_repetitions");
                                        to_show.push("spacing");
                                    }
                                    _ => {
                                        to_hide.push("negative_repetitions");
                                        to_hide.push("positive_repetitions");
                                        to_hide.push("spacing");
                                    }
                                }
                            }
                            _ => {}
                        },
                        "hollow" => match input_param.value() {
                            NodeValueType::Bool { ref value } => {
                                if *value.value() {
                                    to_show.push("wall_thickness");
                                } else {
                                    to_hide.push("wall_thickness");
                                }
                            }
                            _ => {}
                        },
                        "elongate" => match input_param.value() {
                            NodeValueType::Bool { ref value } => {
                                if *value.value() {
                                    to_show.push("elongation");
                                } else {
                                    to_hide.push("elongation");
                                }
                            }
                            _ => {}
                        },
                        "bounding_volume" => match input_param.value() {
                            NodeValueType::Bool { ref value } => {
                                if *value.value() {
                                    to_hide.push("blend_type");
                                    to_hide.push("blend_strength");
                                } else {
                                    to_show.push("blend_type");
                                    to_show.push("blend_strength");
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
        Vec::new()
    }
}
