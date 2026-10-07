// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use indoc::indoc;
use macro_rules_attribute::derive;

use crate::{
    EnumHashTraits,
    gpu::resources::TextureView,
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeErrors, NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{SceneGraph, SceneGraphId, SceneGraphIdType},
    },
    textures::evaluators::{
        TextureEvaluator, TextureEvaluatorId, TextureEvaluators, view::TextureViewer,
    },
};

#[derive(Copy, Default, EnumHashTraits!)]
pub enum TextureViewerInputData {
    #[default]
    Texture,
    OutputResolution,
}

impl NodeInputData for TextureViewerInputData {
    fn default_data(&self) -> InputData {
        let default_texture_viewer = TextureViewer::default();
        match self {
            Self::Texture => InputData::SceneGraphId(SceneGraphId::None),
            Self::OutputResolution => InputData::UVec2(default_texture_viewer.output_resolution()),
        }
    }

    fn tooltip(&self) -> &str {
        match self {
            Self::Texture => indoc! {
                "A render pass which results in the production of the texture
                    to view."
            },
            Self::OutputResolution => indoc! {
                "The resolution of the view area, not necessarily the
                    input texture."
            },
        }
    }
}

#[derive(Copy, Default, EnumHashTraits!)]
pub enum TextureViewerOutputData {
    #[default]
    TextureViewer,
}

impl NodeOutputData for TextureViewerOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::TextureViewer => OutputData::SceneGraphId(SceneGraphIdType::TextureEvaluator),
        }
    }

    fn tooltip(&self) -> &str {
        match self {
            Self::TextureViewer => "A texture viewer.",
        }
    }
}

pub struct TextureViewerNode;

impl EvaluableNode for TextureViewerNode {
    type Inputs = TextureViewerInputData;
    type Outputs = TextureViewerOutputData;

    fn output_data_is_compatible_with_input(
        output_data: &OutputData,
        input: &Self::Inputs,
    ) -> bool {
        false
    }

    fn update_from_data_map(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        input_data: &InputData,
    ) -> NodeResult<()> {
        let texture_evaluator_id: &TextureEvaluatorId = input_data.as_texture_evaluator_id()?;

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

        match &mut scene_graph[*texture_evaluator_id] {
            TextureEvaluators::TextureViewer(texture_viewer) => {
                texture_viewer.set_input_texture_views(input_texture_views);

                Ok(())
            }
            _ => Err(NodeErrors::InvalidCachedData),
        }
    }

    fn evaluate(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        cached_input_data: Option<InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        let texture_evaluator_id: TextureEvaluatorId = match cached_input_data {
            Some(input_data) => input_data.try_to_texture_evaluator_id()?,
            None => scene_graph
                .add_texture_evaluator(TextureEvaluators::TextureViewer(TextureViewer::default())),
        };

        let scene_graph_id = InputData::SceneGraphId(texture_evaluator_id.into());

        Self::update_from_data_map(scene_graph, data_map, &scene_graph_id)?;

        scene_graph[texture_evaluator_id].evaluate(device, queue, encoder);

        match output {
            Self::Outputs::TextureViewer => Ok(scene_graph_id),
        }
    }
}
