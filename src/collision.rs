use bevy::prelude::*;

use super::components::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collide_body);
    }
}

fn collide_body(
    body: Res<SnakeBody>,
    mut game_state: ResMut<GameState>,
    pos_query: Query<&Pos>,
) {
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
