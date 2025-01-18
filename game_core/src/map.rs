use std::collections::HashMap;

use tracing::error;

use crate::{
    chunk::Chunk,
    terrain_generator::{
        noise_to_map::{Layer, NoiseToMap},
        TerrainGenerator,
    },
    tile::{Tile, TileType},
};
#[derive(Debug)]
pub struct Map {
    generator: TerrainGenerator,
    chunks: HashMap<(isize, isize), Chunk>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            generator: TerrainGenerator::new(16, None)
                .set_lacunarity(2.0)
                .set_persistance(0.5)
                .set_octaves(8)
                .set_scale(1.0)
                .set_noise_to_map(
                    NoiseToMap::default()
                        .add_layer(Layer {
                            treshold: 0.85,
                            tile_type: TileType::Mountain,
                        })
                        .add_layer(Layer {
                            treshold: 0.5,
                            tile_type: TileType::Land,
                        })
                        .add_layer(Layer {
                            treshold: 0.45,
                            tile_type: TileType::Beach,
                        })
                        .add_layer(Layer {
                            treshold: 0.0,
                            tile_type: TileType::Water,
                        }),
                ),
            chunks: HashMap::new(),
        }
    }
}
impl Map {
    #[must_use]
    pub fn new(scale: f64) -> Self {
        let mut res = Self::default();
        res.generator = res.generator.set_scale(scale);
        res
    }
    /// Get a reference to a `Chunk` from its coordinates.
    ///
    /// # Panics
    ///
    /// Shouldn't panic but clippy is screaming at me.
    pub fn get_chunk_from_chunk_coord(&mut self, pos: (isize, isize)) -> &Chunk {
        if let std::collections::hash_map::Entry::Vacant(e) = self.chunks.entry(pos) {
            e.insert(self.generator.generate_chunk(pos));
            self.get_chunk_from_chunk_coord(pos)
        } else {
            self.chunks.get(&pos).unwrap()
        }
    }
    pub fn get_chunk_from_world_coord(&mut self, position: (isize, isize)) -> &Chunk {
        self.get_chunk_from_chunk_coord(self.chunk_coord_from_world_coord(position))
    }

    #[must_use]
    pub const fn get_chunk_size(&self) -> usize {
        self.generator.chunk_size
    }

    pub fn get_tile(&mut self, position: (isize, isize)) -> Tile {
        // Chunk coordinates
        let chunk_size = self.get_chunk_size().try_into().unwrap_or({
            error!("Chunk size too big to cast as a isize.");
            0
        });
        let (chunk_x, chunk_y) = self.chunk_coord_from_world_coord(position);

        let cell_x = {
            let cx = position.0 % chunk_size;
            (cx + chunk_size) % chunk_size
        }
        .unsigned_abs();
        let cell_y = {
            let cy = position.1 % chunk_size;
            (cy + chunk_size) % chunk_size
        }
        .unsigned_abs();

        self.get_chunk_from_chunk_coord((chunk_x, chunk_y)).tiles[cell_y][cell_x].clone()
    }
    #[must_use]
    pub fn chunk_coord_from_world_coord(&self, position: (isize, isize)) -> (isize, isize) {
        let chunk_size = self.get_chunk_size().try_into().unwrap_or({
            error!("Chunk size too big to cast as a isize.");
            0
        });
        let x = if position.0 >= 0 {
            position.0 / chunk_size
        } else {
            (position.0 + 1) / chunk_size - 1
        };
        let y = if position.1 >= 0 {
            position.1 / chunk_size
        } else {
            (position.1 + 1) / chunk_size - 1
        };
        (x, y)
    }
}
