use crate::tile::{Tile, TileType};

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    pub tiles: Vec<Vec<Tile>>,
    pub average_tile: TileType,
}
