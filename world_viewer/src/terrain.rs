use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_water::material::{StandardWaterMaterial, WaterMaterial};
use world_gen::WorldGen;

use crate::camera_plugin::MainCamera;

const SIZE: [u32; 2] = [8; 2];
const RESOLUTION: u32 = 32;
const RENDER_DISTANCE: f32 = 16.0; // Radius in chunks

#[derive(Component)]
pub struct Chunk {
    position: [i32; 2],
    /// The size of the terrain.
    size: [u32; 2],
    resolution: u32,
    /// Whether the chunk mesh has been generated
    generated: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            position: [0, 0],
            size: SIZE,
            resolution: RESOLUTION,
            generated: false,
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
        app.add_systems(Startup, (initialize_world_generator, spawn_water).chain())
            .add_systems(
                Update,
                (
                    spawn_chunks_around_camera,
                    generate_terrain,
                    update_water_position,
                )
                    .chain(),
            );
    }
}

#[derive(Resource)]
pub struct WorldGenRes(pub WorldGen);
fn initialize_world_generator(mut commands: Commands) {
    let world_gen = WorldGen::new(1.0, Some(1));
    commands.insert_resource(WorldGenRes(world_gen));
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    camera_query: Query<&Transform, With<MainCamera>>,
    existing_chunks: Query<&Chunk>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let camera_pos = camera_transform.translation;
    let camera_chunk_pos = [
        (camera_pos.x / SIZE[0] as f32).floor() as i32,
        (camera_pos.z / SIZE[1] as f32).floor() as i32,
    ];

    // Collect existing chunk positions for fast lookup
    let existing_positions: std::collections::HashSet<[i32; 2]> =
        existing_chunks.iter().map(|chunk| chunk.position).collect();

    // Spawn chunks in a radius around camera
    let spawn_radius = (RENDER_DISTANCE * 1.5) as i32;
    for dx in -spawn_radius..=spawn_radius {
        for dz in -spawn_radius..=spawn_radius {
            let chunk_pos = [camera_chunk_pos[0] + dx, camera_chunk_pos[1] + dz];

            // Skip if chunk already exists
            if existing_positions.contains(&chunk_pos) {
                continue;
            }

            commands.spawn(ChunkBundle {
                chunk: Chunk {
                    position: chunk_pos,
                    size: SIZE,
                    resolution: RESOLUTION,
                    generated: false,
                },
                ..default()
            });
        }
    }
}

fn generate_terrain(
    world_gen: Res<WorldGenRes>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(
        &mut Chunk,
        &mut Mesh3d,
        &MeshMaterial3d<StandardMaterial>,
        &mut Visibility,
    )>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    // Get camera position
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };
    let camera_pos = camera_transform.translation;
    let camera_chunk_pos = [
        (camera_pos.x / SIZE[0] as f32).floor() as i32,
        (camera_pos.z / SIZE[1] as f32).floor() as i32,
    ];

    // Collect chunks with their distance from camera
    let mut chunks_to_process: Vec<_> = query
        .iter_mut()
        .map(|(chunk, mesh, mat, vis)| {
            let dx = (chunk.position[0] - camera_chunk_pos[0]) as f32;
            let dz = (chunk.position[1] - camera_chunk_pos[1]) as f32;
            let distance = (dx * dx + dz * dz).sqrt();
            (chunk, mesh, mat, vis, distance)
        })
        .collect();

    // Sort by distance (closest first)
    chunks_to_process.sort_by(|a, b| a.4.partial_cmp(&b.4).unwrap());

    const MAX_CHUNKS_PER_FRAME: usize = 3;
    let mut generated_this_frame = 0;

    for (mut terrain, mut mesh_handle, material, mut visibility, distance) in chunks_to_process {
        if distance > RENDER_DISTANCE {
            *visibility = Visibility::Hidden;
            continue;
        }

        *visibility = Visibility::Visible;

        if terrain.generated {
            continue;
        }

        // Limit generation per frame to avoid lag
        if generated_this_frame >= MAX_CHUNKS_PER_FRAME {
            break;
        }

        if let Some(material) = materials.get_mut(material) {
            *material = StandardMaterial::default();
        }
        let size = [
            terrain.size[0] * terrain.resolution,
            terrain.size[1] * terrain.resolution,
        ];
        let world_position = [
            terrain.position[0] * SIZE[0] as i32 * terrain.resolution as i32,
            terrain.position[1] * SIZE[1] as i32 * terrain.resolution as i32,
        ];
        let cells = world_gen.0.generate_chunk(world_position, size);

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

        let _grad = colorgrad::GradientBuilder::new()
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
        for row in 0..rows {
            for col in 0..cols {
                let cell_y = col as usize;
                let cell_x = row as usize;
                let cell = &cells[cell_y][cell_x];

                let height_value = cell.height as f32;

                let x = terrain.position[0] as f32 * SIZE[0] as f32
                    + row as f32 / terrain.resolution as f32;
                let y = height_value * 10.0; // Scale up the height variation
                let z = terrain.position[1] as f32 * SIZE[1] as f32
                    + col as f32 / terrain.resolution as f32;

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
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_indices(Indices::U32(indices.clone()));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        *mesh_handle = Mesh3d(meshes.add(mesh));

        // Mark chunk as generated
        terrain.generated = true;
        generated_this_frame += 1;
    }
}

#[derive(Component)]
struct Water;

fn spawn_water(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut water_materials: ResMut<Assets<StandardWaterMaterial>>,
) {
    let water_size = RENDER_DISTANCE * SIZE[0] as f32 * 2.0;
    let water_mesh = meshes.add(Plane3d::default().mesh().size(water_size, water_size));
    let water_material = water_materials.add(StandardWaterMaterial {
        base: default(),
        extension: WaterMaterial::default(),
    });

    commands.spawn((
        Name::new("Water"),
        Water,
        Mesh3d(water_mesh),
        MeshMaterial3d(water_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn update_water_position(
    camera_query: Query<&Transform, With<MainCamera>>,
    mut water_query: Query<&mut Transform, (With<Water>, Without<MainCamera>)>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    if let Ok(mut water_transform) = water_query.single_mut() {
        water_transform.translation.x = camera_transform.translation.x;
        water_transform.translation.y = 0.0;
        water_transform.translation.z = camera_transform.translation.z;
    }
}
