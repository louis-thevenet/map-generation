use crate::{
    chunk::Chunk,
    tile::{Tile, TileType},
};
use strum::EnumCount;

use super::{TEMPERATURE_LOWER_BOUND, TEMPERATURE_UPPER_BOUND};

#[derive(Default, Debug)]
pub struct Layer {
    pub treshold: f64,
    pub tile_type: TileType,
}
#[derive(Default, Debug)]
/// Settings for the image to create.
pub struct NoiseToMap {
    layers: Vec<Layer>,
}
impl NoiseToMap {
    #[must_use]
    /// Adds a new layer to the `NoiseToImage`
    pub fn add_layer(mut self, layer: Layer) -> Self {
        self.layers.push(layer);
        self.layers
            .sort_by(|l1, l2| l2.treshold.total_cmp(&l1.treshold));
        self
    }
    #[must_use]
    pub fn chunk_from_noise(
        &self,
        terrain_noise: &[Vec<f64>],
        temperature_noise: &[Vec<f64>],
    ) -> Chunk {
        let mut tiles = vec![vec![]; terrain_noise.len()];
        let mut freq = [0; TileType::COUNT];

        let mut temp_sum = 0.0;
        let mut temp_count = 0;
        for y in 0..terrain_noise.len() {
            for x in 0..terrain_noise[y].len() {
                let noise = (terrain_noise[y][x] + 1.) / 2.;

                let mut tile_type = self
                    .layers
                    .last()
                    .unwrap_or(&Layer {
                        treshold: 0.0,
                        tile_type: TileType::default(),
                    })
                    .tile_type;

                // Find the layer that fits the noise
                for layer in &self.layers {
                    if noise >= layer.treshold {
                        tile_type = layer.tile_type;
                        break;
                    }
                }
                freq[tile_type as usize] += 1;

                let temperature = (TEMPERATURE_UPPER_BOUND - TEMPERATURE_LOWER_BOUND)
                    * (1.0 - (temperature_noise[y][x] + 1.) / 2.)
                    + TEMPERATURE_LOWER_BOUND;

                temp_sum += temperature;
                temp_count += 1;

                tiles[y].push(Tile {
                    tile_type,
                    temperature,
                });
            }
        }
        let average_tile = TileType::from_repr(
            freq.iter()
                .enumerate()
                .max_by(|(_, value0), (_, value1)| value0.cmp(value1))
                .map(|(idx, _)| idx)
                .unwrap_or_default(),
        )
        .unwrap_or_default();
        let average_temperature = temp_sum / temp_count as f64;
        Chunk {
            tiles,
            average_tile_type: average_tile,
            average_temperature,
        }
    }
}
