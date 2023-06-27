use std::{borrow::Cow, collections::HashMap};

use eframe::egui::{self, DragValue, TextStyle};
use egui_node_graph::*;
use glam;

use damascus_core::{geometry, materials};

use crate::viewport_3d::Viewport3d;

// ========= First, define your user data types =============

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct DamascusNodeData {
    template: DamascusNodeTemplate,
}

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum DamascusDataType {
    Float,
    Vec2,
    Vec3,
}

/// In the graph, input parameters can optionally have a constant value. This
/// value can be directly edited in a widget inside the node itself.
///
/// There will usually be a correspondence between DataTypes and ValueTypes. But
/// this library makes no attempt to check this consistency. For instance, it is
/// up to the user code in this example to make sure no parameter is created
/// with a DataType of Float and a ValueType of Vec2.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum DamascusValueType {
    Float { value: f32 },
    Vec2 { value: glam::Vec2 },
    Vec3 { value: glam::Vec3 },
}

impl Default for DamascusValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::Float { value: 0.0 }
    }
}

impl DamascusValueType {
    /// Tries to downcast this value type to a float
    pub fn try_to_float(self) -> anyhow::Result<f32> {
        if let DamascusValueType::Float { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to float", self)
        }
    }

    /// Tries to downcast this value type to a vector2
    pub fn try_to_vec2(self) -> anyhow::Result<glam::Vec2> {
        if let DamascusValueType::Vec2 { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to vec2", self)
        }
    }

    /// Tries to downcast this value type to a vector3
    pub fn try_to_vec3(self) -> anyhow::Result<glam::Vec3> {
        if let DamascusValueType::Vec3 { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to vec3", self)
        }
    }
}

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum DamascusNodeTemplate {
    MakeFloat,
    MakeVector2,
    MakeVector3,
    AddFloat,
    AddVector2,
    AddVector3,
}

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamascusResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct DamascusGraphState {
    pub active_node: Option<NodeId>,
}

// =========== Then, you need to implement some traits ============

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<DamascusGraphState> for DamascusDataType {
    fn data_type_color(&self, _user_state: &mut DamascusGraphState) -> egui::Color32 {
        match self {
            DamascusDataType::Float => egui::Color32::from_rgb(38, 109, 211),
            DamascusDataType::Vec2 => egui::Color32::from_rgb(238, 207, 109),
            DamascusDataType::Vec3 => egui::Color32::from_rgb(79, 0, 107),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            DamascusDataType::Float => "scalar float",
            DamascusDataType::Vec2 => "2d vector",
            DamascusDataType::Vec3 => "3d vector",
        })
    }
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for DamascusNodeTemplate {
    type NodeData = DamascusNodeData;
    type DataType = DamascusDataType;
    type ValueType = DamascusValueType;
    type UserState = DamascusGraphState;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            DamascusNodeTemplate::MakeFloat => "New float",
            DamascusNodeTemplate::MakeVector2 => "New vector2",
            DamascusNodeTemplate::MakeVector3 => "New vector3",
            DamascusNodeTemplate::AddFloat => "Float add",
            DamascusNodeTemplate::AddVector2 => "Vector2 add",
            DamascusNodeTemplate::AddVector3 => "Vector3 add",
        })
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        DamascusNodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        // The nodes are created empty by default. This function needs to take
        // care of creating the desired inputs and outputs based on the template

        // We define some closures here to avoid boilerplate. Note that this is
        // entirely optional.
        let input_float = |graph: &mut DamascusGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Float,
                DamascusValueType::Float { value: 0.0 },
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let input_vector2 = |graph: &mut DamascusGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Vec2,
                DamascusValueType::Vec2 {
                    value: glam::Vec2::ZERO,
                },
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let input_vector3 = |graph: &mut DamascusGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Vec3,
                DamascusValueType::Vec3 {
                    value: glam::Vec3::ZERO,
                },
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };

        let output_float = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Float);
        };
        let output_vector2 = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Vec2);
        };
        let output_vector3 = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Vec3);
        };

        match self {
            DamascusNodeTemplate::MakeFloat => {
                input_float(graph, "value");
                output_float(graph, "out");
            }
            DamascusNodeTemplate::MakeVector2 => {
                input_float(graph, "x");
                input_float(graph, "y");
                output_vector2(graph, "out");
            }
            DamascusNodeTemplate::MakeVector3 => {
                input_float(graph, "x");
                input_float(graph, "y");
                input_float(graph, "z");
                output_vector3(graph, "out");
            }
            DamascusNodeTemplate::AddFloat => {
                // This input param doesn't use the closure so we can comment
                // it in more detail.
                graph.add_input_param(
                    node_id,
                    // This is the name of the parameter. Can be later used to
                    // retrieve the value. Parameter names should be unique.
                    "A".into(),
                    // The data type for this input. In this case, a float
                    DamascusDataType::Float,
                    // The value type for this input. We store zero as default
                    DamascusValueType::Float { value: 0.0 },
                    // The input parameter kind. This allows defining whether a
                    // parameter accepts input connections and/or an inline
                    // widget to set its value.
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
                input_float(graph, "B");
                output_float(graph, "out");
            }
            DamascusNodeTemplate::AddVector2 => {
                input_vector2(graph, "v1");
                input_vector2(graph, "v2");
                output_vector2(graph, "out");
            }
            DamascusNodeTemplate::AddVector3 => {
                input_vector3(graph, "v1");
                input_vector3(graph, "v2");
                output_vector3(graph, "out");
            }
        }
    }
}

pub struct AllDamascusNodeTemplates;
impl NodeTemplateIter for AllDamascusNodeTemplates {
    type Item = DamascusNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // This function must return a list of node kinds, which the node finder
        // will use to display it to the user. Crates like strum can reduce the
        // boilerplate in enumerating all variants of an enum.
        vec![
            DamascusNodeTemplate::MakeFloat,
            DamascusNodeTemplate::MakeVector2,
            DamascusNodeTemplate::MakeVector3,
            DamascusNodeTemplate::AddFloat,
            DamascusNodeTemplate::AddVector2,
            DamascusNodeTemplate::AddVector3,
        ]
    }
}

impl WidgetValueTrait for DamascusValueType {
    type Response = DamascusResponse;
    type UserState = DamascusGraphState;
    type NodeData = DamascusNodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut DamascusGraphState,
        _node_data: &DamascusNodeData,
    ) -> Vec<DamascusResponse> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        match self {
            DamascusValueType::Float { value } => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value));
                });
            }
            DamascusValueType::Vec2 { value } => {
                ui.label(param_name);
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(DragValue::new(&mut value.x));
                    ui.label("y");
                    ui.add(DragValue::new(&mut value.y));
                });
            }
            DamascusValueType::Vec3 { value } => {
                ui.label(param_name);
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(DragValue::new(&mut value.x));
                    ui.label("y");
                    ui.add(DragValue::new(&mut value.y));
                    ui.label("z");
                    ui.add(DragValue::new(&mut value.z));
                });
            }
        }
        // This allows you to return your responses from the inline widgets.
        Vec::new()
    }
}

impl UserResponseTrait for DamascusResponse {}
impl NodeDataTrait for DamascusNodeData {
    type Response = DamascusResponse;
    type UserState = DamascusGraphState;
    type DataType = DamascusDataType;
    type ValueType = DamascusValueType;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<DamascusNodeData, DamascusDataType, DamascusValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<DamascusResponse, DamascusNodeData>>
    where
        DamascusResponse: UserResponseTrait,
    {
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        // Pressing the button will emit a custom user response to either set,
        // or clear the active node. These responses do nothing by themselves,
        // the library only makes the responses available to you after the graph
        // has been drawn. See below at the update method for an example.
        if !is_active {
            if ui.button("👁 Set active").clicked() {
                responses.push(NodeResponse::User(DamascusResponse::SetActiveNode(node_id)));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(DamascusResponse::ClearActiveNode));
            }
        }

        responses
    }
}

type DamascusGraph = Graph<DamascusNodeData, DamascusDataType, DamascusValueType>;
type DamascusEditorState = GraphEditorState<
    DamascusNodeData,
    DamascusDataType,
    DamascusValueType,
    DamascusNodeTemplate,
    DamascusGraphState,
>;

pub struct Damascus {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: DamascusEditorState,
    user_state: DamascusGraphState,
    viewport_3d: Option<Viewport3d>,
}

#[cfg(feature = "persistence")]
const PERSISTENCE_KEY: &str = "damascus";

#[cfg(feature = "persistence")]
impl Damascus {
    /// If the persistence feature is enabled, Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
            .unwrap_or_default();
        Self {
            state,
            user_state: DamascusGraphState::default(),
            viewport_3d: Viewport3d::new(cc),
        }
    }
}

#[cfg(not(feature = "persistence"))]
impl Damascus {
    /// If the persistence feature is enabled, Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: DamascusEditorState::default(),
            user_state: DamascusGraphState::default(),
            viewport_3d: Viewport3d::new(cc),
        }
    }
}

impl eframe::App for Damascus {
    #[cfg(feature = "persistence")]
    /// If the persistence function is enabled,
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, &self.state);
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        ui.close_menu();
                    }
                    ui.menu_button("SubMenu", |ui| {
                        ui.menu_button("SubMenu", |ui| {
                            if ui.button("Open...").clicked() {
                                ui.close_menu();
                            }
                            let _ = ui.button("Item");
                        });
                        ui.menu_button("SubMenu", |ui| {
                            if ui.button("Open...").clicked() {
                                ui.close_menu();
                            }
                            let _ = ui.button("Item");
                        });
                        let _ = ui.button("Item");
                        if ui.button("Open...").clicked() {
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("SubMenu", |ui| {
                        let _ = ui.button("Item1");
                        let _ = ui.button("Item2");
                        let _ = ui.button("Item3");
                        let _ = ui.button("Item4");
                        if ui.button("Open...").clicked() {
                            ui.close_menu();
                        }
                    });
                    let _ = ui.button("Very long text for this item");
                });
            });
        });
        egui::SidePanel::right("properties")
            .resizable(true)
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Right Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("test");
                });
            });

        let graph_response = egui::TopBottomPanel::bottom("bottom")
            .resizable(true)
            .default_height(300.0)
            .show(ctx, |ui| {
                self.state
                    .draw_graph_editor(ui, AllDamascusNodeTemplates, &mut self.user_state)
            })
            .inner;
        for node_response in graph_response.node_responses {
            // Here, we ignore all other graph events. But you may find
            // some use for them. For example, by playing a sound when a new
            // connection is created
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    DamascusResponse::SetActiveNode(node) => {
                        self.user_state.active_node = Some(node)
                    }
                    DamascusResponse::ClearActiveNode => self.user_state.active_node = None,
                }
            }
        }

        if let Some(node) = self.user_state.active_node {
            if self.state.graph.nodes.contains_key(node) {
                let angle = match evaluate_node(&self.state.graph, node, &mut HashMap::new()) {
                    Ok(value) => value,
                    Err(error) => {
                        ctx.debug_painter().text(
                            egui::pos2(10.0, 35.0),
                            egui::Align2::LEFT_TOP,
                            format!("Error: {}", error),
                            TextStyle::Button.resolve(&ctx.style()),
                            egui::Color32::RED,
                        );

                        DamascusValueType::Float { value: 0.0 }
                    }
                };
                if let Some(ref mut viewport_3d) = &mut self.viewport_3d {
                    match angle {
                        DamascusValueType::Float { value } => {
                            viewport_3d.angle = value;
                        }
                        DamascusValueType::Vec2 { value } => {
                            viewport_3d.angle = value.x;
                        }
                        DamascusValueType::Vec3 { value } => {
                            viewport_3d.angle = value.x;
                        }
                    }
                }
            } else {
                self.user_state.active_node = None;
            }
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default())
            .show(ctx, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    if let Some(viewport_3d) = &mut self.viewport_3d {
                        viewport_3d.custom_painting(ui);
                    }
                });
            });
    }
}

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

        fn input_float(&mut self, name: &str) -> anyhow::Result<f32> {
            self.evaluate_input(name)?.try_to_float()
        }

        fn output_float(&mut self, name: &str, value: f32) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Float { value })
        }

        fn input_vector2(&mut self, name: &str) -> anyhow::Result<glam::Vec2> {
            self.evaluate_input(name)?.try_to_vec2()
        }

        fn output_vector2(
            &mut self,
            name: &str,
            value: glam::Vec2,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Vec2 { value })
        }

        fn input_vector3(&mut self, name: &str) -> anyhow::Result<glam::Vec3> {
            self.evaluate_input(name)?.try_to_vec3()
        }

        fn output_vector3(
            &mut self,
            name: &str,
            value: glam::Vec3,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Vec3 { value })
        }
    }

    let node = &graph[node_id];
    let mut evaluator = Evaluator::new(graph, outputs_cache, node_id);
    match node.user_data.template {
        DamascusNodeTemplate::MakeFloat => {
            let value = evaluator.input_float("value")?;
            evaluator.output_float("out", value)
        }
        DamascusNodeTemplate::MakeVector2 => {
            let x = evaluator.input_float("x")?;
            let y = evaluator.input_float("y")?;
            evaluator.output_vector2("out", glam::Vec2 { x, y })
        }
        DamascusNodeTemplate::MakeVector3 => {
            let x = evaluator.input_float("x")?;
            let y = evaluator.input_float("y")?;
            let z = evaluator.input_float("z")?;
            evaluator.output_vector3("out", glam::Vec3 { x, y, z })
        }
        DamascusNodeTemplate::AddFloat => {
            let a = evaluator.input_float("A")?;
            let b = evaluator.input_float("B")?;
            evaluator.output_float("out", a + b)
        }
        DamascusNodeTemplate::AddVector2 => {
            let v1 = evaluator.input_vector2("v1")?;
            let v2 = evaluator.input_vector2("v2")?;
            evaluator.output_vector2("out", v1 + v2)
        }
        DamascusNodeTemplate::AddVector3 => {
            let v1 = evaluator.input_vector3("v1")?;
            let v2 = evaluator.input_vector3("v2")?;
            evaluator.output_vector3("out", v1 + v2)
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
    outputs_cache.insert(output_id, value);
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
            Ok(*other_value)
        }
        // This is the first time encountering this node, so we need to
        // recursively evaluate it.
        else {
            // Calling this will populate the cache
            evaluate_node(graph, graph[other_output_id].node, outputs_cache)?;

            // Now that we know the value is cached, return it
            Ok(*outputs_cache
                .get(&other_output_id)
                .expect("Cache should be populated"))
        }
    }
    // No existing connection, take the inline value instead.
    else {
        Ok(graph[input_id].value)
    }
}
