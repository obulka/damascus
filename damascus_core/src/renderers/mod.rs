#[derive(Debug, Default, Copy, Clone, FromPrimitive, serde::Serialize, serde::Deserialize)]
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
pub use ray_marcher::{RayMarcher, Std140RenderParameters};
