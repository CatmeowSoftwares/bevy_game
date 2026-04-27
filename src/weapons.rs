use avian3d::{math::Scalar, prelude::*};
use bevy::{
    color::palettes::css::*, ecs::event::Trigger, input::mouse::MouseButtonInput,
    math::VectorSpace, prelude::*,
};

use crate::player::Player;

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
fn shoot_system(
    query: Query<(Entity, &CurrentWeapon)>,
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    for (entity, current_weapon) in query {
        if mouse.just_pressed(MouseButton::Left) {
            // send a message or event that the current weapon is shot or something
            commands.trigger(ShootEvent {
                from: **current_weapon,
                user: entity,
            });
        }
    }
}
fn handle_pistol_shoot(
    event: On<ShootEvent>,
    pistol_query: Query<(&GlobalTransform, &Muzzle), With<Pistol>>,
    spatial_query: SpatialQuery,
) {
    let Ok((trans, muzzle)) = pistol_query.get(event.from) else {
        return;
    };
    println!("{}, {}", event.from, trans.translation() + **muzzle);
    let spatial_query_filer = SpatialQueryFilter::from_excluded_entities([event.user]);

    let hit = spatial_query.cast_ray(
        trans.translation() + **muzzle,
        trans.rotation() * Dir3::NEG_Z,
        Scalar::MAX,
        true,
        &spatial_query_filer,
    );
    if let Some(hit) = hit {
        println!("HIT!: {}", hit.entity);
    }
}

fn spawn_pistol(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(0.1));
    let material = materials.add(Color::srgb(0.5, 0.5, 0.5));

    commands
        .spawn((
            PickableBundle::new(Collider::cuboid(1.0, 1.0, 1.0)),
            Position::from_xyz(0.0, 100.0, 0.0),
        ))
        .with_child((
            Name::new("Pistol"),
            Pistol,
            Mesh3d(mesh),
            MeshMaterial3d(material),
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
