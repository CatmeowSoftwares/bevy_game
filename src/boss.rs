use avian3d::prelude::*;
use bevy::{
    color::palettes::css::*, ecs::entity_disabling::Disabled, math::ops::atan2, prelude::*,
};
use bevy_sprite3d::prelude::*;
use std::time::Duration;

use crate::{character::*, enemy::Enemy};
pub struct BossPlugin;
#[derive(Component)]
struct DamageHealth(Timer);
impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_boss_ui)
            .add_observer(on_boss_entered)
            .add_systems(Startup, boss_test_init)
            .add_systems(Update, boss_test_update)
            .add_systems(FixedUpdate, (triple_t_ai,find_target, update_health).chain())
            .add_systems(Update, update_progress_bar)
            .add_systems(
                FixedUpdate,
                |time: Res<Time<Fixed>>,
                mut commands: Commands,
                 triple_t_query: Query<(&mut Health, &mut DamageHealth), With<TripleT>>| {
                    for (mut health, mut timer) in triple_t_query {
                        timer.0.tick(time.delta());
                        if timer.0.just_finished() && **health > 0{
                            **health -= 1;
                        }
                    }
                },
            );
    }
}
#[derive(Component)]
struct Boss;

#[derive(Component)]
struct ProgressBar {
    value: f32,
    max_value: f32,
    interpolate: bool,
}
#[derive(Component, Deref, DerefMut, Default)]
struct Target(Option<Entity>);

#[derive(Component)]
struct BossTheme(AudioSource);
fn sync_progress_bar(query: Query<&ProgressBar>) {}

fn update_health(query: Query<(Entity, &mut Health)>, mut commands: Commands) {
    for (entity, health) in query {
        if **health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

fn on_boss_entered(
    event: On<Add, Boss>,
    query: Query<()>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let c = (
        AudioPlayer::new(asset_server.load("uc3.ogg")),
        PlaybackSettings::LOOP,
    );
    println!("BOSS HAS ENTERED!!!");
    //let image = asset_server.load("multiplier.png");
    commands.entity(event.entity).insert(c);
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: percent(99),
                        height: percent(5),
                        margin: UiRect::all(percent(1)),
                        ..default()
                    },
                    BackgroundColor(PURPLE.into()),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            width: percent(100),
                            height: percent(70),
                            margin: UiRect::all(percent(0.5)),
                            ..default()
                        },
                        BackgroundColor(RED.into()),
                        ProgressBar {
                            value: 100.0,
                            max_value: 100.0,
                            interpolate: true,
                        },
                    ));
                    parent.spawn((
                        Node {
                            margin: UiRect::axes(percent(40), percent(1)),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        Text::new("Triple T: 100/100"),
                        TextColor(BLACK.into()),
                    ));
                });
            //parent.spawn((Node { ..default() }, ImageNode { image, ..default() }));
        });
}
fn update_progress_bar(query: Query<(&mut Node, &ProgressBar)>) {
    for (mut node, progress) in query {
        node.width = percent(progress.value / progress.max_value * 100.0);
    }
}
fn draw_boss_ui(
    boss_query: Query<(&Health, Option<&MaxHealth>), With<Boss>>,
    mut progress_bar_query: Query<&mut ProgressBar>,
) {
    for (health, max_health) in boss_query {
        for mut progress_bar in &mut progress_bar_query {
            let max_health = max_health.map_or(100.0, |a| **a as f32);
            if progress_bar.interpolate {
                progress_bar.value = progress_bar
                    .value
                    .lerp((**health as f32 / max_health) * 100.0, 0.25);
            } else {
                progress_bar.value = (**health as f32 / max_health) * 100.0;
            }
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct BossTest(Timer);
fn boss_test_init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let timer = Timer::new(Duration::from_secs(1), TimerMode::Once);
    //let enemy_img = asset_server.load("enemy.png");
    commands.spawn((
        /*
        Sprite {
            image: enemy_img,
            ..Default::default()
        },
        Sprite3d {
            pixels_per_metre: 32.0,
            ..Default::default()
        },
        */
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/triple_t.glb#Scene0")),
        ),
        Enemy,
        Name::new("Triple T"),
        DamageHealth(Timer::new(Duration::from_secs(1), TimerMode::Repeating)),
        Health(100),
        //Transform::from_xyz(0.0, i as f32, 0.0),
        //Transform::default(),
        CharacterCollisions::default(),
        TransformInterpolation,
        GroundDetection {
            cast_shape: Some(Collider::capsule(0.24, 1.8)),
            ..default()
        },
        TripleT,
        Target::default(),
        RigidBody::Kinematic,
        Collider::capsule(0.25, 1.5),
    ));
    commands.spawn(BossTest(timer));
    println!("boss init done");
}
fn boss_test_update(
    boss_test: Single<&mut BossTest>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let mut timer = boss_test.into_inner();
    timer.tick(time.delta());
    if timer.just_finished() {
        let enemy = enemy_query.iter().next().unwrap();
        commands.entity(enemy).insert(Boss);
    }
}

#[derive(Component)]
struct TripleT;

fn find_target(
    mut predator_query: Query<(&mut Target, &Transform)>,
    prey_query: Query<(Entity, &Transform), With<Character>>,
) {
    for (mut target, trans) in &mut predator_query {
        if target.is_some() {
            continue;
        }
        let mut target_result: Option<Entity> = None;
        let mut prev = 0.0;
        for (prey_entity, prey_trans) in prey_query {
            if trans.translation.distance(prey_trans.translation) > prev {
                target_result = Some(prey_entity);
            }
            prev = trans.translation.distance(prey_trans.translation);
        }
        **target = target_result;
    }
}
use avian3d::math::*;
fn triple_t_ai(
    query: Query<
        (Forces, &Target, &mut Transform, &CharacterMovementSettings),
        (With<TripleT>, Without<Character>),
    >,
    trans_query: Query<&Transform, With<Character>>,
    time: Res<Time>,
) {
    for (mut forces, target, mut trans, movement) in query {
        let Some(target) = **target else {
            continue;
        };

        let Ok(target_trans) = trans_query.get(target) else {
            continue;
        };
        let linear_velocity = forces.linear_velocity_mut();

        if trans.translation.distance(target_trans.translation) < 2.0 {
            continue;
        }
        let delta_secs = time.delta_secs_f64().adjust_precision();

        let v = (target_trans.translation - trans.translation).normalize();
        linear_velocity.x += v.x * movement.acceleration * delta_secs;
        linear_velocity.z -= v.z * movement.acceleration * delta_secs;
        trans.rotation = Quat::from_rotation_y(atan2(v.x, v.z));
    }
}
