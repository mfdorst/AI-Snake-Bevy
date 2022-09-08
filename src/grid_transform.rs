use bevy::prelude::*;

use super::components::{Pos, Size};
use super::consts::*;

pub struct GridTransformPlugin;

impl Plugin for GridTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        );
    }
}

fn size_scaling(windows: Res<Windows>, mut query: Query<(&Size, &mut Transform)>) {
    let window = match windows.get_primary() {
        Some(window) => window,
        None => return,
    };
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
    let window = match windows.get_primary() {
        Some(window) => window,
        None => return,
    };
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
