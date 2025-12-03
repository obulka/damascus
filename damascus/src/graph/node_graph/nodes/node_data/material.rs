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
    materials::{Material, MaterialId},
};

#[derive(Copy, Default, EnumHashTraits!)]
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

impl NodeInputData for MaterialInputData {
    fn default_data(&self) -> InputData {
        let default_material = Material::default();
        match self {
            Self::DiffuseColour => InputData::Vec3(default_material.diffuse_colour),
            Self::DiffuseColourTexture => InputData::SceneGraphId(SceneGraphId::None),
            Self::SpecularProbability => InputData::Float(default_material.specular_probability),
            Self::SpecularProbabilityTexture => InputData::SceneGraphId(SceneGraphId::None),
            Self::SpecularRoughness => InputData::Float(default_material.specular_roughness),
            Self::SpecularRoughnessTexture => InputData::SceneGraphId(SceneGraphId::None),
            Self::SpecularColour => InputData::Vec3(default_material.specular_colour),
            Self::SpecularColourTexture => InputData::SceneGraphId(SceneGraphId::None),
            Self::TransmissiveProbability => {
                InputData::Float(default_material.transmissive_probability)
            }
            Self::TransmissiveProbabilityTexture => InputData::SceneGraphId(SceneGraphId::None),
            Self::TransmissiveRoughness => {
                InputData::Float(default_material.transmissive_roughness)
            }
            Self::TransmissiveRoughnessTexture => InputData::SceneGraphId(SceneGraphId::None),
            Self::ExtinctionCoefficient => {
                InputData::Float(default_material.extinction_coefficient)
            }
            Self::TransmissiveColour => InputData::Vec3(default_material.transmissive_colour),
            Self::TransmissiveColourTexture => InputData::SceneGraphId(SceneGraphId::None),
            Self::EmissiveIntensity => InputData::Float(default_material.emissive_intensity),
            Self::EmissiveColour => InputData::Vec3(default_material.emissive_colour),
            Self::EmissiveColourTexture => InputData::SceneGraphId(SceneGraphId::None),
            Self::RefractiveIndex => InputData::Float(default_material.refractive_index),
            Self::RefractiveIndexTexture => InputData::SceneGraphId(SceneGraphId::None),
            Self::ScatteringCoefficient => {
                InputData::Float(default_material.scattering_coefficient)
            }
            Self::ScatteringColour => InputData::Vec3(default_material.scattering_colour),
            Self::ScatteringColourTexture => InputData::SceneGraphId(SceneGraphId::None),
        }
    }
}

#[derive(Copy, Default, EnumHashTraits!)]
pub enum MaterialOutputData {
    #[default]
    Id,
}

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

    fn output_data_is_compatible_with_input(
        output_data: &OutputData,
        input: &Self::Inputs,
    ) -> bool {
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
                *output_data == OutputData::SceneGraphId(SceneGraphIdType::TextureEvaluator)
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
        let material_id: MaterialId = scene_graph.add_material(Material {
            diffuse_colour: Self::Inputs::DiffuseColour
                .from_data_map(data_map)?
                .try_to_vec3()?,
            diffuse_colour_texture_id: Self::Inputs::DiffuseColourTexture
                .from_data_map(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            specular_probability: Self::Inputs::SpecularProbability
                .from_data_map(data_map)?
                .try_to_float()?,
            specular_probability_texture_id: Self::Inputs::SpecularProbabilityTexture
                .from_data_map(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            specular_roughness: Self::Inputs::SpecularRoughness
                .from_data_map(data_map)?
                .try_to_float()?,
            specular_roughness_texture_id: Self::Inputs::SpecularRoughnessTexture
                .from_data_map(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            specular_colour: Self::Inputs::SpecularColour
                .from_data_map(data_map)?
                .try_to_vec3()?,
            specular_colour_texture_id: Self::Inputs::SpecularColourTexture
                .from_data_map(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            transmissive_probability: Self::Inputs::TransmissiveProbability
                .from_data_map(data_map)?
                .try_to_float()?,
            transmissive_probability_texture_id: Self::Inputs::TransmissiveProbabilityTexture
                .from_data_map(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            transmissive_roughness: Self::Inputs::TransmissiveRoughness
                .from_data_map(data_map)?
                .try_to_float()?,
            transmissive_roughness_texture_id: Self::Inputs::TransmissiveRoughnessTexture
                .from_data_map(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            extinction_coefficient: Self::Inputs::ExtinctionCoefficient
                .from_data_map(data_map)?
                .try_to_float()?,
            transmissive_colour: Self::Inputs::TransmissiveColour
                .from_data_map(data_map)?
                .try_to_vec3()?,
            transmissive_colour_texture_id: Self::Inputs::TransmissiveColourTexture
                .from_data_map(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            emissive_intensity: Self::Inputs::EmissiveIntensity
                .from_data_map(data_map)?
                .try_to_float()?,
            emissive_colour: Self::Inputs::EmissiveColour
                .from_data_map(data_map)?
                .try_to_vec3()?,
            emissive_colour_texture_id: Self::Inputs::EmissiveColourTexture
                .from_data_map(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            refractive_index: Self::Inputs::RefractiveIndex
                .from_data_map(data_map)?
                .try_to_float()?,
            refractive_index_texture_id: Self::Inputs::RefractiveIndexTexture
                .from_data_map(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
            scattering_coefficient: Self::Inputs::ScatteringCoefficient
                .from_data_map(data_map)?
                .try_to_float()?,
            scattering_colour: Self::Inputs::ScatteringColour
                .from_data_map(data_map)?
                .try_to_vec3()?,
            scattering_colour_texture_id: Self::Inputs::ScatteringColourTexture
                .from_data_map(data_map)?
                .try_to_texture_evaluator_id()
                .ok(),
        });
        let scene_graph_id = SceneGraphId::Material(material_id);

        match output {
            Self::Outputs::Id => Ok(InputData::SceneGraphId(scene_graph_id)),
        }
    }
}
