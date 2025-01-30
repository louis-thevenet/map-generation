use strum::EnumCount;

use crate::{
    chunk::Chunk,
    tile::{Tile, TileType},
};

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
    pub fn chunk_from_noise(&self, noise: &[Vec<f64>]) -> Chunk {
        let mut tiles = vec![vec![]; noise.len()];
        let mut freq = [0; TileType::COUNT];
        for y in 0..noise.len() {
            for x in 0..noise[y].len() {
                let noise = (noise[y][x] + 1.) / 2.;

                let mut tile_type = self
                    .layers
                    .last()
                    .unwrap_or(&Layer {
                        treshold: 0.0,
                        tile_type: TileType::default(),
                    })
                    .tile_type;

                for layer in &self.layers {
                    if noise >= layer.treshold {
                        tile_type = layer.tile_type;
                        break;
                    }
                }
                freq[tile_type as usize] += 1;
                tiles[y].push(Tile { tile_type });
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
        Chunk {
            tiles,
            average_tile_type: average_tile,
        }
    }
}
