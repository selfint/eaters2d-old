use super::neural_network::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Creature {
    pub age: f32,
    pub health: f32,
}

#[derive(Bundle)]
pub struct CreatureBundle {
    creature: Creature,
    neural_network: NeuralNetwork,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl CreatureBundle {
    pub fn new(
        location: Vec2,
        size: f32,
        health: f32,
        dims: &[usize],
        texture: Handle<Image>,
    ) -> Self {
        CreatureBundle {
            creature: Creature { health, age: 0. },
            neural_network: NeuralNetwork::new(dims, sigmoid_activation),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(size, size)),
                    color: Color::rgb(255., 0., 0.),
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
