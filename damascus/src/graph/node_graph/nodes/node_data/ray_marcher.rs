// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use macro_rules_attribute::derive;

use crate::{
    EnumHashTraits,
    gpu::scene::GPUScene,
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeErrors, NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{SceneGraph, SceneGraphId, SceneGraphIdType},
    },
    textures::evaluators::{
        TextureEvaluatorId, TextureEvaluators,
        ray_marcher::{RayMarcher, RayMarcherRenderData},
    },
};

#[derive(Copy, Default, EnumHashTraits!)]
pub enum RayMarcherInputData {
    #[default]
    SceneRoot,
    MaxRaySteps,
    MaxBounces,
    HitTolerance,
    ShadowBias,
    MaxBrightness,
    Seed,
    DynamicLevelOfDetail,
    EquiangularSamples,
    LightSampling,
    MaxLightSamplingBounces,
    SampleAtmosphere,
    LightSamplingBias,
    SecondarySampling,
    OutputAov,
}

impl NodeInputData for RayMarcherInputData {
    fn default_data(&self) -> InputData {
        let default_ray_marcher = RayMarcherRenderData::default();
        match self {
            Self::SceneRoot => InputData::SceneGraphId(SceneGraphId::None),
            Self::MaxRaySteps => InputData::UInt(default_ray_marcher.max_ray_steps),
            Self::MaxBounces => InputData::UInt(default_ray_marcher.max_bounces),
            Self::HitTolerance => InputData::Float(default_ray_marcher.hit_tolerance),
            Self::ShadowBias => InputData::Float(default_ray_marcher.shadow_bias),
            Self::MaxBrightness => InputData::Float(default_ray_marcher.max_brightness),
            Self::Seed => InputData::UInt(default_ray_marcher.seed),
            Self::DynamicLevelOfDetail => {
                InputData::Bool(default_ray_marcher.dynamic_level_of_detail)
            }
            Self::EquiangularSamples => InputData::UInt(default_ray_marcher.equiangular_samples),
            Self::LightSampling => InputData::Bool(default_ray_marcher.light_sampling),
            Self::MaxLightSamplingBounces => {
                InputData::UInt(default_ray_marcher.max_light_sampling_bounces)
            }
            Self::SampleAtmosphere => InputData::Bool(default_ray_marcher.sample_atmosphere),
            Self::LightSamplingBias => InputData::Float(default_ray_marcher.light_sampling_bias),
            Self::SecondarySampling => InputData::Bool(default_ray_marcher.secondary_sampling),
            Self::OutputAov => InputData::Enum(default_ray_marcher.output_aov.into()),
        }
    }
}

#[derive(Copy, Default, EnumHashTraits!)]
pub enum RayMarcherOutputData {
    #[default]
    Render,
}

impl NodeOutputData for RayMarcherOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Render => OutputData::SceneGraphId(SceneGraphIdType::TextureEvaluator),
        }
    }
}

pub struct RayMarcherNode;

impl EvaluableNode for RayMarcherNode {
    type Inputs = RayMarcherInputData;
    type Outputs = RayMarcherOutputData;

    fn output_data_is_compatible_with_input(
        output_data: &OutputData,
        input: &Self::Inputs,
    ) -> bool {
        match input {
            Self::Inputs::SceneRoot => {
                *output_data == OutputData::SceneGraphId(SceneGraphIdType::Root)
            }
            _ => false,
        }
    }

    fn update_from_data_map(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        input_data: &InputData,
    ) -> NodeResult<()> {
        let texture_evaluator_id: &TextureEvaluatorId = input_data.as_texture_evaluator_id()?;

        let gpu_scene: GPUScene = if let Ok(root_id) = Self::Inputs::SceneRoot
            .from_data_map(data_map)?
            .try_to_root_id()
        {
            scene_graph.as_gpu_scene(root_id)
        } else {
            GPUScene::default()
        };

        match &mut scene_graph[*texture_evaluator_id] {
            TextureEvaluators::RayMarcher(ray_marcher) => {
                ray_marcher.set_gpu_scene(gpu_scene);
                ray_marcher.set_max_ray_steps(
                    Self::Inputs::MaxRaySteps
                        .from_data_map(data_map)?
                        .try_to_uint()?,
                );
                ray_marcher.set_max_bounces(
                    Self::Inputs::MaxBounces
                        .from_data_map(data_map)?
                        .try_to_uint()?,
                );
                ray_marcher.set_hit_tolerance(
                    Self::Inputs::HitTolerance
                        .from_data_map(data_map)?
                        .try_to_float()?,
                );
                ray_marcher.set_shadow_bias(
                    Self::Inputs::ShadowBias
                        .from_data_map(data_map)?
                        .try_to_float()?,
                );
                ray_marcher.set_max_brightness(
                    Self::Inputs::MaxBrightness
                        .from_data_map(data_map)?
                        .try_to_float()?,
                );
                ray_marcher.set_seed(Self::Inputs::Seed.from_data_map(data_map)?.try_to_uint()?);
                ray_marcher.set_dynamic_level_of_detail(
                    Self::Inputs::DynamicLevelOfDetail
                        .from_data_map(data_map)?
                        .try_to_bool()?,
                );
                ray_marcher.set_equiangular_samples(
                    Self::Inputs::EquiangularSamples
                        .from_data_map(data_map)?
                        .try_to_uint()?,
                );
                ray_marcher.set_light_sampling(
                    Self::Inputs::LightSampling
                        .from_data_map(data_map)?
                        .try_to_bool()?,
                );
                ray_marcher.set_max_light_sampling_bounces(
                    Self::Inputs::MaxLightSamplingBounces
                        .from_data_map(data_map)?
                        .try_to_uint()?,
                );
                ray_marcher.set_sample_atmosphere(
                    Self::Inputs::SampleAtmosphere
                        .from_data_map(data_map)?
                        .try_to_bool()?,
                );
                ray_marcher.set_light_sampling_bias(
                    Self::Inputs::LightSamplingBias
                        .from_data_map(data_map)?
                        .try_to_float()?,
                );
                ray_marcher.set_secondary_sampling(
                    Self::Inputs::SecondarySampling
                        .from_data_map(data_map)?
                        .try_to_bool()?,
                );
                ray_marcher.set_output_aov(
                    Self::Inputs::OutputAov
                        .from_data_map(data_map)?
                        .try_to_enum()?,
                );

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
                .add_texture_evaluator(TextureEvaluators::RayMarcher(RayMarcher::default())),
        };

        let scene_graph_id = InputData::SceneGraphId(texture_evaluator_id.into());

        Self::update_from_data_map(scene_graph, data_map, &scene_graph_id)?;

        scene_graph[texture_evaluator_id].evaluate(device, queue, encoder);

        match output {
            Self::Outputs::Render => Ok(scene_graph_id),
        }
    }
}
