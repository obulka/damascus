// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use macro_rules_attribute::derive;

use crate::{
    EnumHashTraits,
    gpu::resources::TextureView,
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{SceneGraph, SceneGraphId, SceneGraphIdType},
    },
    textures::evaluators::{TextureEvaluator, TextureEvaluatorId, TextureEvaluators, grade::Grade},
};

#[derive(Copy, Default, EnumHashTraits!)]
pub enum GradeInputData {
    #[default]
    Texture,
    BlackPoint,
    WhitePoint,
    Lift,
    Gain,
    Gamma,
    Invert,
    Transform,
}

impl NodeInputData for GradeInputData {
    fn default_data(&self) -> InputData {
        let default_grade = Grade::default();
        match self {
            Self::Texture => InputData::SceneGraphId(SceneGraphId::None),
            Self::BlackPoint => InputData::Float(default_grade.black_point),
            Self::WhitePoint => InputData::Float(default_grade.white_point),
            Self::Lift => InputData::Float(default_grade.lift),
            Self::Gain => InputData::Float(default_grade.gain),
            Self::Gamma => InputData::Float(default_grade.gamma),
            Self::Invert => InputData::Bool(default_grade.invert),
            Self::Transform => InputData::Mat4(default_grade.transform),
        }
    }
}

#[derive(Copy, Default, EnumHashTraits!)]
pub enum GradeOutputData {
    #[default]
    Grade,
}

impl NodeOutputData for GradeOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Grade => OutputData::SceneGraphId(SceneGraphIdType::TextureEvaluator),
        }
    }
}

pub struct GradeNode;

impl EvaluableNode for GradeNode {
    type Inputs = GradeInputData;
    type Outputs = GradeOutputData;

    fn output_data_is_compatible_with_input(
        output_data: &OutputData,
        input: &Self::Inputs,
    ) -> bool {
        match input {
            Self::Inputs::Texture => {
                *output_data == OutputData::SceneGraphId(SceneGraphIdType::TextureEvaluator)
            }
            _ => false,
        }
    }

    fn evaluate(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        let mut input_texture_views = Vec::<TextureView>::new();
        if let Ok(input_texture_evaluator_id) = Self::Inputs::Texture
            .from_data_map(data_map)?
            .try_to_texture_evaluator_id()
        {
            input_texture_views = scene_graph[input_texture_evaluator_id]
                .output_texture_view()
                .into_iter()
                .cloned()
                .collect();
        }

        let texture_evaluator_id: TextureEvaluatorId =
            scene_graph.add_texture_evaluator(TextureEvaluators::Grade(
                Grade::default()
                    .black_point(
                        Self::Inputs::BlackPoint
                            .from_data_map(data_map)?
                            .try_to_float()?,
                    )
                    .white_point(
                        Self::Inputs::WhitePoint
                            .from_data_map(data_map)?
                            .try_to_float()?,
                    )
                    .lift(Self::Inputs::Lift.from_data_map(data_map)?.try_to_float()?)
                    .gain(Self::Inputs::Gain.from_data_map(data_map)?.try_to_float()?)
                    .gamma(
                        Self::Inputs::Gamma
                            .from_data_map(data_map)?
                            .try_to_float()?,
                    )
                    .invert(
                        Self::Inputs::Invert
                            .from_data_map(data_map)?
                            .try_to_bool()?,
                    )
                    .transform(
                        Self::Inputs::Transform
                            .from_data_map(data_map)?
                            .try_to_mat4()?,
                    )
                    .with_input_texture_views(input_texture_views),
            ));

        let scene_graph_id = SceneGraphId::TextureEvaluator(texture_evaluator_id);

        scene_graph[texture_evaluator_id].evaluate(device, queue, encoder);

        match output {
            Self::Outputs::Grade => Ok(InputData::SceneGraphId(scene_graph_id)),
        }
    }
}
