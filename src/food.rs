use bevy::{prelude::*, time::FixedTimestep};
use rand::Rng;

use super::components::{Food, Pos, Size};
use super::consts::*;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(spawn_food),
        );
    }
}

fn spawn_food(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Pos::new(
            rng.gen_range(0..ARENA_WIDTH) as i32,
            rng.gen_range(0..ARENA_HEIGHT) as i32,
        ))
        .insert(Size(0.8));
}
