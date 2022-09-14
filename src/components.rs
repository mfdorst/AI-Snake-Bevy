use bevy::prelude::*;

use crate::consts::{ARENA_HEIGHT, ARENA_WIDTH};

#[derive(Component)]
pub struct SnakeHead {
    pub prev_dir: Dir,
    pub next_dir: Dir,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component)]
pub struct Size(pub f32);

#[derive(Component)]
pub struct Food;

pub struct EatEvent;

#[derive(Component, Default, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Deref, DerefMut)]
pub struct SnakeBody(pub Vec<Entity>);

#[derive(Default)]
pub struct LastTailPos(pub Pos);

#[derive(Eq, PartialEq)]
pub enum GameState {
    Playing,
    // Paused,
    Lost,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl SnakeHead {
    pub fn default() -> Self {
        Self {
            prev_dir: Dir::Right,
            next_dir: Dir::Right,
        }
    }
}

impl Dir {
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

    pub const fn in_bounds(self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < ARENA_WIDTH as i32 && self.y < ARENA_HEIGHT as i32
    }

    pub fn in_direction(self, direction: Dir) -> Self {
        match direction {
            Dir::Left => Self::new(self.x - 1, self.y),
            Dir::Right => Self::new(self.x + 1, self.y),
            Dir::Up => Self::new(self.x, self.y + 1),
            Dir::Down => Self::new(self.x, self.y - 1),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Dir, *};

    #[test]
    fn pos_in_direction() {
        let pos = Pos::new(3, 3);
        let right = Pos::new(4, 3);
        let left = Pos::new(2, 3);
        let up = Pos::new(3, 4);
        let down = Pos::new(3, 2);
        assert_eq!(pos.in_direction(Dir::Left), left);
        assert_eq!(pos.in_direction(Dir::Right), right);
        assert_eq!(pos.in_direction(Dir::Up), up);
        assert_eq!(pos.in_direction(Dir::Down), down);
    }

    #[test]
    fn direction_opposite() {
        assert_eq!(Dir::Left.opposite(), Dir::Right);
        assert_eq!(Dir::Right.opposite(), Dir::Left);
        assert_eq!(Dir::Up.opposite(), Dir::Down);
        assert_eq!(Dir::Down.opposite(), Dir::Up);
    }
}
