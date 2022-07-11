use crate::creature::Creature;
use crate::neural_network::NeuralNetwork;
use crate::{random_location, Config};
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Component)]
pub struct Dead;

pub fn aging(
    config: Res<Config>,
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Creature), Without<Dead>>,
) {
    for (entity, mut creature) in query.iter_mut() {
        creature.age += time.delta().as_millis() as f32 / 1000.;

        if creature.age > config.creature_max_age {
            commands.entity(entity).insert(Dead);
        }
    }
}

pub fn hunger(
    #[allow(unused_variables)]
    config: Res<Config>,
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Creature), Without<Dead>>,
) {
    for (entity, mut creature) in query.iter_mut() {
        creature.health -= (0.05 + creature.speed) * time.delta().as_millis() as f32 / 1000.;

        if creature.health <= 0. {
            commands.entity(entity).insert(Dead);
        }
    }
}

pub fn genetic_algorithm(
    config: Res<Config>,
    mut commands: Commands,
    mut dead_creatures: Query<
        (Entity, &mut Creature, &mut Transform, &mut NeuralNetwork),
        With<Dead>,
    >,
    potential_mates: Query<(&Creature, &NeuralNetwork), Without<Dead>>,
) {
    for (entity, mut creature, mut transform, mut neural_network) in dead_creatures.iter_mut() {
        commands.entity(entity).remove::<Dead>();
        transform.translation = random_location().extend(0.);

        // reset creature parameters
        creature.age = 0.;
        creature.health = config.creature_health;
        creature.speed = 0.0;
        creature.food_eaten = 0;

        // sample random potential mate based on their health
        let mut rng = rand::thread_rng();

        let max_food_eaten: usize = potential_mates
            .iter()
            .map(|(creature, _)| creature.food_eaten)
            .max()
            .unwrap_or(1)
            .max(1);

        let potential_mates: Vec<&NeuralNetwork> = potential_mates
            .iter()
            .filter_map(|(creature, network)| {
                let fitness = creature.food_eaten as f32 / max_food_eaten as f32;

                if rng.gen_range(0.0..1.0) < fitness {
                    Some(network)
                } else {
                    None
                }
            })
            .collect();

        // breed with random potential mate
        if let Some(&mate_network) = potential_mates.choose(&mut rng) {
            let my_params = &neural_network.parameters;
            let mate_params = &mate_network.parameters;

            // crossover
            let mut new_params: Vec<f32> = Vec::with_capacity(my_params.len());
            for (my_param, mate_param) in my_params.iter().zip(mate_params.iter()) {
                if rng.gen() {
                    new_params.push(*my_param);
                } else {
                    new_params.push(*mate_param);
                }
            }

            // mutate
            for param in &mut new_params {
                if rng.gen::<f32>() < config.creature_mutation_rate {
                    *param += rng
                        .gen_range(-config.creature_mutation_range..config.creature_mutation_range);
                }
            }

            // replace neural network with new one
            neural_network.parameters = new_params;
        } else {
            // if no potential mate, just mutate
            for param in &mut neural_network.parameters {
                if rng.gen::<f32>() < config.creature_mutation_rate {
                    *param += rng
                        .gen_range(-config.creature_mutation_range..config.creature_mutation_range);
                }
            }

        }
    }
}
