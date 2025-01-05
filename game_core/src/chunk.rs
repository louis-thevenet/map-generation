use crate::tile::Tile;

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    pub tiles: Vec<Vec<Tile>>,
}
