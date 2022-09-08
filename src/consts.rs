use super::components::*;
use bevy::prelude::*;

// Colors
pub const CLEAR_COLOR: Color = Color::BLACK;
pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const FOOD_COLOR: Color = Color::PURPLE;

// Positions
pub const SNAKE_HEAD_START_POS: Pos = Pos::new(3, 3);

// Dimensions
pub const ARENA_WIDTH: u32 = 30;
pub const ARENA_HEIGHT: u32 = 30;
pub const WINDOW_SIZE: f32 = 800.0;

// Timings
pub const MOVE_DELAY: f64 = 0.15;
