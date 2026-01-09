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
pub enum HeightLevel {
    DeepWater,
    ShallowWater,
    Beach,
    Land,
    Hills,
    Mountains,
}

#[derive(Copy, Clone)]
pub struct BiomeSettings {
    pub temperature: TemperatureLevel,
    pub moisture: MoistureLevel,
    pub height: HeightLevel,
}

impl BiomeSettings {
    pub fn new(temp: f64, moisture: f64, height: f64) -> Self {
        let temperature = match temp {
            -2.0..=-0.6 => TemperatureLevel::Freezing,
            -0.6..=-0.3 => TemperatureLevel::Cold,
            -0.3..=0.3 => TemperatureLevel::Temperate,
            0.3..=0.6 => TemperatureLevel::Warm,
            _ => TemperatureLevel::Hot,
        };

        let moisture = match moisture {
            -2.0..=-0.8 => MoistureLevel::Arid,
            -0.8..=-0.4 => MoistureLevel::Dry,
            -0.4..=0.4 => MoistureLevel::Moderate,
            0.4..=0.8 => MoistureLevel::Wet,
            _ => MoistureLevel::Rainforest,
        };

        let height = match height {
            h if h < -0.3 => HeightLevel::DeepWater,
            h if h < -0.05 => HeightLevel::ShallowWater,
            h if h < 0.05 => HeightLevel::Beach,
            h if h < 0.4 => HeightLevel::Land,
            h if h < 0.7 => HeightLevel::Hills,
            _ => HeightLevel::Mountains,
        };

        Self {
            temperature,
            moisture,
            height,
        }
    }
}

enum IntermediateBiomeType {
    // Water biomes
    DeepWater,
    ShallowWater,
    // Water-land biomes
    Beach,
    // Land biomes
    Plain,
    Hills,
    Mountains,
}

struct IntermediateBiome {
    biome_type: IntermediateBiomeType,
    temperature: TemperatureLevel,
    moisture: MoistureLevel,
}

impl From<BiomeSettings> for IntermediateBiomeType {
    fn from(settings: BiomeSettings) -> Self {
        match settings.height {
            HeightLevel::DeepWater => IntermediateBiomeType::DeepWater,
            HeightLevel::ShallowWater => IntermediateBiomeType::ShallowWater,
            HeightLevel::Beach => IntermediateBiomeType::Beach,
            HeightLevel::Land => IntermediateBiomeType::Plain,
            HeightLevel::Hills => IntermediateBiomeType::Hills,
            HeightLevel::Mountains => IntermediateBiomeType::Mountains,
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
    Mountains,
    IceMountains,
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

            BiomeType::Plain => [27, 250, 103],
            BiomeType::Savanna => [255, 177, 60],
            BiomeType::Forest => [1, 191, 32],
            BiomeType::TropicalRainforest => [25, 191, 1],

            BiomeType::Desert => [245, 232, 90],
            BiomeType::Taiga => [255, 255, 255],

            BiomeType::Lake => [71, 211, 255],
            BiomeType::Hills => [73, 196, 143],
            BiomeType::Dunes => [209, 170, 18],
            BiomeType::Mountains => [179, 178, 177],
            BiomeType::IceMountains => [234, 239, 240],
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
                | TemperatureLevel::Hot => BiomeType::Mountains,
            },
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
