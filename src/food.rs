use bevy::prelude::*;
use rand::Rng;

use super::components::*;
use super::consts::*;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(food_spawn).add_system(food_respawn);
    }
}

fn food_respawn(commands: Commands, mut eat_event_reader: EventReader<EatEvent>) {
    if eat_event_reader.iter().next().is_some() {
        food_spawn(commands);
    }
}

fn food_spawn(mut commands: Commands) {
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
