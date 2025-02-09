#[derive(Debug, Clone, Default)]
pub struct Chunk {
    pub temp: f64,
    pub moisture: f64,
    pub continentalness: f64,
    pub erosion: f64,
    // /// Actual tiles of the chunk
    // pub tiles: Vec<Vec<Tile>>,
    // /// Average tile type, avoids computing it every frame
    // pub average_tile_type: TileType,
    // pub average_temperature: f64,
}
