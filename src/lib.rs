use bevy::prelude::*;
use creature::*;
use neural_network::*;
use wasm_bindgen::prelude::*;

mod creature;
mod food;
mod neural_network;

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

    app.add_startup_system(startup_camera)
        .add_startup_system(add_creatures);

    app.run();
}

fn startup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn add_creatures(asset_server: Res<AssetServer>, mut commands: Commands) {
    let texture = asset_server.load("white_circle.png");

    commands.spawn_batch((0..CREATURE_COUNT).map(move |_| {
        CreatureBundle::new(
            random_location(),
            CREATURE_SIZE,
            CREATURE_HEALTH,
            &[1, 3, 1],
            texture.clone(),
        )
    }));
}

fn random_location() -> Vec2 {
    Vec2::new(
        rand::random::<f32>() * WINDOW_WIDTH - WINDOW_WIDTH / 2.,
        rand::random::<f32>() * WINDOW_HEIGHT - WINDOW_HEIGHT / 2.,
    )
}
