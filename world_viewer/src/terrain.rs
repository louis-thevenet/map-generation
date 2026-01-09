use bevy::prelude::*;
use world_gen::WorldGen;

#[derive(Component)]
pub struct Chunk {
    position: [i32; 2],
    /// The size of the terrain.
    size: [u32; 2],
    resolution: u32,
    wireframe: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            position: [0, 0],
            size: [2, 2],
            resolution: 15,
            wireframe: false,
        }
    }
}

/// Render `Terrain` as a `Mesh3d`
#[derive(Bundle, Default)]
pub struct ChunkBundle {
    /// Terrain configuration
    pub chunk: Chunk,
    /// Generated mesh data
    pub mesh: (Mesh3d, MeshMaterial3d<StandardMaterial>),
}

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                initialize_world_generator,
                initialize_terrain,
                generate_terrain,
            )
                .chain(),
        );
    }
}

#[derive(Resource)]
struct WorldGenRes(WorldGen);
fn initialize_world_generator(mut commands: Commands) {
    let world_gen = WorldGen::new(1.0, Some(1));
    commands.insert_resource(WorldGenRes(world_gen));
}

fn initialize_terrain(mut commands: Commands) {
    // spawn 1 chunk
    commands.spawn(ChunkBundle {
        chunk: Chunk {
            position: [0, 0],
            size: [16, 16],
            resolution: 250,
            wireframe: false,
        },
        ..Default::default()
    });
}

fn generate_terrain(
    world_gen: Res<WorldGenRes>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Chunk, &mut Mesh3d, &MeshMaterial3d<StandardMaterial>)>,
) {
    for (terrain, mut mesh_handle, material) in &mut query {
        if let Some(material) = materials.get_mut(material) {
            *material = StandardMaterial::default();
        }
        let size = [
            terrain.size[0] * terrain.resolution,
            terrain.size[1] * terrain.resolution,
        ];
        let cells = world_gen.0.generate_chunk(terrain.position, size);

        // Collect unique biome types to build gradient
        let mut biome_set = std::collections::HashSet::new();
        for row in &cells {
            for cell in row {
                biome_set.insert(format!("{:?}", cell.biome));
            }
        }

        let mut biomes: Vec<_> = cells
            .iter()
            .flat_map(|row| row.iter())
            .map(|cell| &cell.biome)
            .collect();
        biomes.sort_by_key(|b| format!("{b:?}"));
        biomes.dedup_by_key(|b| format!("{b:?}"));

        let mut colors: Vec<colorgrad::Color> = Vec::with_capacity(biomes.len());
        let mut domain: Vec<f32> = Vec::with_capacity(biomes.len());

        for (i, biome) in biomes.iter().enumerate() {
            let rgb = biome.color();
            colors.push(colorgrad::Color {
                r: f32::from(rgb[0]) / 255.0,
                g: f32::from(rgb[1]) / 255.0,
                b: f32::from(rgb[2]) / 255.0,
                a: 1.0,
            });
            // Spread biomes evenly across 0-100 range
            domain.push((i as f32 / (biomes.len() - 1).max(1) as f32) * 100.0);
        }

        let grad = colorgrad::GradientBuilder::new()
            .colors(&colors)
            .domain(&domain)
            .build::<colorgrad::LinearGradient>()
            .unwrap_or_else(|_| {
                colorgrad::GradientBuilder::new()
                    .colors(&colors)
                    .build::<colorgrad::LinearGradient>()
                    .expect("Gradient generation failed")
            });

        let vertices_count: usize = ((size[0] + 1) * (size[1] + 1)) as usize;
        let triangle_count: usize = (size[0] * size[1] * 2 * 3) as usize;

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertices_count);
        let mut indices: Vec<u32> = Vec::with_capacity(triangle_count);
        let mut vertex_colors: Vec<[f32; 4]> = Vec::with_capacity(vertices_count);

        let rows = size[0];
        let cols = size[1];
        let width = terrain.size[0] as f32;
        let depth = terrain.size[1] as f32;

        for row in 0..rows {
            for col in 0..cols {
                let cell = &cells[row as usize][col as usize];

                // Use continentalness for height calculation
                let height_value = (cell.continentalness as f32 + 1.0) / 2.0; // Normalize from [-1,1] to [0,1]

                let x = (row as f32 / terrain.resolution as f32 - width / 2.0) + 0.5;
                let y = ((height_value * 1.2).powf(2.0) - 0.5) * 2.0; // Use fixed exponent
                let z = (col as f32 / terrain.resolution as f32 - depth / 2.0) + 0.5;

                // Get color from biome
                let rgb = cell.biome.color();
                let color = [
                    rgb[0] as f32 / 255.0,
                    rgb[1] as f32 / 255.0,
                    rgb[2] as f32 / 255.0,
                    1.0,
                ];

                positions.push([x, y, z]);
                normals.push([0.0, 1.0, 0.0]);
                uvs.push([row as f32, col as f32]);
                vertex_colors.push(color);
            }
        }

        for i in 0..(rows - 1) {
            for j in 0..(cols - 1) {
                let current = i * cols + j;
                let next_row = (i + 1) * cols + j;

                // Triangle 1
                indices.push(current);
                indices.push(current + 1);
                indices.push(next_row);

                // Triangle 2
                indices.push(next_row);
                indices.push(current + 1);
                indices.push(next_row + 1);
            }
        }

        let mut mesh = Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_indices(bevy::render::mesh::Indices::U32(indices.clone()));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        *mesh_handle = Mesh3d(meshes.add(mesh));
    }
}
