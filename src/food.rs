use crate::smell::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Food {
    pub size: f32,
}

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
            food: Food { size },
            emits_smell: EmitsSmell { smell: size },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(size * 2.0, size * 2.0)),
                    color: Color::rgb(0.0, 1.0, 0.0),
                    ..default()
                },
                texture,
                transform: Transform {
                    translation: location.extend(0.0),
                    ..default()
                },
                ..default()
            },
        }
    }
}
