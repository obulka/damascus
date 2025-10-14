// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    node_graph::{
        inputs::input_data::{InputData, NodeInputData},
        nodes::{node_data::EvaluableNode, NodeResult},
        outputs::output_data::{NodeOutputData, OutputData},
    },
    render_passes::{
        ray_marcher::{RayMarcher, RayMarcherRenderData},
        RenderPass, RenderPasses,
    },
    scene_graph::{SceneGraph, SceneGraphId, SceneGraphIdType},
    Enumerator,
};

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum RayMarcherInputData {
    #[default]
    RenderCamera,
    SceneRoot,
    Atmosphere,
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

impl Enumerator for RayMarcherInputData {}

impl NodeInputData for RayMarcherInputData {
    fn default_data(&self) -> InputData {
        let default_ray_marcher = RayMarcherRenderData::default();
        match self {
            Self::RenderCamera => InputData::SceneGraphId(SceneGraphId::None),
            Self::SceneRoot => InputData::SceneGraphId(SceneGraphId::None),
            Self::Atmosphere => InputData::SceneGraphId(SceneGraphId::None),
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

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum RayMarcherOutputData {
    #[default]
    Render,
}

impl Enumerator for RayMarcherOutputData {}

impl NodeOutputData for RayMarcherOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Render => OutputData::RenderPass,
        }
    }
}

pub struct RayMarcherNode;

impl EvaluableNode for RayMarcherNode {
    type Inputs = RayMarcherInputData;
    type Outputs = RayMarcherOutputData;

    fn output_compatible_with_input(output: &OutputData, input: &Self::Inputs) -> bool {
        match input {
            Self::Inputs::RenderCamera => {
                *output == OutputData::SceneGraphId(SceneGraphIdType::Camera)
            }
            Self::Inputs::SceneRoot => match *output {
                OutputData::SceneGraphId(location_type) => location_type.has_transform(),
                _ => false,
            },
            Self::Inputs::Atmosphere => {
                *output == OutputData::SceneGraphId(SceneGraphIdType::Material)
            }
            _ => false,
        }
    }

    fn evaluate(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        match output {
            Self::Outputs::Render => Ok(InputData::RenderPass(RenderPasses::RayMarcher {
                render_pass: RayMarcher::default()
                    .gpu_scene(
                        Self::Inputs::SceneRoot
                            .get_data(data_map)?
                            .try_to_scene_graph_id()?,
                    )
                    .max_ray_steps(
                        Self::Inputs::MaxRaySteps
                            .get_data(data_map)?
                            .try_to_uint()?,
                    )
                    .max_bounces(Self::Inputs::MaxBounces.get_data(data_map)?.try_to_uint()?)
                    .hit_tolerance(
                        Self::Inputs::HitTolerance
                            .get_data(data_map)?
                            .try_to_float()?,
                    )
                    .shadow_bias(
                        Self::Inputs::ShadowBias
                            .get_data(data_map)?
                            .try_to_float()?,
                    )
                    .max_brightness(
                        Self::Inputs::MaxBrightness
                            .get_data(data_map)?
                            .try_to_float()?,
                    )
                    .seed(Self::Inputs::Seed.get_data(data_map)?.try_to_uint()?)
                    .dynamic_level_of_detail(
                        Self::Inputs::DynamicLevelOfDetail
                            .get_data(data_map)?
                            .try_to_bool()?,
                    )
                    .equiangular_samples(
                        Self::Inputs::EquiangularSamples
                            .get_data(data_map)?
                            .try_to_uint()?,
                    )
                    .max_light_sampling_bounces(
                        Self::Inputs::MaxLightSamplingBounces
                            .get_data(data_map)?
                            .try_to_uint()?,
                    )
                    .light_sampling(
                        Self::Inputs::LightSampling
                            .get_data(data_map)?
                            .try_to_bool()?,
                    )
                    .sample_atmosphere(
                        Self::Inputs::SampleAtmosphere
                            .get_data(data_map)?
                            .try_to_bool()?,
                    )
                    .light_sampling_bias(
                        Self::Inputs::LightSamplingBias
                            .get_data(data_map)?
                            .try_to_float()?,
                    )
                    .secondary_sampling(
                        Self::Inputs::SecondarySampling
                            .get_data(data_map)?
                            .try_to_bool()?,
                    )
                    .output_aov(Self::Inputs::OutputAov.get_data(data_map)?.try_to_enum()?)
                    .finalized(),
            })),
        }
    }
}
