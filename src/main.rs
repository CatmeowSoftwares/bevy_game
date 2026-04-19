use avian3d::prelude::*;
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    diagnostic::{self, DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    gltf,
    prelude::*,
    text::FontSmoothing,
};
use bevy_game::player::PlayerPlugin;
use bevy_sprite3d::Sprite3dPlugin;
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
//

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

// One of the two settings that can be set through the menu. It will be a resource in the app

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

struct OverlayColor;

impl OverlayColor {
    const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..Default::default()
        }))
        // Insert as resource the initial value for the settings resources
        .insert_resource(Volume(7))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(PlayerPlugin)
        .add_plugins(Sprite3dPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    // Here we define size of our overlay
                    font_size: 12.0,
                    // If we want, we can use a custom font
                    font: default(),
                    // We could also disable font smoothing,
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                // We can also change color of the overlay
                text_color: OverlayColor::GREEN,
                // We can also set the refresh interval for the FPS counter
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: false,
                    // The minimum acceptable fps
                    min_fps: 30.0,
                    // The target fps
                    target_fps: 60.0,
                },
            },
        })
        .add_systems(
            Update,
            |diagnostics: Res<DiagnosticsStore>, mut overlay: ResMut<FpsOverlayConfig>| {
                if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
                    && let Some(value) = fps.smoothed()
                {
                    if value < 60.0 {
                        overlay.text_color = OverlayColor::RED;
                    } else {
                        overlay.text_color = OverlayColor::GREEN;
                    }
                }
            },
        )
        // Adds the plugins for each state
        //.add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}
fn rotate_stuff(mut query: Query<&mut Transform>, time: Res<Time>) {
    for mut things in &mut query {
        things.rotation += Quat::from_euler(
            EulerRot::XYZ,
            5.0f32.to_degrees() * time.delta_secs(),
            5.0f32.to_degrees() * time.delta_secs(),
            5.0f32.to_degrees() * time.delta_secs(),
        );
        let (x, y, z) = things.rotation.to_euler(EulerRot::XYZ);
        things.rotation = Quat::from_euler(
            EulerRot::XYZ,
            x.rem_euclid(360f32.to_degrees()),
            y.rem_euclid(360f32.to_degrees()),
            z.rem_euclid(360f32.to_degrees()),
        )
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let image = asset_server.load("multiplier.png");
    commands
        .spawn(Node {
            width: percent(30),
            height: percent(30),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(image),
                Node {
                    ..Default::default()
                },
            ));
        });
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 10.0, 0.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -180.0f32.to_degrees(),
            0.0, //160.0f32.to_degrees(),
            0.0,
        )),
    ));
    // note that we have to include the `Scene0` label
    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    let (x, y, z) = (100.0, 0.5, 100.0);
    let shape = meshes.add(Cuboid::new(x, y, z));
    let material = materials.add(Color::srgb(0.5, 0.5, 0.5));
    commands.spawn((
        Mesh3d(shape),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, -10.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(x, y, z),
        bevy_game::Ground,
    ));
    let (x, y, z) = (0.5, 50.0, 100.0);
    let shape = meshes.add(Cuboid::new(x, y, z));
    commands.spawn((
        Mesh3d(shape.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(5.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(x, y, z),
        bevy_game::Ground,
    ));
    commands.spawn((
        Mesh3d(shape.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(-5.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(x, y, z),
        bevy_game::Ground,
    ));
    let (x, y, z) = (100.0, 50.0, 0.5);
    let shape = meshes.add(Cuboid::new(x, y, z));
    commands.spawn((
        Mesh3d(shape.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 5.0),
        RigidBody::Static,
        Collider::cuboid(x, y, z),
        bevy_game::Ground,
    ));
    commands.spawn((
        Mesh3d(shape.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.0, -5.0),
        RigidBody::Static,
        Collider::cuboid(x, y, z),
        bevy_game::Ground,
    ));
}
