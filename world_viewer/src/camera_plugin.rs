use bevy::{input::mouse::MouseMotion, prelude::*};
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera)
            .add_systems(Update, (camera_movement_system, rotate_camera_to_mouse));
    }
}

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn camera_movement_system(
    input: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let base_speed = 10.0;
    let move_speed = if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) {
        base_speed * 3.0
    } else {
        base_speed
    };

    let mut direction = Vec3::ZERO;

    // Forward/Backward (W/S)
    if input.pressed(KeyCode::KeyW) {
        direction += *camera.forward();
    }
    if input.pressed(KeyCode::KeyS) {
        direction -= *camera.forward();
    }

    // Left/Right (A/D)
    if input.pressed(KeyCode::KeyA) {
        direction -= *camera.right();
    }
    if input.pressed(KeyCode::KeyD) {
        direction += *camera.right();
    }

    // Normalize horizontal movement
    if direction.xz() != Vec2::ZERO {
        let horizontal = Vec3::new(direction.x, 0.0, direction.z).normalize();
        camera.translation += horizontal * move_speed * dt;
    }

    // Up/Down (Space/Ctrl) - separate from horizontal movement
    if input.pressed(KeyCode::Space) {
        camera.translation.y += move_speed * dt;
    }
    if input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight) {
        camera.translation.y -= move_speed * dt;
    }
}
fn rotate_camera_to_mouse(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut transform: Single<&mut Transform, With<MainCamera>>,
) {
    const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;

    // Don't rotate camera if Left Alt is pressed (cursor lock)
    if input.pressed(KeyCode::AltLeft) {
        // Clear mouse motion events to prevent accumulation
        mouse_motion.read().count();
        return;
    }

    let dt = time.delta_secs();
    // The factors are just arbitrary mouse sensitivity values.
    // It's often nicer to have a faster horizontal sensitivity than vertical.
    let mouse_sensitivity = Vec2::new(0.12, 0.10);

    for motion in mouse_motion.read() {
        let delta_yaw = -motion.delta.x * dt * mouse_sensitivity.x;
        let delta_pitch = -motion.delta.y * dt * mouse_sensitivity.y;

        // Add yaw which is turning left/right (global)
        transform.rotate_y(delta_yaw);

        // Add pitch which is looking up/down (local)
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        // Apply the rotation
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
