use crate::tile::{Tile, TileType};

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    /// Actual tiles of the chunk
    pub tiles: Vec<Vec<Tile>>,
    /// Average tile type, avoids computing it every frame
    pub average_tile_type: TileType,
}
