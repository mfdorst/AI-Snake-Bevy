use bevy::{prelude::*, time::FixedTimestep};

use super::components::*;
use super::consts::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeBody::default())
            .insert_resource(LastTailPos::default())
            .add_event::<EatEvent>()
            .add_startup_system(snake_spawn)
            .add_system(snake_direction_input.before(snake_move))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(MOVE_DELAY))
                    .with_system(snake_move)
                    .with_system(snake_eat.after(snake_move))
                    .with_system(snake_grow.after(snake_eat)),
            );
    }
}

fn snake_spawn(mut commands: Commands, mut snake_body: ResMut<SnakeBody>) {
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
            Dir::Left
        } else if keyboard_input.pressed(KeyCode::Right) {
            Dir::Right
        } else if keyboard_input.pressed(KeyCode::Up) {
            Dir::Up
        } else if keyboard_input.pressed(KeyCode::Down) {
            Dir::Down
        } else {
            head.next_dir
        };
        if dir != head.prev_dir.opposite() {
            head.next_dir = dir;
        }
    }
}

fn snake_move(
    body: Res<SnakeBody>,
    game_state: Res<GameState>,
    mut last_tail_pos: ResMut<LastTailPos>,
    mut head_query: Query<(Entity, &mut SnakeHead)>,
    mut pos_query: Query<&mut Pos>,
) {
    if *game_state == GameState::Lost {
        return;
    }
    let (head_entity, mut head) = head_query.single_mut();
    // Finalize the movement direction
    head.prev_dir = head.next_dir;

    for (next_segment, prev_segment) in body.iter().rev().zip(body.iter().rev().skip(1)) {
        let prev_pos = *pos_query.get(*prev_segment).unwrap();
        let mut next_pos = pos_query.get_mut(*next_segment).unwrap();
        *next_pos = prev_pos;
    }
    let mut head_pos = pos_query.get_mut(head_entity).unwrap();
    match head.next_dir {
        Dir::Left => {
            head_pos.x -= 1;
        }
        Dir::Right => {
            head_pos.x += 1;
        }
        Dir::Down => {
            head_pos.y -= 1;
        }
        Dir::Up => {
            head_pos.y += 1;
        }
    }
    *last_tail_pos = LastTailPos(*pos_query.get(*body.last().unwrap()).unwrap());
}

fn snake_eat(
    mut commands: Commands,
    mut eat_event_writer: EventWriter<EatEvent>,
    food_pos_query: Query<(Entity, &Pos), With<Food>>,
    head_pos_query: Query<&Pos, With<SnakeHead>>,
) {
    let head_pos = head_pos_query.single();
    for (food, food_pos) in &food_pos_query {
        if food_pos == head_pos {
            commands.entity(food).despawn();
            eat_event_writer.send(EatEvent);
        }
    }
}

fn snake_grow(
    mut commands: Commands,
    last_tail_pos: Res<LastTailPos>,
    mut body: ResMut<SnakeBody>,
    mut eat_event_reader: EventReader<EatEvent>,
) {
    if eat_event_reader.iter().next().is_some() {
        body.push(spawn_segment(&mut commands, last_tail_pos.0));
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
