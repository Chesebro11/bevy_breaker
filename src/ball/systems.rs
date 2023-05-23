use bevy::prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use rand::prelude::*;

use super::components::Ball;

// Ball Variables
// pub const BALL_COLOR
pub const BALL_SIZE: f32 = 15.;
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(15.0, 0.0, 0.0);
// pub const INITIAL_BALL_DIRECTION
pub const BALL_SPEED: f32 = 200.0;

// Spawn Ball
pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(15.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(BALL_STARTING_POSITION),
            ..default()
        },
        Ball {
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
        },
    ));
}

// Thought a time resource was necessary here, time.delta_seconds is built in.
pub fn start_ball(
    mut ball_query: Query<(&mut Transform, &Ball)>,
    time: Res<Time>,
    // keyboard_input: Res<Input<KeyCode>>,
) {
    // When press Spacebar ball direction = new Vec3(Decide Direction);
    // transform ball translation += direction * BALL_SPEED * time.delta_seconds
    for (mut transform, ball) in ball_query.iter_mut() {
        let direction = Vec3::new(ball.direction.x, ball.direction.y, 0.0);
        transform.translation += direction * BALL_SPEED * time.delta_seconds();
    }
}

// Instead of doing a "Confine_Ball" function This update direction will work in a similar way, When reaching the edges of the screen change direction,
// When making contact with paddle change direction
// When making contact with a brick change direction
// Thinking that I might need to create seperate instances for each scenario where the ball might
// Change direction..
pub fn update_ball_direction(mut ball_query: Query<(&Transform, &mut Ball)>) {
    if let Ok(mut ball_transform) = ball_query.get_single_mut() {
        // These values and the values in confine_paddle are subject to change.
        let x_min = -500.0;
        let x_max = 500.0;
        let y_max = 325.0; // Not sure of this yet testing needed to verify
        let y_min = -325.0; // I don't think I'll use a Y min here because y_min will determine despawning the ball

        for (transform, mut ball) in ball_query.iter_mut() {
            let translation = transform.translation;

            if translation.x < x_min || translation.x > x_max {
                ball.direction.x *= -1.0;
            }

            if translation.y > y_max {
                ball.direction.y *= -1.0;
            }
        }
    }
}

pub fn ball_kill_event(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &Transform), With<Ball>>,
) {
    if let Ok((ball_entity, ball_transform)) = ball_query.get_single_mut() {
        let y_min = -315.0;

        for (entity, transform) in ball_query.iter_mut() {
            let translation = transform.translation;

            if translation.y < y_min {
                commands.entity(ball_entity).despawn();
            }
        }
    }
}