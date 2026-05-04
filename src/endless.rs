use crate::{enemy::SpawnRandomEnemyEvent, game::*, player::PlayerUi};
use avian3d::prelude::*;
use bevy::{color::palettes::css::*, prelude::*};
use bevy_rand::prelude::*;
use rand::RngExt;
#[derive(Resource, Default)]
struct Endless {
    wave: u32,
    points: u32,
}

pub struct EndlessPlugin;
impl Plugin for EndlessPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(start_next_wave);
        app.init_resource::<Endless>();
        app.add_systems(OnEnter(GameMode::Endless), init_endless);
    }
}

#[derive(Component)]
struct WaveText;
#[derive(Component)]
struct EnemyCountText;
fn init_endless(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    player_query: Query<Entity, With<PlayerUi>>,
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("uc3.ogg")),
        PlaybackSettings::LOOP,
    ));
    let map = asset_server.load("models/map.glb#Scene0");
    commands.spawn((
        SceneRoot(map.clone()),
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
    ));
    for entity in player_query {
        println!("something");
        commands.entity(entity).with_child((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect::top(percent(5)),
                border: UiRect::all(px(5)),
                ..Default::default()
            },
            children![
                (WaveText, Text::new("Wave 0"), Node { ..default() }),
                (EnemyCountText, Text::new("Enemies: 0"),)
            ], /*
               children![(
                   Node {
                       width: px(300),
                       height: px(100),
                       align_items: AlignItems::Center,
                       justify_content: JustifyContent::Center,
                       border: UiRect::all(px(5)),
                       ..Default::default()
                   },
                   children![Text::new("bevy_game"),]
               )],
                */
        ));
        println!("triggering event");
        commands.trigger(SpawnEvent);
    }
}
#[derive(Event)]
struct SpawnEvent;
fn start_next_wave(
    event: On<SpawnEvent>,
    mut commands: Commands,
    mut endless: ResMut<Endless>,
    wave_text: Query<&mut Text, (With<WaveText>, Without<EnemyCountText>)>,
    enemy_count_text: Query<&mut Text, (With<EnemyCountText>, Without<WaveText>)>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    let mut enemy_count = 0;
    if endless.wave % 5 == 0 {
        // boss
    }

    for _ in endless.wave.min(5)..rng.random_range(10..25) {
        commands.trigger(SpawnRandomEnemyEvent {
            rank: rng.random_range(1..2),
        });
        enemy_count += 1;
    }
    endless.wave += 1;
    for mut wave_text in wave_text {
        *wave_text = Text::new(format!("Wave {}", endless.wave));
    }
    for mut enemy_count_text in enemy_count_text {
        *enemy_count_text = Text::new(format!("Enemy Count: {}", enemy_count));
    }
}
// fym end endless
fn end_endless() {}
