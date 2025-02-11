use crate::biome::BiomeType;

#[derive(Debug, Clone)]
pub struct Cell {
    pub temp: f64,
    pub moisture: f64,
    pub continentalness: f64,
    pub erosion: f64,
    pub biome: BiomeType,
}
