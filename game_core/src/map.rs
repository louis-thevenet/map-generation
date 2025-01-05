use std::collections::HashMap;

use crate::{chunk::Chunk, terrain_generator::terrain_generator::TerrainGenerator};

#[derive(Debug)]
pub struct Map {
    generator: TerrainGenerator,
    chunks: HashMap<(usize, usize), Chunk>,
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
    pub fn get_chunk(&mut self, pos: (usize, usize)) -> Chunk {
        if let std::collections::hash_map::Entry::Vacant(e) = self.chunks.entry(pos) {
            e.insert(self.generator.generate_chunk(pos));

            self.get_chunk(pos)
        } else {
            self.chunks.get(&pos).unwrap().clone()
        }
    }
}
