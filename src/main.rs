use bevy::prelude::*;
// use rand::*;

#[derive(Component)]
pub struct Paddle {}
#[derive(Component)]
pub struct Ball {}

// Paddle Variables
pub const PADDLE_SPEED: f32 = 25.;
pub const PADDLE_Y: f32 = -125.;

// Ball Variables
// pub const BALL_COLOR
// pub const BALL_STARTING_POSITION
// pub const INITIAL_BALL_DIRECTION
// pub const BALL_SPEED

fn main () {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_paddle)
        .add_startup_system(spawn_camera)
        .add_system(move_paddle)
        .run();
}

pub fn spawn_paddle (
    mut commands: Commands,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::CYAN.into(),
                custom_size: Some(Vec2::new(95., 25.)),
                ..default()
            },                           // Create a variable for Paddle Y Value
            transform: Transform::from_xyz(0.0, -225.0, 0.0),
            ..default()
        },
        Paddle {},
    ));
}

pub fn move_paddle (
    keyboard_input: Res<Input<KeyCode>>,
    // Grab paddle Component
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    // Time Resource **ADD**
    time: Res<Time>,
)
{
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

pub fn spawn_camera(
    mut commands: Commands,
)
{
    commands.spawn(Camera2dBundle{
        ..default()
    });
}

// Confine Paddle

// Spawn Ball
pub fn spawn_ball (mut commands: Commands, 
mut ball_query: Query <&mut Transform, With<Ball>>,
time: Res<Time>,
)
{
    commands.spawn(
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED.into(),
                    
                }
            }
        )
    )
}

// Ball Movement

// Confine Ball