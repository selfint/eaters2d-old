use bevy::prelude::*;

const CREATURE_SIZE: f32 = 10.;

#[derive(Component)]
pub struct Creature;

pub fn add_creatures(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {

    let creature = SpriteBundle {
        sprite: Sprite { 
            custom_size: Some(Vec2::new(CREATURE_SIZE, CREATURE_SIZE)), 
            ..default()
        },
        texture: asset_server.load("red_circle.png"),
        ..Default::default()
    };

    commands.spawn_bundle(creature).insert(Creature);
}
