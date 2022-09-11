use bevy::{prelude::*, window::close_on_esc};

mod collision;
mod components;
mod consts;
mod food;
mod grid_transform;
mod snake;

use collision::CollisionPlugin;
use components::GameState;
use consts::*;
use food::FoodPlugin;
use grid_transform::GridTransformPlugin;
use snake::SnakePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "AI Snake!".to_owned(),
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(GameState::Playing)
        .add_startup_system(setup_camera)
        .add_system(close_on_esc)
        .add_plugin(FoodPlugin)
        .add_plugin(SnakePlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GridTransformPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
