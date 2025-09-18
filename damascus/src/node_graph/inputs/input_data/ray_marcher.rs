// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{render_passes::ray_marcher::RayMarcherRenderData, Enumerator};

use super::{InputData, NodeInputData};

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
    Scene,
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
            Self::Scene => InputData::Scene(default_ray_marcher.scene),
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
