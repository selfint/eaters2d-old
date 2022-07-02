use bevy::prelude::*;

#[derive(Component)]
pub struct Creature {
    pub age: f32,
    pub health: f32,
}