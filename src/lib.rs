use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const CREATURE_AMOUNT: usize = 10;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_web() {
    run();
}

pub fn run() {
    let mut app = App::build();
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.5)));
    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(startup_camera.system())
        .add_startup_system(add_creatures.system());
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
    let creature_bundles = (0..CREATURE_AMOUNT).map(move |i: usize| SpriteBundle {
        material: creature_material.clone(),
        sprite: Sprite::new(Vec2::new(50.0, 50.0)),
        transform: Transform::from_xyz(10.0 * i as f32, 20.0 * i as f32, i as f32),
        ..Default::default()
    });
    commands.spawn_batch(creature_bundles);
}
