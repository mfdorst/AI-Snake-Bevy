use super::components::{Size, *};
use bevy::prelude::*;

// Colors
pub const CLEAR_COLOR: Color = Color::BLACK;
pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const SNAKE_TAIL_COLOR: Color = SNAKE_HEAD_COLOR;
pub const FOOD_COLOR: Color = Color::PURPLE;

// Sizes
pub const SNAKE_HEAD_SIZE: Size = Size(0.8);
pub const SNAKE_TAIL_SEGMENT_SIZE: Size = Size(0.65);

// Dimensions
pub const ARENA_WIDTH: u32 = 20;
pub const ARENA_HEIGHT: u32 = 20;
pub const WINDOW_SIZE: f32 = 800.0;

// Other
pub const SNAKE_HEAD_START_POS: Pos = Pos::new(10, 15);
pub const SNAKE_STARTING_LEN: usize = 4;
pub const MOVE_DELAY: f64 = 0.15;
