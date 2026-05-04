use crate::character::{Character, *};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand::RngExt;
#[derive(Component, Default)]
#[require(Character)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_random_enemy);
    }
}

#[derive(Component)]
#[require(Enemy)]
struct Enemy1;

#[derive(Component)]
#[require(Enemy)]
struct Enemy2;

#[derive(Component)]
#[require(Enemy)]
struct Enemy3;

#[derive(Component)]
#[require(Enemy)]
struct Enemy4;

#[derive(Component)]
#[require(Enemy)]
struct Enemy5;

#[derive(Event)]
pub struct SpawnRandomEnemyEvent {
    pub rank: u32,
}
fn spawn_random_enemy(
    event: On<SpawnRandomEnemyEvent>,
    mut commands: Commands,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    println!("happens");
    match event.rank {
        1 => {
            if rng.random_range(0..100) < 75 {
                commands.spawn(enemy_1());
            } else {
                commands.spawn(enemy_2());
            }
        }
        2 => {
            if rng.random_range(0..100) < 75 {
                commands.spawn(enemy_2());
            } else {
                // mutated enemy 1
                let entity = commands.spawn(enemy_1()).id();
            }
        }
        3 => {}
        4 => {}
        5 => {}
        6 => {}
        7 => {}
        8 => {}
        9 => {}
        10 => {}
        _ => {}
    }
}

/*
* rank 1
enemy 1 - 75%
enemy 2 - 25%

* rank 2
enemy2 - 75%
enemy1lv2 - 25%

* rank 3
enemy 3 - 75%
enemy1lv3 - 12.5%
enemy2lv2 - 12.5%




*/

fn enemy_1() -> impl Bundle {
    (
        Name::new("Enemy1"),
        Enemy1,
        Health(25),
        CharacterController,
        CharacterMovementSettings::default(),
        CharacterCollisions::default(),
        TransformInterpolation,
        GroundDetection {
            cast_shape: Some(Collider::capsule(0.24, 1.8)),
            ..default()
        },
        Collider::capsule(0.25, 1.8),
    )
}
fn enemy_2() -> impl Bundle {
    (
        Name::new("Enemy2"),
        Enemy2,
        Health(50),
        CharacterController,
        CharacterMovementSettings::default(),
        CharacterCollisions::default(),
        TransformInterpolation,
        GroundDetection {
            cast_shape: Some(Collider::capsule(0.24, 1.8)),
            ..default()
        },
        Collider::capsule(0.25, 1.8),
    )
}
fn enemy_3() -> impl Bundle {
    Enemy3
}
fn enemy_4() -> impl Bundle {
    Enemy4
}
fn enemy_5() -> impl Bundle {
    Enemy5
}
