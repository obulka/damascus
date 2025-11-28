// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use macro_rules_attribute::derive;

use crate::{
    EnumHashTraits,
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{SceneGraph, SceneGraphId, SceneGraphIdType},
    },
    textures::evaluators::{TextureEvaluators, grade::Grade},
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

    fn output_is_compatible_with_input(output: &OutputData, input: &Self::Inputs) -> bool {
        match input {
            Self::Inputs::Texture => {
                *output == OutputData::SceneGraphId(SceneGraphIdType::TextureEvaluator)
            }
            _ => false,
        }
    }

    fn evaluate(
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        // let texture_evaluator_id: TextureEvaluatorId = Self::Inputs::Texture.get_data(data_map)?.try_to_texture_evaluator_id()?;

        match output {
            Self::Outputs::Grade => Ok(InputData::SceneGraphId(
                scene_graph
                    .add_texture_evaluator(TextureEvaluators::Grade(
                        Grade::default()
                            .black_point(
                                Self::Inputs::BlackPoint
                                    .get_data(data_map)?
                                    .try_to_float()?,
                            )
                            .white_point(
                                Self::Inputs::WhitePoint
                                    .get_data(data_map)?
                                    .try_to_float()?,
                            )
                            .lift(Self::Inputs::Lift.get_data(data_map)?.try_to_float()?)
                            .gain(Self::Inputs::Gain.get_data(data_map)?.try_to_float()?)
                            .gamma(Self::Inputs::Gamma.get_data(data_map)?.try_to_float()?)
                            .invert(Self::Inputs::Invert.get_data(data_map)?.try_to_bool()?)
                            .transform(Self::Inputs::Transform.get_data(data_map)?.try_to_mat4()?),
                    ))
                    .into(),
            )),
        }
    }
}
