pub enum TemperatureLevel {
    Freezing,
    Cold,
    Temperate,
    Warm,
    Hot,
}

pub enum MoistureLevel {
    Arid,
    Dry,
    Moderate,
    Wet,
    Rainforest,
}

pub enum ContinentalnessLevel {
    DeepWater,
    ShallowWater,
    Beach,
    Land,
    Farland,
}

pub enum ErosionLevel {
    Flat,
    Hills,
    Mountains,
}

pub struct BiomeSettings {
    pub temperature: TemperatureLevel,
    pub moisture: MoistureLevel,
    pub continentalness: ContinentalnessLevel,
    pub erosion: ErosionLevel,
}

pub enum BiomeType {
    // Water biomes
    DeepWater,
    ShallowWater,
    // Water-land biomes
    Beach,
    Cliff,
    // Land biomes
    Plain,
    Hills,
    Mountains,

    // Farland biomes
    Plateau,
    HighMontains,
}
pub struct Biome {
    pub biome_type: BiomeType,
    pub temperature: TemperatureLevel,
    pub moisture: MoistureLevel,
}

impl From<BiomeSettings> for Biome {
    fn from(settings: BiomeSettings) -> Self {
        let biome_type = match settings.continentalness {
            ContinentalnessLevel::DeepWater => BiomeType::DeepWater,

            ContinentalnessLevel::ShallowWater => BiomeType::ShallowWater,
            ContinentalnessLevel::Beach => match settings.erosion {
                ErosionLevel::Flat => BiomeType::Beach,
                ErosionLevel::Hills | ErosionLevel::Mountains => BiomeType::Cliff,
            },
            ContinentalnessLevel::Land => match settings.erosion {
                ErosionLevel::Flat => BiomeType::Plain,
                ErosionLevel::Hills => BiomeType::Hills,

                ErosionLevel::Mountains => BiomeType::Mountains,
            },
            ContinentalnessLevel::Farland => match settings.erosion {
                ErosionLevel::Flat => BiomeType::Plateau,
                ErosionLevel::Hills => BiomeType::Mountains,
                ErosionLevel::Mountains => BiomeType::HighMontains,
            },
        };
        Self {
            biome_type,
            temperature: settings.temperature,
            moisture: settings.moisture,
        }
    }
}
