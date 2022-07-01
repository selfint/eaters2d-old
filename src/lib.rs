use creature::*;
use bevy::prelude::*;
use wasm_bindgen::prelude::*;

mod creature;

const WINDOW_WIDTH: f32 = 500.;
const WINDOW_HEIGHT: f32 = 500.;
const CREATURE_SIZE: f32 = 10.;



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

pub fn add_creatures(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {

    let creature = SpriteBundle {
        sprite: Sprite { 
            custom_size: Some(Vec2::new(CREATURE_SIZE, CREATURE_SIZE)), 
            ..default()
        },
        texture: asset_server.load("red_circle.png"),
        ..Default::default()
    };

    commands.spawn_bundle(creature).insert(Creature);
}

