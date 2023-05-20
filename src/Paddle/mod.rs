use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

// Implement System sets here
pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_paddle)
            .add_system(move_paddle)
            .add_system(confine_paddle);
    }
}

