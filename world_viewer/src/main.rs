use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::HashMap;
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use world_gen::WorldGen;
use world_gen::cell::Cell;

const IMAGE_WIDTH: u32 = 1000;
const IMAGE_HEIGHT: u32 = 1000;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_camera, update_camera))
        .run();
}

#[derive(Resource)]
struct ProcGenImageHandle(Handle<Image>);
#[derive(Resource)]
struct WorldGenerator(world_gen::WorldGen, HashMap<(isize, isize), Cell>);

#[derive(Resource)]
struct GlobalPosition(Vec3);

#[derive(Resource)]
struct ZoomLevel(f32);

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    // Create an image that we are going to draw into
    let image = Image::new_fill(
        Extent3d {
            width: IMAGE_WIDTH,
            height: IMAGE_HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // Initialize it with a beige color
        &(css::BEIGE.to_u8_array()),
        // Use the same encoding as the color we set
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    // Add it to Bevy's assets, so it can be used for rendering
    // this will give us a handle we can use
    // (to display it in a sprite, or as part of UI, etc.)
    let handle = images.add(image);

    // Create a sprite entity using our image
    commands.spawn(Sprite::from_image(handle.clone()));

    let position = Vec3::ZERO;
    let world_gen = WorldGen::new(1.0, Some(1));
    let cells = HashMap::new();

    let image = images.get_mut(&handle).unwrap();
    image
        .data
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let x = (i as u32) % IMAGE_WIDTH;
            let y = (i as u32) / IMAGE_WIDTH;
            // Set the pixel to be fully transparent
            #[allow(clippy::cast_possible_wrap)]
            let color = world_gen
                .generate_cell((x as isize, y as isize))
                .biome
                .color();
            px[0] = color[0];
            px[1] = color[1];
            px[2] = color[2];
            px[3] = 255;
        });

    commands.insert_resource(ProcGenImageHandle(handle));
    commands.insert_resource(GlobalPosition(position));
    commands.insert_resource(ZoomLevel(1.0));
    commands.insert_resource(WorldGenerator(world_gen, cells));
}

fn move_camera(
    mut position: ResMut<GlobalPosition>,
    mut zoom_level: ResMut<ZoomLevel>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    const CAMERA_SPEED: f32 = 500.0;
    const CAMERA_HIGH_SPEED: f32 = 1000.0;
    let multiplier = if kb_input.pressed(KeyCode::ShiftLeft) {
        CAMERA_HIGH_SPEED
    } else {
        CAMERA_SPEED
    };
    let move_delta = direction.normalize_or_zero() * multiplier * time.delta_secs();
    position.0 += move_delta.extend(0.);

    for ev in evr_scroll.read() {
        let scroll_amount = ev.y;
        let zoom_sensitivity = 0.1;
        zoom_level.0 *= 1.0 - (scroll_amount * zoom_sensitivity);
        zoom_level.0 = zoom_level.0.clamp(0.1, 10.0);
    }
}

/// Update the camera position by tracking the player.
fn update_camera(
    camera: Single<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    global_position: Res<GlobalPosition>,
    zoom_level: Res<ZoomLevel>,
    time: Res<Time>,
) {
    let (mut transform, mut projection) = camera.into_inner();
    let Vec3 { x, y, .. } = global_position.0;
    let direction = Vec3::new(x, y, 0.0);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    const CAMERA_DECAY_RATE: f32 = 5.0;
    transform
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());

    // Update zoom via orthographic projection scale
    projection.scale = zoom_level.0;
}
