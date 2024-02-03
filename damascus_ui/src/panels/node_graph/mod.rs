use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use egui_node_graph::{Graph, NodeId, OutputId};
use glam::Vec4Swizzles;
use strum::IntoEnumIterator;

use damascus_core::{geometry, lights, materials, renderers, scene};

mod data_type;
mod node;
mod node_graph_state;
mod response;
mod value_type;

pub use data_type::DamascusDataType;
pub use node::{AllDamascusNodeTemplates, DamascusNodeData, DamascusNodeTemplate, NodeCallbacks};
pub use node_graph_state::DamascusGraphState;
pub use response::DamascusResponse;
pub use value_type::{Bool, DamascusValueType, Mat4, UIInput};

pub type DamascusGraph = Graph<DamascusNodeData, DamascusDataType, DamascusValueType>;
type OutputsCache = HashMap<OutputId, DamascusValueType>;

/// Recursively evaluates all dependencies of this node, then evaluates the node itself.
pub fn evaluate_node(
    graph: &DamascusGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<DamascusValueType> {
    // To solve a similar problem as creating node types above, we define an
    // Evaluator as a convenience. It may be overkill for this small example,
    // but something like this makes the code much more readable when the
    // number of nodes starts growing.

    struct Evaluator<'a> {
        graph: &'a DamascusGraph,
        outputs_cache: &'a mut OutputsCache,
        node_id: NodeId,
    }
    impl<'a> Evaluator<'a> {
        fn new(
            graph: &'a DamascusGraph,
            outputs_cache: &'a mut OutputsCache,
            node_id: NodeId,
        ) -> Self {
            Self {
                graph,
                outputs_cache,
                node_id,
            }
        }

        fn evaluate_input(&mut self, name: &str) -> anyhow::Result<DamascusValueType> {
            // Calling `evaluate_input` recursively evaluates other nodes in the
            // graph until the input value for a paramater has been computed.
            evaluate_input(self.graph, self.node_id, name, self.outputs_cache)
        }

        fn populate_output(
            &mut self,
            name: &str,
            value: DamascusValueType,
        ) -> anyhow::Result<DamascusValueType> {
            // After computing an output, we don't just return it, but we also
            // populate the outputs cache with it. This ensures the evaluation
            // only ever computes an output once.
            //
            // The return value of the function is the "final" output of the
            // node, the thing we want to get from the evaluation. The example
            // would be slightly more contrived when we had multiple output
            // values, as we would need to choose which of the outputs is the
            // one we want to return. Other outputs could be used as
            // intermediate values.
            //
            // Note that this is just one possible semantic interpretation of
            // the graphs, you can come up with your own evaluation semantics!
            populate_output(self.graph, self.outputs_cache, self.node_id, name, value)
        }

        fn input_bool(&mut self, name: &str) -> anyhow::Result<bool> {
            self.evaluate_input(name)?.try_to_bool()
        }

        fn input_bool_vector3(&mut self, name: &str) -> anyhow::Result<glam::BVec3> {
            self.evaluate_input(name)?.try_to_bvec3()
        }

        fn input_combo_box<E: IntoEnumIterator + Display + FromStr>(
            &mut self,
            name: &str,
        ) -> anyhow::Result<E> {
            self.evaluate_input(name)?.try_to_enum::<E>()
        }

        fn input_int(&mut self, name: &str) -> anyhow::Result<i32> {
            self.evaluate_input(name)?.try_to_int()
        }

        fn input_uint(&mut self, name: &str) -> anyhow::Result<u32> {
            self.evaluate_input(name)?.try_to_uint()
        }

        fn input_uint_vector3(&mut self, name: &str) -> anyhow::Result<glam::UVec3> {
            self.evaluate_input(name)?.try_to_uvec3()
        }

        fn input_float(&mut self, name: &str) -> anyhow::Result<f32> {
            self.evaluate_input(name)?.try_to_float()
        }

        fn input_vector2(&mut self, name: &str) -> anyhow::Result<glam::Vec2> {
            self.evaluate_input(name)?.try_to_vec2()
        }

        fn input_vector3(&mut self, name: &str) -> anyhow::Result<glam::Vec3> {
            self.evaluate_input(name)?.try_to_vec3()
        }

        fn input_vector4(&mut self, name: &str) -> anyhow::Result<glam::Vec4> {
            self.evaluate_input(name)?.try_to_vec4()
        }

        fn input_matrix3(&mut self, name: &str) -> anyhow::Result<glam::Mat3> {
            self.evaluate_input(name)?.try_to_mat3()
        }

        fn input_matrix4(&mut self, name: &str) -> anyhow::Result<glam::Mat4> {
            self.evaluate_input(name)?.try_to_mat4()
        }

        fn output_matrix4(
            &mut self,
            name: &str,
            value: glam::Mat4,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(
                name,
                DamascusValueType::Mat4 {
                    value: Mat4::new(value),
                },
            )
        }

        fn input_image(&mut self, name: &str) -> anyhow::Result<ndarray::Array4<f32>> {
            self.evaluate_input(name)?.try_to_image()
        }

        fn output_image(
            &mut self,
            name: &str,
            value: ndarray::Array4<f32>,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Image { value })
        }

        fn input_camera(&mut self, name: &str) -> anyhow::Result<geometry::camera::Camera> {
            self.evaluate_input(name)?.try_to_camera()
        }

        fn output_camera(
            &mut self,
            name: &str,
            value: geometry::camera::Camera,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Camera { value })
        }

        fn input_light(&mut self, name: &str) -> anyhow::Result<Vec<lights::Light>> {
            self.evaluate_input(name)?.try_to_light()
        }

        fn output_light(
            &mut self,
            name: &str,
            value: Vec<lights::Light>,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Light { value })
        }

        fn input_material(&mut self, name: &str) -> anyhow::Result<materials::Material> {
            self.evaluate_input(name)?.try_to_material()
        }

        fn output_material(
            &mut self,
            name: &str,
            value: materials::Material,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Material { value })
        }

        fn input_primitive(&mut self, name: &str) -> anyhow::Result<Vec<geometry::Primitive>> {
            self.evaluate_input(name)?.try_to_primitive()
        }

        fn output_primitive(
            &mut self,
            name: &str,
            value: Vec<geometry::Primitive>,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Primitive { value })
        }

        fn input_procedural_texture(
            &mut self,
            name: &str,
        ) -> anyhow::Result<materials::ProceduralTexture> {
            self.evaluate_input(name)?.try_to_procedural_texture()
        }

        fn output_procedural_texture(
            &mut self,
            name: &str,
            value: materials::ProceduralTexture,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::ProceduralTexture { value })
        }

        fn input_ray_marcher(&mut self, name: &str) -> anyhow::Result<renderers::RayMarcher> {
            self.evaluate_input(name)?.try_to_ray_marcher()
        }

        fn output_ray_marcher(
            &mut self,
            name: &str,
            value: renderers::RayMarcher,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::RayMarcher { value })
        }

        fn input_scene(&mut self, name: &str) -> anyhow::Result<scene::Scene> {
            self.evaluate_input(name)?.try_to_scene()
        }

        fn output_scene(
            &mut self,
            name: &str,
            value: scene::Scene,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Scene { value })
        }
    }

    let node = &graph[node_id];
    let mut evaluator = Evaluator::new(graph, outputs_cache, node_id);
    match node.user_data.template {
        DamascusNodeTemplate::Axis => {
            let input_axis = evaluator.input_matrix4("axis")?;
            let translate = evaluator.input_vector3("translate")?;
            let rotate = evaluator.input_vector3("rotate")? * std::f32::consts::PI / 180.0;
            let uniform_scale = evaluator.input_float("uniform_scale")?;

            let quaternion =
                glam::Quat::from_euler(glam::EulerRot::XYZ, rotate.x, rotate.y, rotate.z);

            evaluator.output_matrix4(
                "out",
                input_axis
                    * glam::Mat4::from_scale_rotation_translation(
                        glam::Vec3::splat(uniform_scale),
                        quaternion,
                        translate,
                    ),
            )
        }
        DamascusNodeTemplate::Camera => {
            let focal_length = evaluator.input_float("focal_length")?;
            let horizontal_aperture = evaluator.input_float("horizontal_aperture")?;
            let near_plane = evaluator.input_float("near_plane")?;
            let far_plane = evaluator.input_float("far_plane")?;
            let focal_distance = evaluator.input_float("focal_distance")?;
            let f_stop = evaluator.input_float("f_stop")?;
            let world_matrix = evaluator.input_matrix4("world_matrix")?;
            let enable_depth_of_field = evaluator.input_bool("enable_depth_of_field")?;
            evaluator.output_camera(
                "out",
                geometry::camera::Camera::new(
                    1., // TODO use the root resolution or add a resolution knob
                    focal_length,
                    horizontal_aperture,
                    near_plane,
                    far_plane,
                    focal_distance,
                    f_stop,
                    world_matrix,
                    enable_depth_of_field,
                ),
            )
        }
        DamascusNodeTemplate::Light => {
            let mut scene_lights = evaluator.input_light("lights")?;
            let world_matrix = evaluator.input_matrix4("world_matrix")?;
            let light_type = evaluator.input_combo_box::<lights::Lights>("light_type")?;
            let dimensional_data = match light_type {
                lights::Lights::Directional => (world_matrix
                    * glam::Vec4::from((evaluator.input_vector3("direction")?, 1.)))
                .xyz()
                .normalize(),
                lights::Lights::Point => (world_matrix
                    * glam::Vec4::from((evaluator.input_vector3("position")?, 1.)))
                .xyz(),
                lights::Lights::AmbientOcclusion => {
                    glam::Vec3::new(evaluator.input_uint("iterations")? as f32, 0., 0.)
                }
                _ => glam::Vec3::ZERO,
            };
            let intensity = evaluator.input_float("intensity")?;
            let falloff = evaluator.input_uint("falloff")?;
            let colour = evaluator.input_vector3("colour")?;
            let shadow_hardness = evaluator.input_float("shadow_hardness")?;
            let soften_shadows = evaluator.input_bool("soften_shadows")?;

            let light = lights::Light {
                light_type: light_type,
                dimensional_data: dimensional_data,
                intensity: intensity,
                falloff: falloff,
                colour: colour,
                shadow_hardness: shadow_hardness,
                soften_shadows: soften_shadows,
            };

            scene_lights.push(light);
            evaluator.output_light("out", scene_lights)
        }
        DamascusNodeTemplate::Material => {
            let diffuse_colour = evaluator.input_vector3("diffuse_colour")?;
            let diffuse_texture = evaluator.input_procedural_texture("diffuse_texture")?;
            let specular_probability = evaluator.input_float("specular_probability")?;
            let specular_roughness = evaluator.input_float("specular_roughness")?;
            let specular_colour = evaluator.input_vector3("specular_colour")?;
            let transmissive_probability = evaluator.input_float("transmissive_probability")?;
            let transmissive_roughness = evaluator.input_float("transmissive_roughness")?;
            let transmissive_colour = evaluator.input_vector3("transmissive_colour")?;
            let extinction_coefficient = evaluator.input_float("extinction_coefficient")?;
            let emissive_probability = evaluator.input_float("emissive_probability")?;
            let emissive_colour = evaluator.input_vector3("emissive_colour")?;
            let refractive_index = evaluator.input_float("refractive_index")?;
            let scattering_coefficient = evaluator.input_float("scattering_coefficient")?;
            let scattering_colour = evaluator.input_vector3("scattering_colour")?;

            evaluator.output_material(
                "out",
                materials::Material {
                    diffuse_colour: diffuse_colour,
                    diffuse_texture: diffuse_texture,
                    specular_probability: specular_probability,
                    specular_roughness: specular_roughness * specular_roughness,
                    specular_colour: specular_colour,
                    transmissive_probability: transmissive_probability,
                    transmissive_roughness: transmissive_roughness * transmissive_roughness,
                    transmissive_colour: transmissive_colour,
                    extinction_coefficient: extinction_coefficient,
                    emissive_probability: emissive_probability,
                    emissive_colour: emissive_colour,
                    refractive_index: refractive_index,
                    scattering_coefficient: scattering_coefficient,
                    scattering_colour: scattering_colour,
                },
            )
        }
        DamascusNodeTemplate::Primitive => {
            let mut scene_primitives = evaluator.input_primitive("siblings")?;
            let mut descendants = evaluator.input_primitive("children")?;
            let material = evaluator.input_material("material")?;
            let shape = evaluator.input_combo_box::<geometry::Shapes>("shape")?;

            let dimensional_data = match shape {
                geometry::Shapes::Sphere => {
                    glam::Vec4::new(evaluator.input_float("radius")?, 0., 0., 0.)
                }
                geometry::Shapes::Ellipsoid => {
                    glam::Vec4::from((evaluator.input_vector3("radii")?, 0.))
                }
                geometry::Shapes::CutSphere => glam::Vec4::new(
                    evaluator.input_float("radius")?,
                    evaluator.input_float("height")?,
                    0.,
                    0.,
                ),
                geometry::Shapes::HollowSphere => glam::Vec4::new(
                    evaluator.input_float("radius")?,
                    evaluator.input_float("height")?,
                    evaluator.input_float("thickness")?,
                    0.,
                ),
                geometry::Shapes::DeathStar => glam::Vec4::new(
                    evaluator.input_float("radius")?,
                    evaluator.input_float("hollow_radius")?,
                    evaluator.input_float("hollow_height")?,
                    0.,
                ),
                geometry::Shapes::SolidAngle => glam::Vec4::new(
                    evaluator.input_float("radius")?,
                    evaluator.input_float("solid_angle")?,
                    0.,
                    0.,
                ),
                geometry::Shapes::RectangularPrism => glam::Vec4::new(
                    evaluator.input_float("width")?,
                    evaluator.input_float("height")?,
                    evaluator.input_float("depth")?,
                    0.,
                ),
                geometry::Shapes::RectangularPrismFrame => glam::Vec4::new(
                    evaluator.input_float("width")?,
                    evaluator.input_float("height")?,
                    evaluator.input_float("depth")?,
                    evaluator.input_float("thickness")?,
                ),
                geometry::Shapes::Rhombus => glam::Vec4::new(
                    evaluator.input_float("width")?,
                    evaluator.input_float("height")?,
                    evaluator.input_float("depth")?,
                    evaluator.input_float("corner_radius")?,
                ),
                geometry::Shapes::TriangularPrism => glam::Vec4::new(
                    evaluator.input_float("base")?,
                    evaluator.input_float("depth")?,
                    0.,
                    0.,
                ),
                geometry::Shapes::Cylinder => glam::Vec4::new(
                    evaluator.input_float("radius")?,
                    evaluator.input_float("height")?,
                    0.,
                    0.,
                ),
                geometry::Shapes::InfiniteCylinder => {
                    glam::Vec4::new(evaluator.input_float("radius")?, 0., 0., 0.)
                }
                geometry::Shapes::Plane => {
                    glam::Vec4::from((evaluator.input_vector3("normal")?, 0.))
                }
                geometry::Shapes::Capsule => glam::Vec4::new(
                    evaluator.input_float("radius")?,
                    evaluator.input_float("negative_height")?,
                    evaluator.input_float("positive_height")?,
                    0.,
                ),
                geometry::Shapes::Cone => glam::Vec4::new(
                    evaluator.input_float("angle")?,
                    evaluator.input_float("height")?,
                    0.,
                    0.,
                ),
                geometry::Shapes::InfiniteCone => {
                    glam::Vec4::new(evaluator.input_float("angle")?, 0., 0., 0.)
                }
                geometry::Shapes::CappedCone | geometry::Shapes::RoundedCone => glam::Vec4::new(
                    evaluator.input_float("height")?,
                    evaluator.input_float("lower_radius")?,
                    evaluator.input_float("upper_radius")?,
                    0.,
                ),
                geometry::Shapes::Torus => glam::Vec4::new(
                    evaluator.input_float("ring_radius")?,
                    evaluator.input_float("tube_radius")?,
                    0.,
                    0.,
                ),
                geometry::Shapes::CappedTorus => glam::Vec4::new(
                    evaluator.input_float("ring_radius")?,
                    evaluator.input_float("tube_radius")?,
                    evaluator.input_float("cap_angle")?,
                    0.,
                ),
                geometry::Shapes::Link => glam::Vec4::new(
                    evaluator.input_float("ring_radius")?,
                    evaluator.input_float("tube_radius")?,
                    evaluator.input_float("height")?,
                    0.,
                ),
                geometry::Shapes::HexagonalPrism => glam::Vec4::new(
                    evaluator.input_float("height")?,
                    evaluator.input_float("depth")?,
                    0.,
                    0.,
                ),
                geometry::Shapes::Octahedron => {
                    glam::Vec4::new(evaluator.input_float("radial_extent")?, 0., 0., 0.)
                }
                geometry::Shapes::Mandelbulb => glam::Vec4::new(
                    evaluator.input_float("power")?,
                    evaluator.input_uint("iterations")? as f32,
                    evaluator.input_float("max_square_radius")?,
                    0.,
                ),
                geometry::Shapes::Mandelbox => glam::Vec4::new(
                    evaluator.input_float("scale")?,
                    evaluator.input_uint("iterations")? as f32,
                    evaluator.input_float("min_square_radius")?,
                    evaluator.input_float("folding_limit")?,
                ),
            };
            let edge_radius = evaluator.input_float("edge_radius")?;
            let repetition = evaluator.input_combo_box::<geometry::Repetition>("repetition")?;
            let negative_repetitions = evaluator.input_uint_vector3("negative_repetitions")?;
            let positive_repetitions = evaluator.input_uint_vector3("positive_repetitions")?;
            let spacing = evaluator.input_vector3("spacing")?;
            let bounding_volume = evaluator.input_bool("bounding_volume")?;
            let blend_type = evaluator.input_combo_box::<geometry::BlendType>("blend_type")?;
            let blend_strength = evaluator.input_float("blend_strength")?;
            let mirror = evaluator.input_bool_vector3("mirror")?;
            let hollow = evaluator.input_bool("hollow")?;
            let wall_thickness = evaluator.input_float("wall_thickness")?;
            let elongate = evaluator.input_bool("elongate")?;
            let elongation = evaluator.input_vector3("elongation")?;
            let world_matrix = evaluator.input_matrix4("world_matrix")?;
            for child in descendants.iter_mut() {
                child.world_matrix = world_matrix * child.world_matrix;
            }

            let primitive = geometry::Primitive {
                shape: shape,
                world_matrix: world_matrix,
                material: material,
                hollow: hollow,
                wall_thickness: wall_thickness,
                edge_radius: edge_radius,
                mirror: mirror,
                elongate: elongate,
                elongation: elongation,
                repetition: repetition,
                negative_repetitions: negative_repetitions,
                positive_repetitions: positive_repetitions,
                spacing: spacing,
                blend_type: blend_type,
                blend_strength: blend_strength,
                bounding_volume: bounding_volume,
                num_descendants: descendants.len() as u32,
                dimensional_data: dimensional_data,
            };

            scene_primitives.push(primitive);
            scene_primitives.append(&mut descendants);
            evaluator.output_primitive("out", scene_primitives)
        }
        DamascusNodeTemplate::ProceduralTexture => {
            let texture_type =
                evaluator.input_combo_box::<materials::ProceduralTextureType>("texture_type")?;
            let black_point = evaluator.input_float("black_point")?;
            let white_point = evaluator.input_float("white_point")?;
            let lift = evaluator.input_float("lift")?;
            let gamma = evaluator.input_float("gamma")?;

            evaluator.output_procedural_texture(
                "out",
                materials::ProceduralTexture {
                    texture_type: texture_type,
                    black_point: black_point,
                    white_point: white_point,
                    lift: lift,
                    gamma: gamma,
                },
            )
        }
        DamascusNodeTemplate::RayMarcher => {
            let scene = evaluator.input_scene("scene")?;
            let paths_per_pixel = evaluator.input_uint("paths_per_pixel")?;
            let roulette = evaluator.input_bool("roulette")?;
            let max_distance = evaluator.input_float("max_distance")?;
            let max_ray_steps = evaluator.input_uint("max_ray_steps")?;
            let max_bounces = evaluator.input_uint("max_bounces")?;
            let hit_tolerance = evaluator.input_float("hit_tolerance")?;
            let shadow_bias = evaluator.input_float("shadow_bias")?;
            let max_brightness = evaluator.input_float("max_brightness")?;
            let seeds = evaluator.input_vector3("seeds")?;
            let dynamic_level_of_detail = evaluator.input_bool("dynamic_level_of_detail")?;
            let max_light_sampling_bounces = evaluator.input_uint("max_light_sampling_bounces")?;
            let sample_hdri = evaluator.input_bool("sample_hdri")?;
            let sample_all_lights = evaluator.input_bool("sample_all_lights")?;
            let light_sampling_bias = evaluator.input_float("light_sampling_bias")?;
            let secondary_sampling = evaluator.input_bool("secondary_sampling")?;
            let hdri_offset_angle = evaluator.input_float("hdri_offset_angle")?;
            let latlong = evaluator.input_bool("latlong")?;
            let output_aov = evaluator.input_combo_box::<renderers::AOVs>("output_aov")?;

            evaluator.output_ray_marcher(
                "out",
                renderers::RayMarcher {
                    scene: scene,
                    paths_per_pixel: paths_per_pixel,
                    roulette: roulette,
                    max_distance: max_distance,
                    max_ray_steps: max_ray_steps,
                    max_bounces: max_bounces,
                    hit_tolerance: hit_tolerance,
                    shadow_bias: shadow_bias,
                    max_brightness: max_brightness,
                    seeds: seeds,
                    dynamic_level_of_detail: dynamic_level_of_detail,
                    max_light_sampling_bounces: max_light_sampling_bounces,
                    sample_hdri: sample_hdri,
                    sample_all_lights: sample_all_lights,
                    light_sampling_bias: light_sampling_bias,
                    secondary_sampling: secondary_sampling,
                    hdri_offset_angle: hdri_offset_angle,
                    output_aov: output_aov,
                    latlong: latlong,
                },
            )
        }
        DamascusNodeTemplate::Scene => {
            let render_camera = evaluator.input_camera("render_camera")?;
            let primitives = evaluator.input_primitive("primitives")?;
            let lights = evaluator.input_light("lights")?;
            let atmosphere = evaluator.input_material("atmosphere")?;
            evaluator.output_scene(
                "out",
                scene::Scene {
                    render_camera: render_camera,
                    lights: lights,
                    primitives: primitives,
                    atmosphere: atmosphere,
                },
            )
        }
    }
}

fn populate_output(
    graph: &DamascusGraph,
    outputs_cache: &mut OutputsCache,
    node_id: NodeId,
    param_name: &str,
    value: DamascusValueType,
) -> anyhow::Result<DamascusValueType> {
    let output_id = graph[node_id].get_output(param_name)?;
    outputs_cache.insert(output_id, value.clone());
    Ok(value)
}

// Evaluates the input value of
fn evaluate_input(
    graph: &DamascusGraph,
    node_id: NodeId,
    param_name: &str,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<DamascusValueType> {
    let input_id = graph[node_id].get_input(param_name)?;

    // The output of another node is connected.
    if let Some(other_output_id) = graph.connection(input_id) {
        // The value was already computed due to the evaluation of some other
        // node. We simply return value from the cache.
        if let Some(other_value) = outputs_cache.get(&other_output_id) {
            Ok((*other_value).clone())
        }
        // This is the first time encountering this node, so we need to
        // recursively evaluate it.
        else {
            // Calling this will populate the cache
            evaluate_node(graph, graph[other_output_id].node, outputs_cache)?;

            // Now that we know the value is cached, return it
            Ok((*outputs_cache
                .get(&other_output_id)
                .expect("Cache should be populated"))
            .clone())
        }
    }
    // No existing connection, take the inline value instead.
    else {
        Ok(graph[input_id].value.clone())
    }
}
