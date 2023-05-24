use ball::systems::BALL_SIZE;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};

pub mod paddle;
pub mod ball;

use paddle::PaddlePlugin;
use ball::BallPlugin;

use ball::components::Ball;

#[derive(Component)]
pub struct Brick{}

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

// #[derive(Resource)]
// pub struct

pub const BRICK_WIDTH: f32 = 32.0;
pub const BRICK_HEIGHT: f32 = 12.0;
pub const BRICK_SPACING: f32 = 6.0;

// pub const BIRCK_COUNT: u32 = 32;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PaddlePlugin)
        .add_plugin(BallPlugin)
        .add_startup_system(spawn_bricks)
        .add_startup_system(spawn_camera)
        .add_system(ball_hit_brick)
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        // Decently large positive Z value on the camera is neccesarry PLEASE DONT FORGET OR YOU WILL LOSE YOUR MIND AGAIN!!!!
        transform: Transform::from_xyz(0.0, 0.0, 999.9),
        ..default()
    });
}


// I'm going to rewrtie this function to stop using the window_query and use the coordinate system like I have been using. If this doesnt fix it I will tweak the spawning command?
fn spawn_bricks(
    mut commands: Commands,
    // brick_texture: Handle<ColorMaterial>,
) {

    pub const ARENA_WIDTH: f32 = 1000.0;
    // pub const ARENA_HEIGHT: f32 = 650.0;
    
    // Calculate the number of bricks that can fit in the window
    let brick_count = ((ARENA_WIDTH - BRICK_SPACING) / (BRICK_WIDTH + BRICK_SPACING)) as usize;

    // Calculate the starting position for the first brick
    let start_x = -500.0 + BRICK_WIDTH;
    let start_y = 325.0 - BRICK_HEIGHT;

    // Spawn the bricks
    for i in 0..brick_count {
        let x = start_x + i as f32 * (BRICK_WIDTH + BRICK_SPACING);
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::RED.into(),
                custom_size: Some(Vec2::new(BRICK_WIDTH, BRICK_HEIGHT)),
                ..default()
            },
            // material: brick_texture.clone(),
            transform: Transform {
                translation: Vec3::new(x, start_y, 0.0),
                // scale: Vec3::new(BRICK_WIDTH, BRICK_HEIGHT, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
        Brick{};
    }
}

// I can feel that I'm so close to getting this but for some reason this is different than ball and paddle collison
// Even though this system compiles without any issue within a running game there are no collisons between the ball
// and the bricks.
pub fn ball_hit_brick (
mut commands: Commands,
mut ball_query: Query<(&Transform, &mut Ball)>,
mut brick_query: Query<(Entity, &Transform), With<Brick>>,
 ) {
    if let Ok((brick_entity, brick_transform)) = brick_query.get_single_mut() {
        for (ball_transform, mut ball) in ball_query.iter_mut() {
            // let ball_distance = ball_transform
            //     .translation
            //     .distance(ball_transform.translation);
            let brick_distance = brick_transform
                .translation
                .distance(brick_transform.translation);
            let ball_radius = BALL_SIZE / 2.0;
            let brick_girth = (BRICK_HEIGHT + BRICK_WIDTH) / 2.0;
            if brick_distance < ball_radius + brick_girth {
                commands.entity(brick_entity).despawn();
                ball.direction.x *= -1.0;
                ball.direction.y *= -1.0;
            }
        }
    } 
 }

//  pub fn ball_hit_brick
//  (
//     mut commands: Commands,
//     mut ball_query: Query<(&Transform, With<Ball>)>,
//     mut brick_query: Query<(Entity, &Transform), With<Brick>>,
//  )
//  {
//     if let Ok((ball_transform, ball_entity, )) = ball_query.get_single_mut() {
//         for(mut brick, brick_transform) in brick_query.get_single_mut() {
//             let distance = ball_transform
//             .translation
//             .distance(ball_transform.translation);

//         let ball_radius = BALL_SIZE / 2.0;
//         let brick_girth = (BRICK_HEIGHT + BRICK_WIDTH) / 2.0;
//         if distance < ball_radius + brick_girth {
//             commands.entity(brick_entity).despawn();
//             ball.direction.x *= -1.0;
//             ball.direction.y *= -1.0;
//         }
//         }
//     }
//  }


// Commenting this out because as of right now it is unnecessary, might be used later when changing game states
// pub fn despawn_ball(mut commands: Commands, paddle_query: Query<Entity, With<Paddle>>) {
//     if let Ok(paddle_entity) = paddle_query.get_single() {
//         commands.entity(paddle_entity).despawn();
//     }
// }
