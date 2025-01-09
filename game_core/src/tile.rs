use core::hash;
use std::{hash::Hash, mem::discriminant};

#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd, Ord, Eq)]
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
impl Hash for TileType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_isize(*self as isize);
    }
}
