#[derive(Copy, Clone)]
pub enum TemperatureLevel {
    Freezing,
    Cold,
    Temperate,
    Warm,
    Hot,
}

#[derive(Copy, Clone)]
pub enum MoistureLevel {
    Arid,
    Dry,
    Moderate,
    Wet,
    Rainforest,
}

#[derive(Copy, Clone)]
pub enum ContinentalnessLevel {
    DeepWater,
    ShallowWater,
    Beach,
    Land,
    Farland,
}

#[derive(Copy, Clone)]
pub enum ErosionLevel {
    Flat,
    SlightHills,
    Hills,
    Mountains,
}

#[derive(Copy, Clone)]
pub struct BiomeSettings {
    pub temperature: TemperatureLevel,
    pub moisture: MoistureLevel,
    pub continentalness: ContinentalnessLevel,
    pub erosion: ErosionLevel,
}

impl BiomeSettings {
    pub fn new(temp: f64, moisture: f64, continentalness: f64, erosion: f64) -> Self {
        // -2 is a temp fix because the noise generator can generate values outside [-1,1]
        let temperature = match temp {
            -2.0..=-0.8 => TemperatureLevel::Freezing,
            -0.8..=-0.4 => TemperatureLevel::Cold,
            -0.4..=0.4 => TemperatureLevel::Temperate,
            0.4..=0.8 => TemperatureLevel::Warm,
            _ => TemperatureLevel::Hot,
        };

        let moisture = match moisture {
            -2.0..=-0.8 => MoistureLevel::Arid,
            -0.8..=-0.4 => MoistureLevel::Dry,
            -0.4..=0.4 => MoistureLevel::Moderate,
            0.4..=0.8 => MoistureLevel::Wet,
            _ => MoistureLevel::Rainforest,
        };

        let continentalness = match continentalness {
            -2.0..=-0.8 => ContinentalnessLevel::DeepWater,
            -0.8..=0.0 => ContinentalnessLevel::ShallowWater,
            0.0..=0.4 => ContinentalnessLevel::Beach,
            0.4..=0.8 => ContinentalnessLevel::Land,
            _ => ContinentalnessLevel::Farland,
        };

        let erosion = match erosion {
            -2.0..=-0.6 => ErosionLevel::Flat,
            -0.6..=0.2 => ErosionLevel::SlightHills,
            -0.2..=0.6 => ErosionLevel::Hills,
            _ => ErosionLevel::Mountains,
        };

        Self {
            temperature,
            moisture,
            continentalness,
            erosion,
        }
    }
}

enum IntermediateBiomeType {
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
    HighMountains,
}

struct IntermediateBiome {
    biome_type: IntermediateBiomeType,
    temperature: TemperatureLevel,
    moisture: MoistureLevel,
}
impl From<BiomeSettings> for IntermediateBiomeType {
    fn from(settings: BiomeSettings) -> Self {
        match settings.continentalness {
            ContinentalnessLevel::DeepWater => IntermediateBiomeType::DeepWater,

            ContinentalnessLevel::ShallowWater => match settings.erosion {
                ErosionLevel::Flat | ErosionLevel::SlightHills | ErosionLevel::Hills => {
                    IntermediateBiomeType::ShallowWater
                }
                ErosionLevel::Mountains => IntermediateBiomeType::Cliff,
            },
            ContinentalnessLevel::Land => match settings.erosion {
                ErosionLevel::Flat | ErosionLevel::SlightHills => IntermediateBiomeType::Plain,
                ErosionLevel::Hills => IntermediateBiomeType::Hills,
                ErosionLevel::Mountains => IntermediateBiomeType::HighMountains,
            },
            ContinentalnessLevel::Farland => match settings.erosion {
                ErosionLevel::Flat | ErosionLevel::SlightHills => IntermediateBiomeType::Plain,
                ErosionLevel::Hills => IntermediateBiomeType::Plateau,

                ErosionLevel::Mountains => IntermediateBiomeType::HighMountains,
            },
            ContinentalnessLevel::Beach => IntermediateBiomeType::Beach,
        }
    }
}

#[derive(Clone, Debug)]
pub enum BiomeType {
    // Water biomes
    ArcticWater,
    DeepTemperateWater,
    DeepTropicalWater,

    ShallowTemperateWater,
    ShallowTropicalWater,
    ShallowIceWater,

    // Water-land biomes
    TropicalBeach,
    TemperateBeach,
    IceField,
    Cliff,
    // Land biomes
    Plain,
    Savanna,
    Forest,
    TropicalRainforest,

    Desert,
    Taiga,

    Lake,
    Hills,
    Dunes,
    Montains,
    IceMountains,

    // Farland biomes
    Plateau,
    HighMountains,
}
impl BiomeType {
    pub fn color(&self) -> [u8; 3] {
        match self {
            BiomeType::ArcticWater => [204, 229, 255],
            BiomeType::DeepTemperateWater => [0, 46, 240],
            BiomeType::DeepTropicalWater => [67, 159, 240],

            BiomeType::ShallowTemperateWater => [83, 115, 247],
            BiomeType::ShallowTropicalWater => [115, 182, 240],
            BiomeType::ShallowIceWater => [139, 243, 255],

            BiomeType::TropicalBeach => [255, 249, 99],
            BiomeType::TemperateBeach => [237, 234, 147],
            BiomeType::IceField => [221, 246, 255],
            BiomeType::Cliff => [183, 187, 196],

            BiomeType::Plain => [27, 250, 103],
            BiomeType::Savanna => [255, 177, 60],
            BiomeType::Forest => [1, 191, 32],
            BiomeType::TropicalRainforest => [25, 191, 1],

            BiomeType::Desert => [245, 232, 90],
            BiomeType::Taiga => [255, 255, 255],

            BiomeType::Lake => [71, 211, 255],
            BiomeType::Hills => [73, 196, 143],
            BiomeType::Dunes => [209, 170, 18],
            BiomeType::Montains => [179, 178, 177],
            BiomeType::IceMountains => [234, 239, 240],

            BiomeType::Plateau => [179, 197, 201],
            BiomeType::HighMountains => [105, 111, 112],
        }
    }
}
impl From<IntermediateBiome> for BiomeType {
    fn from(value: IntermediateBiome) -> Self {
        match value.biome_type {
            IntermediateBiomeType::DeepWater => match value.temperature {
                TemperatureLevel::Freezing => BiomeType::ArcticWater,
                TemperatureLevel::Cold | TemperatureLevel::Temperate => {
                    BiomeType::DeepTemperateWater
                }
                TemperatureLevel::Warm | TemperatureLevel::Hot => BiomeType::DeepTropicalWater,
            },
            IntermediateBiomeType::ShallowWater => match value.temperature {
                TemperatureLevel::Freezing => BiomeType::ShallowIceWater,
                TemperatureLevel::Cold | TemperatureLevel::Temperate => {
                    BiomeType::ShallowTemperateWater
                }
                TemperatureLevel::Warm | TemperatureLevel::Hot => BiomeType::ShallowTropicalWater,
            },
            IntermediateBiomeType::Beach => match value.temperature {
                TemperatureLevel::Warm | TemperatureLevel::Hot => BiomeType::TropicalBeach,
                TemperatureLevel::Cold | TemperatureLevel::Temperate => BiomeType::TemperateBeach,
                TemperatureLevel::Freezing => BiomeType::IceField,
            },
            IntermediateBiomeType::Cliff => BiomeType::Cliff,

            IntermediateBiomeType::Plain => match value.temperature {
                TemperatureLevel::Freezing => BiomeType::Taiga,
                TemperatureLevel::Cold | TemperatureLevel::Temperate => match value.moisture {
                    MoistureLevel::Arid | MoistureLevel::Dry | MoistureLevel::Moderate => {
                        BiomeType::Plain
                    }
                    MoistureLevel::Wet => BiomeType::Forest,
                    MoistureLevel::Rainforest => BiomeType::Lake,
                },

                TemperatureLevel::Warm => match value.moisture {
                    MoistureLevel::Arid | MoistureLevel::Dry => BiomeType::Desert,
                    MoistureLevel::Moderate => BiomeType::Savanna,
                    MoistureLevel::Wet => BiomeType::Forest,
                    MoistureLevel::Rainforest => BiomeType::TropicalRainforest,
                },
                TemperatureLevel::Hot => match value.moisture {
                    MoistureLevel::Arid | MoistureLevel::Dry => BiomeType::Desert,
                    MoistureLevel::Moderate => BiomeType::Savanna,
                    MoistureLevel::Wet => BiomeType::Forest,
                    MoistureLevel::Rainforest => BiomeType::TropicalRainforest,
                },
            },
            IntermediateBiomeType::Hills => match value.temperature {
                TemperatureLevel::Freezing => BiomeType::IceMountains,
                TemperatureLevel::Cold | TemperatureLevel::Temperate => BiomeType::Hills,
                TemperatureLevel::Warm | TemperatureLevel::Hot => BiomeType::Dunes,
            },
            IntermediateBiomeType::Mountains => match value.temperature {
                TemperatureLevel::Freezing => BiomeType::IceMountains,
                TemperatureLevel::Cold
                | TemperatureLevel::Temperate
                | TemperatureLevel::Warm
                | TemperatureLevel::Hot => BiomeType::Montains,
            },

            IntermediateBiomeType::Plateau => BiomeType::Plateau,

            IntermediateBiomeType::HighMountains => BiomeType::HighMountains,
        }
    }
}

impl From<BiomeSettings> for BiomeType {
    fn from(settings: BiomeSettings) -> Self {
        let intermediate_biome = IntermediateBiome {
            biome_type: settings.into(),
            temperature: settings.temperature,
            moisture: settings.moisture,
        };
        intermediate_biome.into()
    }
}
