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
pub enum AOVs {
    #[default]
    Beauty,
    WorldPosition,
    LocalPosition,
    Normals,
    Depth,
    Stats,
}

mod ray_marcher;
pub use ray_marcher::{RayMarcher, Std430GPURayMarcher};
