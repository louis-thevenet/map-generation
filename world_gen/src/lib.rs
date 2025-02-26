use std::collections::HashMap;

use biome::BiomeSettings;
use city_generation::CityGenerator;
use concrete_cell::{BuildingElement, BuildingPart, ConcreteCell};
use intermediate_cell::IntermediateCell;
use perlin_noise::PerlinNoiseGenerator;
use rand::{thread_rng, RngCore};

mod biome;
pub mod city_generation;
pub mod concrete_cell;
pub mod image_utils;
pub mod intermediate_cell;
mod perlin_noise;
mod vector;
/// So that 1.0 is a good scale
const GLOBAL_SCALE_FIX: f64 = 70.;
#[derive(Debug, Clone)]

pub struct WorldGen {
    pub seed: u64,
    cities: HashMap<(isize, isize), HashMap<(isize, isize), BuildingElement>>,

    temperature_noise: PerlinNoiseGenerator,
    moisture_noise: PerlinNoiseGenerator,
    continentalness_noise: PerlinNoiseGenerator,
    erosion_noise: PerlinNoiseGenerator,
    temp_scale: f64,
    moisture_scale: f64,
    continentalness_scale: f64,
    erosion_scale: f64,
}

impl WorldGen {
    #[must_use]
    pub fn new(global_scale: f64, seed_opt: Option<u64>) -> Self {
        let seed = if let Some(seed_value) = seed_opt {
            seed_value
        } else {
            let seed_value = thread_rng().next_u64();
            println!("Seed: {seed_value}");
            seed_value
        };

        let temp_scale = GLOBAL_SCALE_FIX * global_scale * 64.;
        let moisture_scale = GLOBAL_SCALE_FIX * global_scale * 64.;
        let continentalness_scale = GLOBAL_SCALE_FIX * global_scale * 64.;
        let erosion_scale = GLOBAL_SCALE_FIX * global_scale * 16.;
        Self {
            seed,
            temperature_noise: PerlinNoiseGenerator::new(seed)
                .set_lacunarity(1.3)
                .set_persistence(0.5)
                .set_octaves(4),
            temp_scale,
            moisture_noise: PerlinNoiseGenerator::new(seed + 2)
                .set_lacunarity(1.3)
                .set_persistence(0.5)
                .set_octaves(4),
            moisture_scale,
            continentalness_noise: PerlinNoiseGenerator::new(seed + 4)
                .set_lacunarity(1.7)
                .set_persistence(0.6)
                .set_octaves(8),
            continentalness_scale,
            erosion_noise: PerlinNoiseGenerator::new(seed + 8)
                .set_lacunarity(2.0)
                .set_persistence(0.5)
                .set_octaves(8),
            erosion_scale,
            cities: HashMap::new(),
        }
    }
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn generate_intermediate_cell(&self, pos: (isize, isize), scale: f64) -> IntermediateCell {
        let pos = (pos.0 as f64, pos.1 as f64);
        let temp = self.temperature_noise.noise(pos, self.temp_scale / scale);
        let moisture = self.moisture_noise.noise(pos, self.moisture_scale / scale);
        let continentalness = self
            .continentalness_noise
            .noise(pos, self.continentalness_scale / scale);
        let erosion = self.erosion_noise.noise(pos, self.erosion_scale / scale);

        IntermediateCell {
            temp,
            moisture,
            continentalness,
            erosion,
            biome: BiomeSettings::new(temp, moisture, continentalness, erosion).into(),
        }
    }
    #[must_use]
    pub fn generate_concrete_cell(
        &mut self,
        pos: (isize, isize),
        chunk_pos: (isize, isize),
    ) -> ConcreteCell {
        let intermediate_cell = self.generate_intermediate_cell(pos, 1.0);

        // random city for now
        let cell = self.get_city_cell(pos, chunk_pos);

        let building_part = cell.map(|building_element| BuildingPart {
            element: building_element,
            is_door: false,
        });
        ConcreteCell {
            biome: intermediate_cell.biome,
            building_part,
        }
    }

    fn get_city_cell(
        &mut self,
        pos: (isize, isize),
        chunk_pos: (isize, isize),
    ) -> Option<BuildingElement> {
        if let Some(city) = self.cities.get(&chunk_pos) {
            city.get(&pos).cloned()
        } else {
            let mut city = CityGenerator::new(self.seed, 10..20, 10..20, 30..50, 100);
            city.generate(5, 1, 1);
            self.cities.insert(chunk_pos, city.is_something);
            self.get_city_cell(pos, chunk_pos)
        }
    }
}
