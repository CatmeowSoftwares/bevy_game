use bevy::prelude::*;

use crate::{
    character::CharacterPlugin, endless::EndlessPlugin, menu::MenuPlugin, player::PlayerPlugin,
};

pub struct GamePlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
}

#[derive(Resource)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameMode {
    #[default]
    None,
    Story,
    Endless,
    Sandbox,
}
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.init_state::<GameMode>();
        app.insert_resource(Difficulty::Easy);
        app.add_systems(Startup, setup);
        app.add_plugins(MenuPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(CharacterPlugin);
        app.add_plugins(EndlessPlugin);
    }
}
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
    ));
}

fn init_story_mode() {}
fn init_endless_mode() {}
fn init_sandbox_mode() {}
