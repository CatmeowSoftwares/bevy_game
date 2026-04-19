use bevy::prelude::*;

#[derive(Component)]
struct Character;

#[derive(Component, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord )]
struct Health(u32);
