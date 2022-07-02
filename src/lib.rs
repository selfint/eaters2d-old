use bevy::prelude::*;
use creature::*;
use food::*;
use wasm_bindgen::prelude::*;

mod creature;
mod food;
mod neural_network;
mod smell;

const WINDOW_WIDTH: f32 = 500.;
const WINDOW_HEIGHT: f32 = 500.;
const CREATURE_SIZE: f32 = 10.;
const FOOD_SIZE: f32 = 5.;
const FOOD_COUNT: usize = 100;
const CREATURE_COUNT: usize = 10;
const CREATURE_HEALTH: f32 = 100.;

#[wasm_bindgen]
pub fn run_web() {
    run()
}

pub fn run() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.5)));
    app.insert_resource(WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    });

    app.add_plugins(DefaultPlugins);

    app.add_startup_system(startup_camera);
    app.add_startup_system(add_creatures);
    app.add_startup_system(spawn_foods);

    app.add_system(world_bounds);
    app.add_system(smell::smell_system);
    app.add_system(creature::creature_movement);

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
        transform.translation.x = transform.translation.x.clamp(-WINDOW_WIDTH / 2., WINDOW_WIDTH / 2.);
        transform.translation.y = transform.translation.y.clamp(-WINDOW_HEIGHT / 2., WINDOW_HEIGHT / 2.);
    }
}