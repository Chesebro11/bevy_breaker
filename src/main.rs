use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::window::PrimaryWindow;
use rand::prelude::*;


#[derive(Component)]
pub struct Paddle {}
#[derive(Component)]
pub struct Ball {
    pub direction: Vec2,
}

// Paddle Variables
pub const PADDLE_SIZE: f32 = 64.0;
pub const PADDLE_SPEED: f32 = 25.;
pub const PADDLE_Y: f32 = -125.;

// Ball Variables
// pub const BALL_COLOR
pub const BALL_SIZE: f32 = 15.;
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(15.0, 0.0, 0.0);
// pub const INITIAL_BALL_DIRECTION
// pub const BALL_SPEED

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_paddle)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_ball)
        .add_system(move_paddle)
        .add_system(confine_paddle)
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

// Need to Re-Write this slightly using a window query, weird things are happening as I'm using somethings with the window for spawning, some are just random cords.
pub fn spawn_paddle(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::CYAN.into(),
                custom_size: Some(Vec2::new(95., 25.)),
                ..default()
            }, // Create a variable for Paddle Y Value
            transform: Transform::from_xyz(0.0, -225.0, 0.0),
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
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut paddle_transform) = paddle_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_paddle_size = PADDLE_SIZE / 2.0;
        let x_min = 0.0 + half_paddle_size;
        let x_max = window.width() - half_paddle_size;
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
            direction: Vec2::new(random::<f32>(), random::<f32>()),
        },
    ));
}

// Ball Movement
// pub fn ball_movement(
//     mut commands: Commands,
//     mut ball_query: Query<&mut Transform, With<Ball>>,
// )
// {

// }

// Confine Ball