use crate::neural_network::*;
use crate::smell::CanSmell;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Creature {
    pub age: f32,
    pub health: f32,
    pub size: f32,
    speed: f32,
    pub food_eaten: usize,
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
            creature: Creature {
                health,
                age: 0.,
                size,
                speed: 0.,
                food_eaten: 0,
            },
            neural_network: NeuralNetwork::new(dims, sigmoid_activation),
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
    mut query: Query<(&mut Transform, &NeuralNetwork, &mut Creature, &Children)>,
    children_q: Query<(&CanSmell, &Transform), Without<Creature>>,
) {
    for (mut transform, neural_network, mut creature, children) in query.iter_mut() {
        let mut inputs: Vec<(&CanSmell, &Transform)> = children
            .iter()
            .map(|&child| children_q.get(child).unwrap())
            .collect();

        // sort inputs by distance to creature
        // TODO: Are the child components receive in the same order they are inserted? If so, this isn't necessary
        inputs.sort_by(|(_, &at), (_, &bt)| at.translation.partial_cmp(&bt.translation).unwrap());

        // pipe inputs into neural network
        let network_inputs: Vec<f32> = inputs.iter().map(|v| v.0.smell).collect();
        let network_outputs = neural_network.forward(&network_inputs);
        
        println!("{:?} -> {:?}", network_inputs, network_outputs);

        // unpack network outputs
        let new_speed = network_outputs[0];
        let angle_change = network_outputs[1] * 0.05 - 0.025;

        // calculate step
        creature.speed = new_speed;

        // turn
        transform.rotate(Quat::from_rotation_z(angle_change * std::f32::consts::PI));
        let new_angle = transform.rotation.to_scaled_axis().z;

        // advance
        let step: Vec3 = Vec3::new(new_angle.cos(), new_angle.sin(), 0.0) * creature.speed;
        transform.translation += step;
    }
}
