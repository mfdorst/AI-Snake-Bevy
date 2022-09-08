use bevy::{prelude::*, time::FixedTimestep};

use super::components::{Direction, *};
use super::consts::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeBody::default())
            .add_startup_system(spawn_snake)
            .add_system(snake_direction_input.before(snake_movement))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(MOVE_DELAY))
                    .with_system(snake_movement),
            );
    }
}

fn spawn_segment(commands: &mut Commands, pos: Pos) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_TAIL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(pos)
        .insert(SNAKE_TAIL_SEGMENT_SIZE)
        .id()
}

fn spawn_snake(mut commands: Commands, mut snake_body: ResMut<SnakeBody>) {
    let head = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead::default())
        .insert(SNAKE_HEAD_START_POS)
        .insert(SNAKE_HEAD_SIZE)
        .id();

    let mut body = Vec::with_capacity(SNAKE_STARTING_LEN);
    body.push(head);

    for i in 1..SNAKE_STARTING_LEN {
        body.push(spawn_segment(
            &mut commands,
            Pos::new(SNAKE_HEAD_START_POS.x - i as i32, SNAKE_HEAD_START_POS.y),
        ))
    }
    *snake_body = SnakeBody(body);
}

fn snake_direction_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_query: Query<&mut SnakeHead>,
) {
    if let Some(mut head) = head_query.iter_mut().next() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else {
            head.next_dir
        };
        if dir != head.prev_dir.opposite() {
            head.next_dir = dir;
        }
    }
}

fn snake_movement(mut head_query: Query<(&mut Pos, &mut SnakeHead)>) {
    for (mut pos, mut head) in &mut head_query {
        // Finalize the movement direction
        head.prev_dir = head.next_dir;

        match head.next_dir {
            Direction::Left => {
                pos.x -= 1;
            }
            Direction::Right => {
                pos.x += 1;
            }
            Direction::Down => {
                pos.y -= 1;
            }
            Direction::Up => {
                pos.y += 1;
            }
        }
    }
}
