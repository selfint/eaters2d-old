use super::smell::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Food;

#[derive(Bundle)]
pub struct FoodBundle {
    food: Food,
    emits_smell: EmitsSmell,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl FoodBundle {
    pub fn new(location: Vec2, size: f32, texture: Handle<Image>) -> Self {
        FoodBundle {
            food: Food,
            emits_smell: EmitsSmell { smell: size },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(size, size)),
                    color: Color::rgb(0., 255., 0.),
                    ..default()
                },
                texture,
                transform: Transform {
                    translation: Vec3::new(location.x, location.y, 0.),
                    ..default()
                },
                ..default()
            },
        }
    }
}
