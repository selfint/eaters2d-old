use crate::neural_network::*;
use crate::smell::CanSmell;
use bevy::prelude::*;

#[derive(Component)]
pub struct Creature {
    pub age: f32,
    pub health: f32,
    pub size: f32,
    direction: Vec2,
    speed: f32,
}

#[derive(Bundle)]
pub struct CreatureBundle {
    creature: Creature,
    neural_network: NeuralNetwork,
    can_smell: CanSmell,

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
            creature: Creature {
                health,
                age: 0.,
                size,
                direction: Vec2::new(0., 0.),
                speed: 0.,
            },
            neural_network: NeuralNetwork::new(dims, sigmoid_activation),
            can_smell: CanSmell {
                smell_radius: size * 3.,
                current_smell: 0.,
                previous_smell: 0.,
            },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(size, size)),
                    color: Color::rgb(255., 0., 0.),
                    ..default()
                },
                texture,
                transform: Transform {
                    translation: location.extend(0.),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn creature_movement(
    mut query: Query<(&CanSmell, &NeuralNetwork, &mut Creature, &mut Transform)>,
) {
    for (can_smell, neural_network, mut creature, mut transform) in query.iter_mut() {
        let network_input = vec![can_smell.get_signal()];
        
        let network_output = neural_network.forward(&network_input);

        // println!("{:?} {:?}", network_input, network_output);

        let turn = network_output[0] * 0.2 - 0.1;
        let speed = network_output[1] * 2. - 1.;

        let current_angle = creature.direction.y.atan2(creature.direction.x);
        let new_angle = current_angle + turn;
        creature.direction = Vec2::new(new_angle.cos(), new_angle.sin());

        creature.speed += speed;
        creature.speed = creature.speed.clamp(0., 1.);

        transform.translation.x += creature.direction.x * creature.speed;         
        transform.translation.y += creature.direction.y * creature.speed;         
    }
}
