use std::collections::HashMap;

use tracing::debug;

use crate::{chunk::Chunk, terrain_generator::terrain_generator::TerrainGenerator, tile::Tile};
#[derive(Debug)]
pub struct Map {
    generator: TerrainGenerator,
    chunks: HashMap<(isize, isize), Chunk>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            generator: TerrainGenerator::new(64, None)
                .set_lacunarity(2.0)
                .set_persistance(0.5)
                .set_octaves(8)
                .set_scale(1.0),
            chunks: HashMap::new(),
        }
    }
}
impl Map {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn get_chunk(&mut self, pos: (isize, isize)) -> Chunk {
        debug!("Getting chunk {pos:#?}");
        if let std::collections::hash_map::Entry::Vacant(e) = self.chunks.entry(pos) {
            debug!("Generating chunk {pos:#?}");
            e.insert(self.generator.generate_chunk(pos));

            self.get_chunk(pos)
        } else {
            self.chunks.get(&pos).unwrap().clone()
        }
    }

    #[must_use]
    pub fn get_chunk_size(&self) -> usize {
        self.generator.chunk_size
    }

    pub fn get_tile(&mut self, position: (isize, isize)) -> Tile {
        let (xc, yc) = (
            position.0 % self.get_chunk_size() as isize,
            position.1 % self.get_chunk_size() as isize,
        );
        self.get_chunk((xc, yc)).tiles[2][2].clone()
    }
}
