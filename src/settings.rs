use bevy::prelude::*;

#[derive(Resource)]
struct Volume(u8);

#[derive(Resource)]
enum Graphics {
    Potato,
    Toaster,
    Low,
    Medium,
    High,
    UltraHigh,
}
