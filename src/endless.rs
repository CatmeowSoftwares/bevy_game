use crate::game::*;
use avian3d::prelude::*;
use bevy::prelude::*;
#[derive(Resource)]
struct Endless {
    wave: u32,
    points: u32,
}

pub struct EndlessPlugin;
impl Plugin for EndlessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMode::Endless), load_endless_map);
    }
}
fn load_endless_map(asset_server: Res<AssetServer>, mut commands: Commands) {
    let map = asset_server.load("models/map.glb#Scene0");
    commands.spawn((
        SceneRoot(map.clone()),
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
    ));
}

fn start_endless() {}

// fym end endless
fn end_endless() {}

fn start_next_wave(mut thing: ResMut<Endless>) {
    thing.wave += 1;
}
