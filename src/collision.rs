use bevy::prelude::*;

use crate::consts::{ARENA_HEIGHT, ARENA_WIDTH};

use super::components::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collide_body).add_system(wall_collision);
    }
}

fn collide_body(body: Res<SnakeBody>, mut game_state: ResMut<GameState>, pos_query: Query<&Pos>) {
    if *game_state == GameState::Lost {
        return;
    }
    // The first element of the snake body is the head
    let (head, body) = body.0.split_at(1);
    let head_pos = pos_query.get(head[0]).unwrap();
    for body_pos in pos_query.iter_many(body) {
        if head_pos == body_pos {
            *game_state = GameState::Lost;
            println!("You lose");
        }
    }
}

fn wall_collision(mut game_state: ResMut<GameState>, head_query: Query<&Pos, With<SnakeHead>>) {
    if *game_state == GameState::Lost {
        return;
    }
    let head = head_query.single();
    if head.x < 0 || head.x >= ARENA_WIDTH as i32 || head.y < 0 || head.y >= ARENA_HEIGHT as i32 {
        *game_state = GameState::Lost;
        println!("You lose");
    }
}
