use bevy::prelude::*;

use super::components::Paddle;

// Paddle Variables
pub const PADDLE_SIZE: f32 = 50.0;
pub const PADDLE_SPEED: f32 = 25.;
pub const PADDLE_Y: f32 = -125.;

pub fn spawn_paddle(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::CYAN.into(),
                custom_size: Some(Vec2::new(95., 25.)),
                ..default()
            }, // Create a variable for Paddle Y Value
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..default()
        },
        Paddle {},
    ));
}

pub fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    // Grab paddle Component
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    // Time Resource
    time: Res<Time>,
) {
    if let Ok(mut transform) = paddle_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        transform.translation += direction * PADDLE_SPEED + time.delta_seconds();
    }
}

// Confine Paddle
pub fn confine_paddle(mut paddle_query: Query<&mut Transform, With<Paddle>>) {
    if let Ok(mut paddle_transform) = paddle_query.get_single_mut() {
        let x_min = -500.0;
        let x_max = 500.0;
        // Y values not needed as paddle should only move on the X Axis
        // let y_min = 0.0 + half_paddle_size;
        // let y_max  = window.height() - half_paddle_size;

        let mut translation = paddle_transform.translation;

        // Limit Paddle on the X Axis
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        paddle_transform.translation = translation;
    }
}