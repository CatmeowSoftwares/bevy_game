use avian3d::prelude::*;
use bevy::{
    ecs::entity::{EntityCloner, EntityClonerBuilder},
    prelude::*,
};
use bevy_sprite3d::Sprite3d;
use std::{f32::consts::FRAC_PI_2, ops::Add};
#[derive(Component)]
struct Player;
use bevy::input::mouse::AccumulatedMouseMotion;
pub struct PlayerPlugin;

#[derive(Resource)]
enum CurrentCameraUser {
    Player,
    Debug,
}
#[derive(Event)]
struct CameraSwitch;

#[derive(Component)]
struct CurrentCamera;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_camera);
        app.add_observer(switch_camera);
        app.add_systems(Update, input_camera_switch);
        app.insert_resource(CurrentCameraUser::Player);
        app.add_systems(FixedUpdate, debug_camera_movement);
        app.add_systems(FixedUpdate, movement_system);
        app.add_systems(
            FixedUpdate,
            |ray_caster_query: Query<(&RayCaster, &RayHits, &Transform)>,
             keys: Res<ButtonInput<KeyCode>>,
             ground_query: Query<&crate::Ground>,
             mut forces_query: Query<Forces>| {
                for (_ray_caster, ray_hits, trans) in &ray_caster_query {
                    for hit in ray_hits.iter() {
                        if !ground_query.contains(hit.entity) {
                            if keys.just_pressed(KeyCode::KeyF) {
                                let res = forces_query.get_mut(hit.entity);
                                if let Ok(mut v) = res {
                                    let dir = trans.rotation * Vec3::NEG_Z;
                                    let force = dir.clamp_length_max(1.0);
                                    *v.linear_velocity_mut() += force * 20.0;
                                }
                            } else if keys.just_pressed(KeyCode::KeyG) {
                                let res = forces_query.get_mut(hit.entity);
                                if let Ok(mut v) = res {
                                    let dir = trans.rotation * Vec3::NEG_Z;
                                    let force = dir.clamp_length_max(1.0);
                                    *v.linear_velocity_mut() -= force * 20.0;
                                }
                            }
                        }
                    }
                }
            },
        );
        app.add_systems(FixedUpdate, |query: Query<(Entity, &CollidingEntities)>| {
            for (entity, colliding_entities) in query {
                println!("{} is colliding with {:?}", entity, colliding_entities);
            }
        });
    }
}
fn input_camera_switch(keys: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::F1) {
        commands.trigger(CameraSwitch);
    }
}
fn parry_system() {}
#[derive(Component)]
struct DebugCamera;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_img = asset_server.load("player.png");
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..Default::default()
        },
        CurrentCamera,
        Player,
        RigidBody::Dynamic,
        LockedAxes::new()
            .lock_rotation_x()
            .lock_rotation_y()
            .lock_rotation_z(),
        Collider::capsule(0.25, 1.8),
        CollidingEntities::default(),
        //SceneRoot(
        //  asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/player.glb#Scene0")),
        //),
        //
        Sprite {
            image: player_img,
            ..Default::default()
        },
        Sprite3d {
            pixels_per_metre: 32.0,
            ..Default::default()
        },
    ));
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 1,
            is_active: false,
            ..Default::default()
        },
        DebugCamera,
        Transform::from_xyz(0.0, 0.0, 20.0),
        RayCaster::default().with_direction(Dir3::NEG_Z),
    ));
}

fn debug_camera_movement(
    keys: Res<ButtonInput<KeyCode>>,
    debug_cam: Single<&mut Transform, (With<DebugCamera>, With<CurrentCamera>)>,
    time: Res<Time<Fixed>>,
) {
    let mut input = Vec3::default();
    let mut trans = debug_cam.into_inner();
    if keys.pressed(KeyCode::KeyW) {
        input.y += 1.0;
    } else if keys.pressed(KeyCode::KeyS) {
        input.y -= 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    } else if keys.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }

    if keys.pressed(KeyCode::KeyQ) {
        input.z -= 1.0;
    } else if keys.pressed(KeyCode::KeyE) {
        input.z += 1.0;
    }

    let input_3d = Vec3 {
        x: input.x,
        y: input.z,
        z: -input.y,
    };

    let rotated_input = trans.rotation * input_3d;
    trans.translation += rotated_input.clamp_length_max(1.0) * 10.0 * time.delta_secs();
}
fn movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&Transform, Forces), (With<Player>, With<CurrentCamera>)>,
) {
    let mut input = Vec3::default();
    if keys.pressed(KeyCode::KeyW) {
        input.z -= 1.0;
    } else if keys.pressed(KeyCode::KeyS) {
        input.z += 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    } else if keys.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }

    let space_pressed = keys.just_pressed(KeyCode::Space);

    for (trans, mut forces) in &mut player {
        let linear_velocity = forces.linear_velocity_mut();
        let rotated_input = (trans.rotation * input).clamp_length_max(1.0) * 20.0;
        linear_velocity.x = rotated_input.x;
        linear_velocity.z = rotated_input.z;
        if space_pressed {
            linear_velocity.y = 0.0;
            forces.apply_force(Vec3::new(0.0, 100.0, 0.0));
        }
    }
}
fn switch_camera(
    _events: On<CameraSwitch>,
    mut commands: Commands,
    mut players: Query<(Entity, &mut Camera), (With<Player>, Without<DebugCamera>)>,
    mut debug_camera: Query<(Entity, &mut Camera), (With<DebugCamera>, Without<Player>)>,
    mut current_camera_user: ResMut<CurrentCameraUser>,
) {
    for (player_entity, mut player_cam) in &mut players {
        for (debug_entity, mut debug_cam) in &mut debug_camera {
            match *current_camera_user {
                CurrentCameraUser::Player => {
                    player_cam.is_active = false;
                    debug_cam.is_active = true;
                    commands.entity(player_entity).remove::<CurrentCamera>();
                    commands.entity(debug_entity).insert(CurrentCamera);
                    *current_camera_user = CurrentCameraUser::Debug;
                    println!("current is now debug");
                }
                CurrentCameraUser::Debug => {
                    debug_cam.is_active = false;
                    player_cam.is_active = true;
                    commands.entity(debug_entity).remove::<CurrentCamera>();
                    commands.entity(player_entity).insert(CurrentCamera);
                    *current_camera_user = CurrentCameraUser::Player;
                    println!("current is now player");
                }
            }
        }
    }
}
fn move_camera(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    camera: Single<&mut Transform, With<CurrentCamera>>,
) {
    let mut transform = camera.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = -delta.x * 0.002;
        let delta_pitch = -delta.y * 0.002;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
        // When the user wants to move the camera back to the horizon, which way should the camera face?
        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
        // so the direction picked will for all intents and purposes be arbitrary.
        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
