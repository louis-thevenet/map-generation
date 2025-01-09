use crate::{
    chunk::Chunk,
    tile::{Tile, TileType},
};

#[derive(Default, Debug)]
pub struct Layer {
    pub treshold: f64,
    pub color: TileType,
}
#[derive(Default, Debug)]
/// Settings for the image to create.
pub struct NoiseToMap {
    layers: Vec<Layer>,
}
impl NoiseToMap {
    pub fn chunk_from_noise(&self, noise: Vec<Vec<f64>>) -> Chunk {
        let mut chunk = Chunk::default();
        chunk.tiles = vec![vec![]; noise.len()];
        for y in 0..noise.len() {
            for x in 0..noise[y].len() {
                let noise = (noise[y][x] + 1.) / 2.;

                let mut tile_type = self
                    .layers
                    .last()
                    .unwrap_or(&Layer {
                        treshold: 0.0,
                        color: TileType::default(),
                    })
                    .color;

                for layer in &self.layers {
                    if noise >= layer.treshold {
                        tile_type = layer.color;
                        break;
                    }
                }
                chunk.tiles[y].push(Tile { tile_type });
            }
        }
        chunk
    }
}
