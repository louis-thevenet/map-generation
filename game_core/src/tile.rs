#[derive(Debug, Copy, Clone, Default)]
pub enum TileType {
    #[default]
    Water,
    Beach,
    Land,
    Mountain,
}
#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
}
