// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    Enumerator,
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::NodeResult,
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{SceneGraph, SceneGraphId, SceneGraphIdType},
    },
    materials::{Material, MaterialId},
    textures::evaluators::TextureEvaluator,
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
            Self::DiffuseColourTexture => InputData::TextureEvaluator(TextureEvaluator::White),
            Self::SpecularProbability => InputData::Float(default_material.specular_probability),
            Self::SpecularProbabilityTexture => {
                InputData::TextureEvaluator(TextureEvaluator::White)
            }
            Self::SpecularRoughness => InputData::Float(default_material.specular_roughness),
            Self::SpecularRoughnessTexture => InputData::TextureEvaluator(TextureEvaluator::White),
            Self::SpecularColour => InputData::Vec3(default_material.specular_colour),
            Self::SpecularColourTexture => InputData::TextureEvaluator(TextureEvaluator::White),
            Self::TransmissiveProbability => {
                InputData::Float(default_material.transmissive_probability)
            }
            Self::TransmissiveProbabilityTexture => {
                InputData::TextureEvaluator(TextureEvaluator::White)
            }
            Self::TransmissiveRoughness => {
                InputData::Float(default_material.transmissive_roughness)
            }
            Self::TransmissiveRoughnessTexture => {
                InputData::TextureEvaluator(TextureEvaluator::White)
            }
            Self::ExtinctionCoefficient => {
                InputData::Float(default_material.extinction_coefficient)
            }
            Self::TransmissiveColour => InputData::Vec3(default_material.transmissive_colour),
            Self::TransmissiveColourTexture => InputData::TextureEvaluator(TextureEvaluator::White),
            Self::EmissiveIntensity => InputData::Float(default_material.emissive_intensity),
            Self::EmissiveColour => InputData::Vec3(default_material.emissive_colour),
            Self::EmissiveColourTexture => InputData::TextureEvaluator(TextureEvaluator::White),
            Self::RefractiveIndex => InputData::Float(default_material.refractive_index),
            Self::RefractiveIndexTexture => InputData::TextureEvaluator(TextureEvaluator::White),
            Self::ScatteringCoefficient => {
                InputData::Float(default_material.scattering_coefficient)
            }
            Self::ScatteringColour => InputData::Vec3(default_material.scattering_colour),
            Self::ScatteringColourTexture => InputData::TextureEvaluator(TextureEvaluator::White),
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
    Id,
}

impl Enumerator for MaterialOutputData {}

impl NodeOutputData for MaterialOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Id => OutputData::SceneGraphId(SceneGraphIdType::Material),
        }
    }
}

pub struct MaterialNode;

impl EvaluableNode for MaterialNode {
    type Inputs = MaterialInputData;
    type Outputs = MaterialOutputData;

    fn output_is_compatible_with_input(output: &OutputData, input: &Self::Inputs) -> bool {
        match input {
            Self::Inputs::DiffuseColourTexture
            | Self::Inputs::SpecularProbabilityTexture
            | Self::Inputs::SpecularRoughnessTexture
            | Self::Inputs::SpecularColourTexture
            | Self::Inputs::TransmissiveProbabilityTexture
            | Self::Inputs::TransmissiveRoughnessTexture
            | Self::Inputs::TransmissiveColourTexture
            | Self::Inputs::EmissiveColourTexture
            | Self::Inputs::RefractiveIndexTexture
            | Self::Inputs::ScatteringColourTexture => {
                *output == OutputData::SceneGraphId(SceneGraphIdType::TextureEvaluator)
            }
            _ => false,
        }
    }

    fn evaluate(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        let material_id: MaterialId = scene_graph.add_material(Material {
            diffuse_colour: Self::Inputs::DiffuseColour
                .get_data(data_map)?
                .try_to_vec3()?,
            diffuse_colour_texture_id: Self::Inputs::DiffuseColourTexture
                .get_data(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            specular_probability: Self::Inputs::SpecularProbability
                .get_data(data_map)?
                .try_to_float()?,
            specular_probability_texture_id: Self::Inputs::SpecularProbabilityTexture
                .get_data(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            specular_roughness: Self::Inputs::SpecularRoughness
                .get_data(data_map)?
                .try_to_float()?,
            specular_roughness_texture_id: Self::Inputs::SpecularRoughnessTexture
                .get_data(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            specular_colour: Self::Inputs::SpecularColour
                .get_data(data_map)?
                .try_to_vec3()?,
            specular_colour_texture_id: Self::Inputs::SpecularColourTexture
                .get_data(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            transmissive_probability: Self::Inputs::TransmissiveProbability
                .get_data(data_map)?
                .try_to_float()?,
            transmissive_probability_texture_id: Self::Inputs::TransmissiveProbabilityTexture
                .get_data(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            transmissive_roughness: Self::Inputs::TransmissiveRoughness
                .get_data(data_map)?
                .try_to_float()?,
            transmissive_roughness_texture_id: Self::Inputs::TransmissiveRoughnessTexture
                .get_data(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            extinction_coefficient: Self::Inputs::ExtinctionCoefficient
                .get_data(data_map)?
                .try_to_float()?,
            transmissive_colour: Self::Inputs::TransmissiveColour
                .get_data(data_map)?
                .try_to_vec3()?,
            transmissive_colour_texture_id: Self::Inputs::TransmissiveColourTexture
                .get_data(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            emissive_intensity: Self::Inputs::EmissiveIntensity
                .get_data(data_map)?
                .try_to_float()?,
            emissive_colour: Self::Inputs::EmissiveColour
                .get_data(data_map)?
                .try_to_vec3()?,
            emissive_colour_texture_id: Self::Inputs::EmissiveColourTexture
                .get_data(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            refractive_index: Self::Inputs::RefractiveIndex
                .get_data(data_map)?
                .try_to_float()?,
            refractive_index_texture_id: Self::Inputs::RefractiveIndexTexture
                .get_data(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            scattering_coefficient: Self::Inputs::ScatteringCoefficient
                .get_data(data_map)?
                .try_to_float()?,
            scattering_colour: Self::Inputs::ScatteringColour
                .get_data(data_map)?
                .try_to_vec3()?,
            scattering_colour_texture_id: Self::Inputs::ScatteringColourTexture
                .get_data(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
        });
        let scene_graph_id = SceneGraphId::Material(material_id);

        match output {
            Self::Outputs::Id => Ok(InputData::SceneGraphId(scene_graph_id)),
        }
    }
}
