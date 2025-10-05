// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    materials::Material,
    node_graph::{
        inputs::input_data::{InputData, NodeInputData},
        outputs::output_data::{NodeOutputData, OutputData},
    },
    render_passes::RenderPasses,
    Enumerator,
};

use super::EvaluableNode;

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
pub enum MaterialInputData {
    #[default]
    DiffuseColour,
    DiffuseColourTexture,
    SpecularProbability,
    SpecularProbabilityTexture,
    SpecularRoughness,
    SpecularRoughnessTexture,
    SpecularColour,
    SpecularColourTexture,
    TransmissiveProbability,
    TransmissiveProbabilityTexture,
    TransmissiveRoughness,
    TransmissiveRoughnessTexture,
    ExtinctionCoefficient,
    TransmissiveColour,
    TransmissiveColourTexture,
    EmissiveIntensity,
    EmissiveColour,
    EmissiveColourTexture,
    RefractiveIndex,
    RefractiveIndexTexture,
    ScatteringCoefficient,
    ScatteringColour,
    ScatteringColourTexture,
}

impl Enumerator for MaterialInputData {}

impl NodeInputData for MaterialInputData {
    fn default_data(&self) -> InputData {
        let default_material = Material::default();
        match self {
            Self::DiffuseColour => InputData::Vec3(default_material.diffuse_colour),
            Self::DiffuseColourTexture => InputData::RenderPass(RenderPasses::White),
            Self::SpecularProbability => InputData::Float(default_material.specular_probability),
            Self::SpecularProbabilityTexture => InputData::RenderPass(RenderPasses::White),
            Self::SpecularRoughness => InputData::Float(default_material.specular_roughness),
            Self::SpecularRoughnessTexture => InputData::RenderPass(RenderPasses::White),
            Self::SpecularColour => InputData::Vec3(default_material.specular_colour),
            Self::SpecularColourTexture => InputData::RenderPass(RenderPasses::White),
            Self::TransmissiveProbability => {
                InputData::Float(default_material.transmissive_probability)
            }
            Self::TransmissiveProbabilityTexture => InputData::RenderPass(RenderPasses::White),
            Self::TransmissiveRoughness => {
                InputData::Float(default_material.transmissive_roughness)
            }
            Self::TransmissiveRoughnessTexture => InputData::RenderPass(RenderPasses::White),
            Self::ExtinctionCoefficient => {
                InputData::Float(default_material.extinction_coefficient)
            }
            Self::TransmissiveColour => InputData::Vec3(default_material.transmissive_colour),
            Self::TransmissiveColourTexture => InputData::RenderPass(RenderPasses::White),
            Self::EmissiveIntensity => InputData::Float(default_material.emissive_intensity),
            Self::EmissiveColour => InputData::Vec3(default_material.emissive_colour),
            Self::EmissiveColourTexture => InputData::RenderPass(RenderPasses::White),
            Self::RefractiveIndex => InputData::Float(default_material.refractive_index),
            Self::RefractiveIndexTexture => InputData::RenderPass(RenderPasses::White),
            Self::ScatteringCoefficient => {
                InputData::Float(default_material.scattering_coefficient)
            }
            Self::ScatteringColour => InputData::Vec3(default_material.scattering_colour),
            Self::ScatteringColourTexture => InputData::RenderPass(RenderPasses::White),
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
pub enum MaterialOutputData {
    #[default]
    SceneGraph,
}

impl Enumerator for MaterialOutputData {}

impl NodeOutputData for MaterialOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::SceneGraph => OutputData::SceneGraph,
        }
    }
}

pub struct MaterialNode;

impl EvaluableNode for MaterialNode {
    type Inputs = MaterialInputData;
    type Outputs = MaterialOutputData;
}
