use bevy::{color::palettes::css::*, prelude::*};

use crate::game::{Difficulty, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuState>();
        app.add_systems(OnEnter(MainMenuState::MainMenu), main_menu_init)
            .add_systems(
                Update,
                main_menu_system.run_if(in_state(MainMenuState::MainMenu)),
            )
            .add_systems(
                OnEnter(MainMenuState::DifficultySelector),
                difficulty_selector_init,
            )
            .add_systems(
                Update,
                difficulty_selector_system.run_if(in_state(MainMenuState::DifficultySelector)),
            );
    }
}
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MainMenuState {
    #[default]
    MainMenu,
    DifficultySelector,
    GameModeSelector,
    Settings,
}
#[derive(Component)]
enum MainMenuButtonAction {
    Play,
    Settings,
    Exit,
}
#[derive(Component)]
enum DifficultyButtonAction {
    Easy,
    Medium,
    Hard,
}
#[derive(Component)]
enum GameModeButtonAction {
    StoryMode,
    Endless,
    Sandbox,
}
#[derive(Component)]
struct PlayButton;
#[derive(Component)]
struct SettingsButton;
fn main_menu_init(mut commands: Commands) {
    println!("Main menu");
    commands.spawn((
        DespawnOnExit(MainMenuState::MainMenu),
        Name::new("Main Menu"),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            BackgroundColor(MAGENTA.into()),
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![
                (
                    Button,
                    PlayButton,
                    MainMenuButtonAction::Play,
                    Node {
                        width: px(200),
                        height: px(50),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(px(5)),
                        ..Default::default()
                    },
                    BackgroundColor(BLACK.into()),
                    BorderColor::all(WHITE),
                    children![Text::new("Play")],
                ),
                (
                    Node {
                        width: px(200),
                        height: px(50),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(px(5)),
                        ..Default::default()
                    },
                    MainMenuButtonAction::Settings,
                    BackgroundColor(BLACK.into()),
                    BorderColor::all(WHITE),
                    Button,
                    SettingsButton,
                    children![Text::new("Settings"),],
                ),
                (
                    Node {
                        width: px(200),
                        height: px(50),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(px(5)),
                        ..Default::default()
                    },
                    MainMenuButtonAction::Exit,
                    BackgroundColor(BLACK.into()),
                    BorderColor::all(WHITE),
                    Button,
                    SettingsButton,
                    children![Text::new("Exit"),],
                )
            ]
        )],
    ));
}

fn main_menu_system(
    interaction_query: Query<
        (&Interaction, &MainMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<NextState<MainMenuState>>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            println!("pressed !!!!!");
            match menu_button_action {
                MainMenuButtonAction::Play => {
                    menu_state.set(MainMenuState::DifficultySelector);
                }
                MainMenuButtonAction::Settings => {
                    menu_state.set(MainMenuState::Settings);
                }
                MainMenuButtonAction::Exit => {
                    app_exit_writer.write(AppExit::Success);
                }
            }
        }
    }
}

fn difficulty_selector_init(mut commands: Commands) {
    dbg!("difficulty_selector_init!!!!!!!!!!!!");
    commands.spawn((
        DespawnOnExit(MainMenuState::DifficultySelector),
        Name::new("DifficultySelector"),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            BackgroundColor(MAGENTA.into()),
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![
                (
                    Button,
                    Node {
                        width: px(200),
                        height: px(50),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(px(5)),
                        ..Default::default()
                    },
                    BackgroundColor(BLACK.into()),
                    BorderColor::all(WHITE),
                    children![Text::new("Easy")],
                ),
                (
                    Node {
                        width: px(200),
                        height: px(50),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(px(5)),
                        ..Default::default()
                    },
                    BackgroundColor(BLACK.into()),
                    BorderColor::all(WHITE),
                    Button,
                    children![Text::new("Medium"),],
                ),
                (
                    Node {
                        width: px(200),
                        height: px(50),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(px(5)),
                        ..Default::default()
                    },
                    BackgroundColor(BLACK.into()),
                    BorderColor::all(WHITE),
                    Button,
                    children![Text::new("Hard"),],
                )
            ]
        )],
    ));
}

fn difficulty_selector_system(
    interaction_query: Query<
        (&Interaction, &DifficultyButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut difficulty_state: ResMut<Difficulty>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, difficulty_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match difficulty_button_action {
                DifficultyButtonAction::Easy => {
                    *difficulty_state = Difficulty::Easy;
                }
                DifficultyButtonAction::Medium => {
                    *difficulty_state = Difficulty::Medium;
                }
                DifficultyButtonAction::Hard => {
                    *difficulty_state = Difficulty::Hard;
                }
            }
            menu_state.set(MainMenuState::GameModeSelector);
        }
    }
}
