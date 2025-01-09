use std::collections::HashMap;

use tracing::debug;

use crate::{
    chunk::Chunk,
    terrain_generator::{
        noise_to_map::{Layer, NoiseToMap},
        terrain_generator::TerrainGenerator,
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
            generator: TerrainGenerator::new(64, None)
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
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn get_chunk(&mut self, pos: (isize, isize)) -> Chunk {
        // debug!("Getting chunk {pos:#?}");
        if let std::collections::hash_map::Entry::Vacant(e) = self.chunks.entry(pos) {
            // debug!("Generating chunk {pos:#?}");
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
        let chunk_size = self.get_chunk_size() as isize;
        let (xc, yc) = (position.0 % chunk_size, position.1 % chunk_size);
        let mut x_offset = position.0 / chunk_size;
        let mut y_offset = position.1 / chunk_size;

        if position.0 < 0 {
            x_offset = chunk_size - x_offset;
        }
        if position.1 < 0 {
            y_offset = chunk_size - y_offset;
        }
        // debug!("Position {position:#?}: Chunk {xc},{yc}, cell {x_offset},{y_offset}");
        self.get_chunk((xc, yc)).tiles[y_offset as usize][x_offset as usize].clone()
    }
}
