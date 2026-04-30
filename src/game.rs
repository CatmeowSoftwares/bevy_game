use bevy::prelude::*;

use crate::menu::MenuPlugin;

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
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_plugins(MenuPlugin);
    }
}
