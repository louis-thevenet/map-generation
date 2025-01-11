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
        let mut chunk = Chunk {
            tiles: vec![vec![]; noise.len()],
        };
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
                chunk.tiles[y].push(Tile { tile_type });
            }
        }
        chunk
    }
}
