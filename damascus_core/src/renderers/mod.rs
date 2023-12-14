#[derive(Debug, Default, Copy, Clone, FromPrimitive)]
pub enum AOVs {
    #[default]
    Beauty,
    WorldPosition,
    LocalPosition,
    Normals,
    Depth,
    // TODO heatmap/stats
}

mod ray_marcher;
pub use ray_marcher::{RayMarcher, Std140RenderParameters};
