use avian3d::{math::Scalar, prelude::*};
use bevy::{
    color::palettes::css::*, ecs::event::Trigger, input::mouse::MouseButtonInput,
    math::VectorSpace, prelude::*,
};

use crate::{
    boss::Target,
    character::{DamageEvent, Health},
    enemy::Enemy,
    player::Player,
};

#[derive(Component)]
struct Pistol;

#[derive(Component)]
struct Rifle;

#[derive(Component)]
struct Shotgun;

#[derive(Component)]
struct Sniper;

#[derive(Component, Deref, DerefMut)]
struct CurrentWeapon(Entity);

#[derive(Component)]
struct LookPos(Vec3);
enum DamageType {
    Hitscan,
    Projectile,
}
#[derive(Component, Deref, DerefMut)]
struct Muzzle(Vec3);
#[derive(Event)]
struct ShootEvent {
    from: Entity,
    user: Entity,
    origin: Vec3,
    direction: Dir3,
}
pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pistol)
            .add_systems(Update, shoot_system)
            .add_systems(
                Update,
                |mut gizmo: Gizmos,
                 pistol_query: Query<(&GlobalTransform, &Muzzle), With<Pistol>>| {
                    for (trans, muzzle) in pistol_query {
                        gizmo.ray(
                            trans.translation() + **muzzle,
                            trans.rotation() * Vec3::NEG_Z,
                            RED,
                        );
                    }
                },
            )
            .add_systems(Update, rotate_current_weapon_to_target_system)
            .add_systems(FixedUpdate, rotate_current_weapon_to_pos)
            .add_observer(knockback_system)
            .add_observer(health_hit_system)
            .add_observer(handle_pistol_shoot);
    }
}

fn player_shoot_system(
    query: Query<&CurrentWeapon>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    for current_weapon in query {
        if mouse.just_pressed(MouseButton::Left) {}
    }
}
#[derive(Component)]
struct Pickable;
#[derive(Bundle)]
struct PickableBundle {
    pickable: Pickable,
    collider: Collider,
    rigidbody: RigidBody,
    sensor: Sensor,
    mass: Mass,
    collision_events_enabled: CollisionEventsEnabled,
    swep_ccd: SweptCcd,
}
impl Default for PickableBundle {
    fn default() -> Self {
        let collider = Collider::cuboid(0.25, 0.25, 0.25);
        Self {
            pickable: Pickable,
            mass: Mass::from_shape(&collider, 1.0),
            collider,
            rigidbody: RigidBody::Dynamic,
            sensor: Sensor,
            collision_events_enabled: CollisionEventsEnabled,
            swep_ccd: SweptCcd::new_with_mode(SweepMode::NonLinear),
        }
    }
}
impl PickableBundle {
    fn new(collider: Collider) -> Self {
        Self {
            pickable: Pickable,
            mass: Mass::from_shape(&collider, 1.0),
            collider,
            rigidbody: RigidBody::Dynamic,
            sensor: Sensor,
            collision_events_enabled: CollisionEventsEnabled,
            swep_ccd: SweptCcd::new_with_mode(SweepMode::NonLinear),
        }
    }
}
fn rotate_current_weapon_to_target_system(
    query: Query<(&Target, &CurrentWeapon), With<Enemy>>,
    mut transform_query: Query<&mut Transform>,
) {
    for (target, current_weapon) in query {
        let Ok(trans) = transform_query.get(**target) else {
            continue;
        };
        let target_trans = trans.translation;
        let Ok(mut weapon_trans) = transform_query.get_mut(**current_weapon) else {
            continue;
        };
        weapon_trans.look_at(target_trans, Dir3::NEG_Z);
    }
}
fn rotate_current_weapon_to_pos(
    player_query: Query<(Entity, &Children, &CurrentWeapon), With<Player>>,
    parent_query: Query<&ChildOf>,
    camera_query: Query<Entity, With<Camera>>,
    global_trans_query: Query<&GlobalTransform>,
    mut trans_query: Query<&mut Transform>,
    spatial_query: SpatialQuery,
    mut commands: Commands,
) {
    for (player_entity, children, current_weapon) in player_query {
        let spatial_query_filer = SpatialQueryFilter::from_excluded_entities([player_entity]);
        let Some(cam_entity) = children
            .iter()
            .find_map(|child| camera_query.get(child).ok())
        else {
            continue;
        };
        let Ok(trans) = global_trans_query.get(cam_entity) else {
            continue;
        };
        let Ok(global_weapon_trans) = global_trans_query.get(**current_weapon) else {
            continue;
        };
        let Ok(player_trans) = global_trans_query.get(player_entity) else {
            continue;
        };
        let Ok(mut weapon_trans) = trans_query.get_mut(**current_weapon) else {
            continue;
        };
        let origin = trans.translation();
        let direction = trans.rotation() * Dir3::NEG_Z;
        let hit =
            spatial_query.cast_ray(origin, direction, Scalar::MAX, true, &spatial_query_filer);
        let pos = if let Some(hit) = hit {
            origin + (direction * hit.distance)
        } else {
            origin + (direction * 1000.0)
        };
        commands.entity(player_entity).insert(LookPos(pos));

        weapon_trans.look_at(
            player_trans.affine().inverse().transform_point3(pos),
            Dir3::Y,
        );
    }
}
fn shoot_system(
    query: Query<(Entity, &CurrentWeapon, &Children), With<Player>>,
    camera_query: Query<(&GlobalTransform, &Camera3d)>,
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    for (entity, current_weapon, children) in query {
        for child in children {
            let Ok((trans, cam)) = camera_query.get(*child) else {
                continue;
            };
            if mouse.just_pressed(MouseButton::Left) {
                // send a message or event that the current weapon is shot or something
                commands.trigger(ShootEvent {
                    from: **current_weapon,
                    user: entity,
                    origin: trans.translation(),
                    direction: trans.rotation() * Dir3::NEG_Z,
                });
            }
        }
    }
}
#[derive(Event)]
struct HitEvent {
    entity: Entity,
    damage: u32,
    position: Vec3,
    direction: Vec3,
}
fn health_hit_system(
    event: On<HitEvent>,
    mut health_query: Query<&mut Health>,
    mut commands: Commands,
) {
    commands.trigger(DamageEvent {
        damage: event.damage,
    });
}
fn knockback_system(
    event: On<HitEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lin_vel_query: Query<&mut LinearVelocity>,
) {
    let shape = meshes.add(Sphere::new(0.1));
    let material = materials.add(Color::srgb(1.0, 0.0, 1.0));

    commands.spawn((
        Mesh3d(shape),
        MeshMaterial3d(material),
        Transform::from_translation(event.position),
    ));

    let Ok(mut linear_velocity) = lin_vel_query.get_mut(event.entity) else {
        return;
    };

    **linear_velocity += event.direction * 10.0;
}
fn handle_pistol_shoot(
    event: On<ShootEvent>,
    pistol_query: Query<(&GlobalTransform, &Muzzle), With<Pistol>>,
    spatial_query: SpatialQuery,
    mut commands: Commands,
) {
    let Ok((trans, muzzle)) = pistol_query.get(event.from) else {
        return;
    };
    dbg!(
        "{}, {}, {}",
        event.from,
        trans.translation() + **muzzle,
        trans.rotation() * Dir3::NEG_Z
    );
    let spatial_query_filer = SpatialQueryFilter::from_excluded_entities([event.user]);

    let origin = trans.translation() + **muzzle;
    let direction = trans.rotation() * Dir3::NEG_Z;
    let hit = spatial_query.cast_ray(origin, direction, Scalar::MAX, true, &spatial_query_filer);
    if let Some(hit) = hit {
        dbg!(
            "HIT!: {}, {}",
            hit.entity,
            origin + (direction * hit.distance)
        );
        commands.trigger(HitEvent {
            position: origin + (direction * hit.distance),
            entity: hit.entity,
            damage: 10,
            direction: direction.as_vec3(),
        });
    }
}

fn spawn_pistol(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            PickableBundle::new(Collider::cuboid(1.0, 1.0, 1.0)),
            Position::from_xyz(0.0, 50.0, 0.0),
        ))
        .with_child((
            Name::new("Pistol"),
            Pistol,
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/pistol.glb#Scene0")),
            ),
            Muzzle(Vec3::ZERO),
        ))
        .observe(handle_picking_stuff);
}

fn handle_picking_stuff(
    event: On<CollisionStart>,
    query: Query<&Children>,
    mut commands: Commands,
) {
    let Ok(children) = query.get(event.collider1) else {
        return;
    };
    let mut weapon = Entity::PLACEHOLDER;
    for child in children {
        weapon = *child;
        commands.entity(event.collider2).add_child(weapon);
    }
    commands
        .entity(event.collider2)
        .insert(CurrentWeapon(weapon));

    commands.entity(event.collider1).despawn();
}
