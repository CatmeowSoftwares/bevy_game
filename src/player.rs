use avian3d::{math::AdjustPrecision, prelude::*};
use bevy::{
    camera::CameraProjection,
    color::palettes::css::*,
    ecs::entity::{EntityCloner, EntityClonerBuilder},
    gizmos::GizmoPlugin,
    prelude::*,
};
use bevy_sprite3d::Sprite3d;
use std::{f32::consts::FRAC_PI_2, ops::Add, time::Duration};
#[derive(Component)]
pub struct Player;
#[derive(Component)]
struct MainPlayer;
use bevy::input::mouse::AccumulatedMouseMotion;

use crate::{boss::Target, character::*};
pub struct PlayerPlugin;

#[derive(Resource, Deref, DerefMut)]
struct ActiveCamera(Entity);
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
        app.add_systems(Startup, setup)
            .add_systems(Update, move_camera)
            .add_systems(Update, move_debug_camera)
            .add_observer(switch_camera)
            .add_systems(Update, input_camera_switch)
            .insert_resource(CurrentCameraUser::Player)
            .insert_resource(ActiveCamera(Entity::PLACEHOLDER))
            .add_systems(FixedUpdate, debug_camera_input_movement)
            //.add_systems(FixedUpdate, (input_movement_system, slow_down).chain())
            .add_systems(FixedUpdate, melee_system)
            //.add_message::<Movement>()
            //.add_systems(FixedUpdate, movement_system)
            //.add_systems(Update, cast_ray_test)
            .add_systems(
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
                                        *v.linear_velocity_mut() = force * 100.0;
                                    }
                                } else if keys.just_pressed(KeyCode::KeyG) {
                                    let res = forces_query.get_mut(hit.entity);
                                    if let Ok(mut v) = res {
                                        let dir = trans.rotation * Vec3::Z;
                                        let force = dir.clamp_length_max(1.0);
                                        *v.linear_velocity_mut() = force * 100.0;
                                    }
                                }
                            }
                        }
                    }
                },
            );
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
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut active_camera: ResMut<ActiveCamera>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //let player_img = asset_server.load("player.png");
    let camera_stuff = (
        Camera3d::default(),
        Camera {
            order: 1,
            ..Default::default()
        },
        Projection::from(PerspectiveProjection {
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
        CurrentCamera,
        Melee,
    );
    let player_cmd = commands.spawn((
        Player,
        Transform::from_xyz(0.0, 10.0, 0.0),
        Character,
        MainPlayer,
        CharacterController,
        CharacterMovementSettings::default(),
        CharacterCollisions::default(),
        TransformInterpolation,
        GroundDetection {
            cast_shape: Some(Collider::capsule(0.24, 1.8)),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::capsule(0.25, 1.8),
        CollidingEntities::default(),
        //SceneRoot(
        //  asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/player.glb#Scene0")),
        //),
        //
        Name::new("Player"),
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/triple_t.glb#Scene0")),
        ),
        /*Sprite {
            image: player_img,
            ..Default::default()
        },
        Sprite3d {
            pixels_per_metre: 32.0,
            ..Default::default()
        },*/
    ));
    let player = player_cmd.id();

    let mut camera = Entity::PLACEHOLDER;
    commands.entity(player).with_children(|parent| {
        camera = parent
            .spawn((
                camera_stuff,
                RayCaster::default()
                    .with_direction(Dir3::NEG_Z)
                    .with_query_filter(SpatialQueryFilter::from_excluded_entities([player])),
            ))
            .id()
    });

    **active_camera = camera;
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            is_active: false,
            ..Default::default()
        },
        DebugCamera,
        Transform::from_xyz(0.0, 0.0, 20.0),
        RayCaster::default().with_direction(Dir3::NEG_Z),
    ));
    for _ in 0..5 {
        let mesh = meshes.add(Sphere::new(1.0));
        let material = materials.add(Color::srgb(0.5, 0.5, 0.5));

        commands.spawn((
            RigidBody::Dynamic,
            Collider::sphere(1.0),
            Mesh3d(mesh),
            MeshMaterial3d(material),
        ));
    }
}

fn set_target(query: Query<(&ChildOf, &Camera, &RayCaster, &RayHits)>, mut commands: Commands) {
    for (child_of, cam, raycaster, rayhits) in query {
        if let Some(hit) = rayhits.first() {
            commands
                .entity(child_of.parent())
                .insert(Target(hit.entity));
        }
    }
}

#[derive(Message)]
enum Movement {
    Move(Vec3),
    Jump,
}
fn debug_camera_input_movement(
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
fn input_movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&Transform, Forces, Option<&Grounded>), With<MainPlayer>>,
    mut message_writer: MessageWriter<Movement>,
) {
    for (trans, mut forces, grounded) in &mut player {
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

        let space_pressed = keys.just_pressed(KeyCode::Space) && grounded.is_some();
        let rotated_input = (trans.rotation * input).clamp_length_max(1.0) * 20.0;
        if rotated_input.length() > 0.0 {
            message_writer.write(Movement::Move(rotated_input));
        }
        if space_pressed {
            message_writer.write(Movement::Jump);
        }
    }
}
fn slow_down(query: Query<Forces, With<Grounded>>, time: Res<Time<Fixed>>) {
    for mut forces in query {
        let linear_velocity = forces.linear_velocity_mut();
        linear_velocity.x = linear_velocity.x.lerp(0.0, 0.1);
        linear_velocity.z = linear_velocity.z.lerp(0.0, 0.1);
    }
}
fn movement_system(
    mut message_reader: MessageReader<Movement>,
    player_query: Query<Forces, With<MainPlayer>>,
) {
    for mut forces in player_query {
        let linear_velocity = forces.linear_velocity_mut();
        for message in message_reader.read() {
            match message {
                Movement::Move(v3) => {
                    let velocity = Vec3::new(linear_velocity.x, 0.0, linear_velocity.z);
                    let speed = velocity.length();
                    *linear_velocity += Vec3::new(v3.x, 0.0, v3.z);

                    let horizontal = Vec3::new(linear_velocity.x, 0.0, linear_velocity.z);
                    let horizontal_length = horizontal.length();
                    if horizontal_length > speed {
                        let c = horizontal.normalize() * 6.0;
                        linear_velocity.x = c.x;
                        linear_velocity.z = c.z
                    } else if speed > 6.0 && horizontal_length < speed {
                        if horizontal_length > 0.0 {
                            let c = horizontal.normalize() * speed;
                            linear_velocity.x = c.x;
                            linear_velocity.z = c.z
                        } else {
                            let c = velocity.normalize() * speed;
                            linear_velocity.x = c.x;
                            linear_velocity.z = c.z
                        }
                    }
                }
                Movement::Jump => {
                    linear_velocity.y = 5.0;
                }
            }
        }
    }
}

fn switch_camera(
    _events: On<CameraSwitch>,
    mut commands: Commands,
    mut players: Query<(Entity, &Children), (With<Player>, Without<DebugCamera>)>,
    mut camera_query: Query<(Entity, &mut Camera), Without<DebugCamera>>,
    mut debug_camera: Query<(Entity, &mut Camera), (With<DebugCamera>, Without<Player>)>,
    mut current_camera_user: ResMut<CurrentCameraUser>,
    mut active_camera: ResMut<ActiveCamera>,
) {
    for (player_entity, children) in &mut players {
        for (debug_entity, mut debug_cam) in &mut debug_camera {
            for child in children {
                let Ok((cam_entity, mut player_cam)) = camera_query.get_mut(*child) else {
                    continue;
                };
                match *current_camera_user {
                    CurrentCameraUser::Player => {
                        player_cam.is_active = false;
                        debug_cam.is_active = true;
                        commands.entity(cam_entity).remove::<CurrentCamera>();
                        commands.entity(debug_entity).insert(CurrentCamera);
                        *current_camera_user = CurrentCameraUser::Debug;
                        println!("current is now debug");
                        **active_camera = debug_entity;
                    }
                    CurrentCameraUser::Debug => {
                        debug_cam.is_active = false;
                        player_cam.is_active = true;
                        commands.entity(debug_entity).remove::<CurrentCamera>();
                        commands.entity(cam_entity).insert(CurrentCamera);
                        *current_camera_user = CurrentCameraUser::Player;
                        println!("current is now player");
                        **active_camera = cam_entity;
                    }
                }
            }
        }
    }
}
fn move_camera(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut camera_query: Query<(&mut Transform, &ChildOf), (With<CurrentCamera>, Without<Player>)>,
    mut player: Query<&mut Transform, With<Player>>,
    camera2: Res<ActiveCamera>,
) {
    let camera = camera_query.get_mut(camera2.0);
    //let mut transform = camera.into_inner();
    let Ok((mut transform, child_of)) = camera else {
        return;
    };
    let Ok(mut player_transform) = player.get_mut(child_of.parent()) else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse input_movement, we already get the full input_movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = -delta.x * 0.002;
        let delta_pitch = -delta.y * 0.002;

        let (yaw, _, _) = player_transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
        // When the user wants to move the camera back to the horizon, which way should the camera face?
        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
        // so the direction picked will for all intents and purposes be arbitrary.
        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let (_, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, pitch, 0.0);
        player_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, 0.0, 0.0);
    }
}

fn move_debug_camera(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut camera_query: Query<&mut Transform, With<DebugCamera>>,
    camera2: Res<ActiveCamera>,
) {
    let camera = camera_query.get_mut(camera2.0);
    //let mut transform = camera.into_inner();
    let Ok(mut transform) = camera else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse input_movement, we already get the full input_movement that happened since the last frame.
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
#[derive(Component)]
struct Melee;
fn melee_system(
    melee_query: Query<(&GlobalTransform, &ChildOf), With<Melee>>,
    mut rigid_body_query: Query<&RigidBody, Without<Player>>,
    mut forces_query: Query<Forces, Without<Player>>,
    spatial_query: SpatialQuery,
    mut player_query: Query<(Entity, Forces), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut gizmos: Gizmos,
) {
    for (trans, child) in melee_query {
        let Ok((player_entity, mut player)) = player_query.get_mut(child.parent()) else {
            continue;
        };
        if !keys.just_pressed(KeyCode::KeyF) {
            continue;
        }
        let has_hit = spatial_query.cast_ray(
            trans.translation(),
            trans.rotation() * Dir3::NEG_Z,
            5.0,
            true,
            &SpatialQueryFilter::from_excluded_entities([player_entity]),
        );
        let Some(hit) = has_hit else {
            continue;
        };

        let Ok(rigid_body) = rigid_body_query.get_mut(hit.entity) else {
            continue;
        };

        if let RigidBody::Static = rigid_body {
            let dir = trans.rotation() * Vec3::Z;
            player.apply_force(dir * 50.0);
        }

        let Ok(mut forces) = forces_query.get_mut(hit.entity) else {
            continue;
        };

        let dir = trans.rotation() * Vec3::NEG_Z;
        match rigid_body {
            RigidBody::Dynamic => {
                forces.apply_force(dir * 20.0);
            }
            RigidBody::Kinematic => {
                *forces.linear_velocity_mut() = dir * 20.0;
            }
            _ => {}
        }
    }
}
