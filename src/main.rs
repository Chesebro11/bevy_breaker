use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::window::PrimaryWindow;
use rand::prelude::*;


#[derive(Component)]
pub struct Paddle {}
#[derive(Component)]
pub struct Ball {
    pub direction: Vec2,
}

// #[derive(Resource)]
// pub struct 

// Paddle Variables
pub const PADDLE_SIZE: f32 = 64.0;
pub const PADDLE_SPEED: f32 = 25.;
pub const PADDLE_Y: f32 = -125.;

// Ball Variables
// pub const BALL_COLOR
pub const BALL_SIZE: f32 = 15.;
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(15.0, 0.0, 0.0);
// pub const INITIAL_BALL_DIRECTION
pub const BALL_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_paddle)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_ball)
        .add_system(move_paddle)
        .add_system(confine_paddle)
        .add_system(start_ball)
        .add_system(update_ball_direction)
        .run();
}

pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle {
        // Decently large positive Z value on the camera is neccesarry PLEASE DONT FORGET OR YOU WILL LOSE YOUR MIND AGAIN!!!!
        transform: Transform::from_xyz(0.0, 0.0, 999.9),
        ..default()
    });
}

// Need to Re-Write this slightly using a window query, weird things are happening as I'm using somethings with the window for spawning, some are just random cords.
pub fn spawn_paddle(mut commands: Commands, ) {
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
pub fn confine_paddle(
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
) {
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
        } else if  translation.x > x_max {
            translation.x = x_max;
        }

        paddle_transform.translation = translation;
    }
}

// Spawn Ball
pub fn spawn_ball(
    mut commands: Commands,
    // ball_query: Query <&mut Transform, With<Ball>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // time: Res<Time>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(25.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(BALL_STARTING_POSITION),
            ..default()
        },
        Ball {
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
        },
    ));
}
// Will need to modify this function later, for now this is testing making the ball start moving.
// Before this function can work I need to implement the time resource, I believe I'll need to create a time.deltatime somewhere.
pub fn start_ball (
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
pub fn update_ball_direction (
mut ball_query: Query<(&Transform, &mut Ball)>,
mut paddle_query: Query<(Entity, &Transform), With<Paddle>>,
)
{
    if let Ok (mut ball_transform) = ball_query.get_single_mut() {
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

            if translation.y > y_max || translation.y < y_min {
                ball.direction.y *= -1.0;
            }
        }

        if let Ok((paddle_entity, paddle_transform)) = paddle_query.get_single_mut() {
            for ball_transform in ball_query.iter() {
                let distance = paddle_transform
                    .translation
                    .distance(ball_transform.translation);
                let paddle_radius = PADDLE_SIZE /2.0;
                let ball_radius = BALL_SIZE / 2.0;
            }
        }
    }
}

// This function will despawn the ball when falling BELOW the paddle's Y coordinate
pub fn despawn_ball () {}
