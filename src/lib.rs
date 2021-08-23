use bevy::prelude::*;
use rand::Rng;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const CREATURE_AMOUNT: usize = 10;
const CREATURE_SIZE: f32 = 10.0;
const CREATURE_PRIORITY: f32 = 2.;
const FOOD_AMOUNT: usize = 10;
const FOOD_SIZE: f32 = 5.0;
const FOOD_PRIORITY: f32 = 1.;
const WINDOW_WIDTH: f32 = 300.;
const WINDOW_HEIGHT: f32 = 300.;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_web() {
    run();
}

pub fn run() {
    let mut app = App::build();
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.5)));
    app.insert_resource(WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(startup_camera.system())
        .add_startup_system(add_creatures.system())
        .add_startup_system(add_foods.system());
    app.run();
}

fn startup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn add_creatures(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle: ColorMaterial = asset_server.load("red_circle.png").into();
    let creature_material = materials.add(texture_handle);
    let mut rng = rand::thread_rng();

    let min_x = -WINDOW_WIDTH / 2.;
    let max_x = min_x + WINDOW_WIDTH;
    let min_y = -WINDOW_HEIGHT / 2.;
    let max_y = min_y + WINDOW_HEIGHT;

    for _ in 0..CREATURE_AMOUNT {
        let creature = SpriteBundle {
            material: creature_material.clone(),
            sprite: Sprite::new(Vec2::new(CREATURE_SIZE, CREATURE_SIZE)),
            transform: Transform::from_xyz(
                rng.gen_range(min_x..max_x),
                rng.gen_range(min_y..max_y),
                CREATURE_PRIORITY,
            ),
            ..Default::default()
        };
        commands.spawn_bundle(creature);
    }
}

fn add_foods(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle: ColorMaterial = asset_server.load("white_circle.png").into();
    let creature_material = materials.add(texture_handle);
    let mut rng = rand::thread_rng();

    let min_x = -WINDOW_WIDTH / 2.;
    let max_x = min_x + WINDOW_WIDTH;
    let min_y = -WINDOW_HEIGHT / 2.;
    let max_y = min_y + WINDOW_HEIGHT;

    for _ in 0..FOOD_AMOUNT {
        let food = SpriteBundle {
            material: creature_material.clone(),
            sprite: Sprite::new(Vec2::new(FOOD_SIZE, FOOD_SIZE)),
            transform: Transform::from_xyz(
                rng.gen_range(min_x..max_x),
                rng.gen_range(min_y..max_y),
                FOOD_PRIORITY,
            ),
            ..Default::default()
        };
        commands.spawn_bundle(food);
    }
}
