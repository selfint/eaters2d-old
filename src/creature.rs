use bevy::prelude::*;

const CREATURE_SIZE: f32 = 10.;

pub struct Creature;

pub fn add_creatures(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle: ColorMaterial = asset_server.load("red_circle.png").into();
    let creature_material = materials.add(texture_handle);

    let creature = SpriteBundle {
        material: creature_material.clone(),
        sprite: Sprite::new(Vec2::new(CREATURE_SIZE, CREATURE_SIZE)),
        ..Default::default()
    };
    commands.spawn_bundle(creature);
}
