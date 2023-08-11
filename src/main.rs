use bevy::prelude::*;
use chicken::ChickenPlugin;

pub mod chicken;
pub mod spritesheet;

fn setup(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    dbg!(window.resolution.width(), window.resolution.height());
    commands.spawn(Camera2dBundle::default());
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, spritesheet::make_tiles));
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin, ChickenPlugin))
        .run();
}
