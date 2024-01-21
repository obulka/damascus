use crevice::std140::AsStd140;
use strum::{Display, EnumIter, EnumString};

#[derive(
    Debug,
    Default,
    Display,
    Copy,
    Clone,
    EnumIter,
    EnumString,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum ProceduralTextureType {
    #[default]
    None,
    Grade,
    Checkerboard,
    // FBMNoise,
    // TurbulenceNoise,
    // VoronoiNoise,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd140)]
pub struct GPUProceduralTexture {
    texture_type: u32,
    black_point: f32,
    white_point: f32,
    lift: f32,
    gamma: f32,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProceduralTexture {
    pub texture_type: ProceduralTextureType,
    pub black_point: f32,
    pub white_point: f32,
    pub lift: f32,
    pub gamma: f32,
}

impl Default for ProceduralTexture {
    fn default() -> Self {
        Self {
            texture_type: ProceduralTextureType::None,
            black_point: 0.,
            white_point: 1.,
            lift: 0.,
            gamma: 1.,
        }
    }
}

impl ProceduralTexture {
    pub fn to_gpu(&self) -> Std140GPUProceduralTexture {
        GPUProceduralTexture {
            texture_type: self.texture_type as u32,
            black_point: self.black_point,
            white_point: self.white_point,
            lift: self.lift,
            gamma: self.gamma,
        }
        .as_std140()
    }
}
