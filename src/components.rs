use bevy::prelude::*;

#[derive(Component)]
pub struct SnakeHead {
    pub prev_dir: Direction,
    pub next_dir: Direction,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component)]
pub struct Size(pub f32);

#[derive(Component)]
pub struct Food;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Default, Deref, DerefMut)]
pub struct SnakeBody(pub Vec<Entity>);

impl SnakeHead {
    pub fn default() -> Self {
        Self {
            prev_dir: Direction::Right,
            next_dir: Direction::Right,
        }
    }
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
