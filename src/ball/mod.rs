use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_ball)
        .add_system(start_ball)
        .add_system(update_ball_direction)
        .add_system(ball_kill_event);
    }
}