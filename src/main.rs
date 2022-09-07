use bevy::{prelude::*, time::FixedTimestep};
use rand::Rng;

// Colors
const CLEAR_COLOR: Color = Color::BLACK;
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::PURPLE;

// Dimensions
const ARENA_WIDTH: u32 = 30;
const ARENA_HEIGHT: u32 = 30;
const WINDOW_SIZE: f32 = 800.0;

// Positions
const SNAKE_HEAD_START_POS: Pos = Pos::new(3, 3);

// Timings
const MOVE_DELAY: f64 = 0.15;

#[derive(Component)]
struct SnakeHead {
    prev_dir: Direction,
    next_dir: Direction,
}

impl SnakeHead {
    fn default() -> Self {
        Self {
            prev_dir: Direction::Right,
            next_dir: Direction::Right,
        }
    }
}

#[derive(Component)]
struct Size(f32);

#[derive(Component)]
struct Food;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

impl Pos {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_owned(),
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_direction_input.before(snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(MOVE_DELAY))
                .with_system(snake_movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(spawn_food),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    commands
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
        .insert(Size(0.8));
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

fn size_scaling(windows: Res<Windows>, mut query: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in &mut query {
        // Scale all transforms with Size components
        transform.scale = Vec3::new(
            sprite_size.0 / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.0 / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut query: Query<(&Pos, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (pos, mut xform) in &mut query {
        xform.translation = Vec3::new(
            convert(pos.x, window.width(), ARENA_WIDTH),
            convert(pos.y, window.height(), ARENA_HEIGHT),
            0.0,
        );
    }
    /// Convert "arena" units to "window" units, move the "arena" origin (0, 0) to the bottom left of
    /// the window, and move the "tile" origin from the center to the bottom left of the tile.
    fn convert(pos: i32, window_bound: f32, arena_bound: u32) -> f32 {
        let unit_size = window_bound / arena_bound as f32;
        let window_center = window_bound / 2.0;
        let unit_center = unit_size / 2.0;
        // Convert from "arena" units to "window" units
        let window_pos = pos as f32 / arena_bound as f32 * window_bound;
        // Move (0, 0) coordinates to bottom left instead of center screen
        let window_pos = window_pos - window_center;
        // Shift the tile over by half, so that its origin is the bottom left, not the center
        window_pos + unit_center
    }
}
