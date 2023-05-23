use bevy::{prelude::*, sprite::MaterialMesh2dBundle};


pub mod paddle;
pub mod ball;

use paddle::PaddlePlugin;
use ball::BallPlugin;

// #[derive(Resource)]
// pub struct

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PaddlePlugin)
        .add_plugin(BallPlugin)
        .add_startup_system(spawn_camera)
        // .add_system(ball_hit_paddle)
 
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        // Decently large positive Z value on the camera is neccesarry PLEASE DONT FORGET OR YOU WILL LOSE YOUR MIND AGAIN!!!!
        transform: Transform::from_xyz(0.0, 0.0, 999.9),
        ..default()
    });
}

// Need to Re-Write this slightly using a window query, weird things are happening as I'm using somethings with the window for spawning, some are just random cords.




// I AM SO CLOSE TO GETTING this
// pub fn ball_hit_paddle(
//     mut ball_query: Query<(&Transform, &mut Ball)>,
//     mut paddle_query: Query<(Entity, &Transform), With<Paddle>>,
// ) {
//     if let Ok((paddle_entity, paddle_transform)) = paddle_query.get_single_mut() {
//         for (ball_transform, mut ball) in ball_query.iter_mut() {
//             let distance = paddle_transform
//                 .translation
//                 .distance(ball_transform.translation);
//             let paddle_radius = PADDLE_SIZE;
//             let ball_radius = BALL_SIZE / 2.0;
//             if distance < paddle_radius + ball_radius {
//                 ball.direction.x *= -1.0;
//                 ball.direction.y *= -1.0;
//             }
//         }
//     }
// }

// This function will despawn the ball when falling BELOW the paddle's Y coordinate
// pub fn despawn_ball(mut commands: Commands, paddle_query: Query<Entity, With<Paddle>>) {
//     if let Ok(paddle_entity) = paddle_query.get_single() {
//         commands.entity(paddle_entity).despawn();
//     }
// }
