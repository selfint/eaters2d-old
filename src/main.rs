use bevy::prelude::*;

const CREATURE_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.45, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_camera.system())
        .add_startup_system(add_creatures.system())
        .run();
}

fn startup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn add_creatures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut texture_handle: ColorMaterial = asset_server.load("white_circle.png").into();
    texture_handle.color = CREATURE_COLOR;
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle),
        sprite: Sprite::new(Vec2::new(50.0, 50.0)),
        ..Default::default()
    });
}
