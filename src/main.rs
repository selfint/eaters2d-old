use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.45, 0.0)))
        .add_plugins(DefaultPlugins)
        .run();
}
