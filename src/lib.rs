use bevy::prelude::*;
use creature::*;
use food::*;
use genetic_algorithm::Dead;
use wasm_bindgen::prelude::*;

mod creature;
mod food;
mod genetic_algorithm;
mod neural_network;
mod smell;

const WINDOW_WIDTH: f32 = 300.;
const WINDOW_HEIGHT: f32 = 300.;
const CREATURE_SIZE: f32 = 10.;
const FOOD_SIZE: f32 = 5.;
const FOOD_COUNT: usize = 100;
const CREATURE_COUNT: usize = 20;
const CREATURE_HEALTH: f32 = 4.;
const CREATURE_MAX_AGE: f32 = 10.;
const CREATURE_MUTATION_RATE: f32 = 0.1;
const CREATURE_MUTATION_RANGE: f32 = 0.05;

pub struct Config {
    pub creature_health: f32,
    pub creature_max_age: f32,
    pub creature_mutation_rate: f32,
    pub creature_mutation_range: f32,
}

#[wasm_bindgen]
pub fn run_web() {
    run()
}

pub fn run() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.3, 0.2, 0.0)));
    app.insert_resource(WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    });
    app.insert_resource(Config {
        creature_health: CREATURE_HEALTH,
        creature_max_age: CREATURE_MAX_AGE,
        creature_mutation_rate: CREATURE_MUTATION_RATE,
        creature_mutation_range: CREATURE_MUTATION_RANGE,
    });

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(startup_camera);
    app.add_startup_system(add_creatures);
    app.add_startup_system(spawn_foods);

    app.add_system(world_bounds);
    app.add_system(creature_eat);
    app.add_system(smell::smell_system);
    app.add_system(creature::creature_movement);
    app.add_system(genetic_algorithm::aging);
    app.add_system(genetic_algorithm::hunger);
    app.add_system(genetic_algorithm::genetic_algorithm);

    app.add_system(debugger);

    app.run();
}

fn startup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn add_creatures(asset_server: Res<AssetServer>, mut commands: Commands) {
    let texture = asset_server.load("white_circle.png");

    commands.spawn_batch((0..CREATURE_COUNT).map(move |_| {
        CreatureBundle::new(
            random_location(),
            CREATURE_SIZE,
            CREATURE_HEALTH,
            &[1, 3, 2],
            texture.clone(),
        )
    }));
}

fn spawn_foods(asset_server: Res<AssetServer>, mut commands: Commands) {
    let texture = asset_server.load("white_circle.png");

    commands.spawn_batch(
        (0..FOOD_COUNT)
            .map(move |_| FoodBundle::new(random_location(), FOOD_SIZE, texture.clone())),
    );
}

fn random_location() -> Vec2 {
    Vec2::new(
        rand::random::<f32>() * WINDOW_WIDTH - WINDOW_WIDTH / 2.,
        rand::random::<f32>() * WINDOW_HEIGHT - WINDOW_HEIGHT / 2.,
    )
}

fn world_bounds(mut query: Query<&mut Transform>) {
    for mut transform in query.iter_mut() {
        transform.translation.x = transform
            .translation
            .x
            .clamp(-WINDOW_WIDTH / 2., WINDOW_WIDTH / 2.);
        transform.translation.y = transform
            .translation
            .y
            .clamp(-WINDOW_HEIGHT / 2., WINDOW_HEIGHT / 2.);
    }
}

fn creature_eat(
    mut creatures: Query<(&mut Creature, &Transform), (With<Creature>, Without<Dead>)>,
    mut foods: Query<(&Food, &mut Transform), Without<Creature>>,
) {
    for (mut creature, transform) in creatures.iter_mut() {
        for (food, mut food_transform) in foods.iter_mut() {
            let distance = (food_transform.translation - transform.translation).length();

            if distance < creature.size / 2. + food.size / 2. {
                creature.health += food.size;
                food_transform.translation = random_location().extend(0.);
            }
        }
    }
}

fn debugger(query: Query<&Creature>) {
    let mut max_health: f32 = 0.;
    for creature in query.iter() {
        max_health = max_health.max(creature.health);
    }

    println!("max_health: {:?}", max_health);
}