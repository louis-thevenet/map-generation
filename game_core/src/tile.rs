use std::hash::Hash;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(
    Debug, Copy, Clone, Default, PartialEq, PartialOrd, Ord, Eq, EnumCount, EnumIter, FromRepr,
)]
pub enum TileType {
    #[default]
    Water,
    Beach,
    Land,
    Mountain,
}

impl Hash for TileType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_isize(*self as isize);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Tile {
    pub tile_type: TileType,
    pub temperature: f64,
}
