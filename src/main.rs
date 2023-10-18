use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_inspector_egui::{quick::WorldInspectorPlugin, InspectorOptions};
use bevy_xpbd_2d::prelude::*;

mod paddle;
use paddle::*;

mod ball;
use ball::*;

#[derive(Component, Debug, Clone)]
struct MainCamera;

const WINDOW_WIDTH: f32 = 1000.;
const WINDOW_HEIGHT: f32 = 720.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            ..default()
        },
        MainCamera,
        Name::new("MainCamera"),
    ));

    let paddle_mesh: Mesh2dHandle = meshes
        .add(shape::Quad::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGH)).into())
        .into();
    let ball_mesh: Mesh2dHandle = meshes
        .add(shape::Quad::new(Vec2::new(BALL_SIZE, BALL_SIZE)).into())
        .into();
    let white: Handle<ColorMaterial> = materials.add(Color::WHITE.into());

    commands.spawn((
        PaddleBundle::new(10., PaddleSide::LEFT, paddle_mesh.clone(), white.clone()),
        Name::new("LeftPaddle"),
    ));

    commands.spawn((
        PaddleBundle::new(10., PaddleSide::RIGHT, paddle_mesh.clone(), white.clone()),
        Name::new("RightPaddle"),
    ));

    commands.spawn((
        BallBundle::new(Vec3::new(0., 0., 0.), BALL_SIZE, ball_mesh, white),
        Name::new("Ball"),
    ));

    // TODO: top and bottom bars.
}

fn movement(keyboard: Res<Input<KeyCode>>, mut query: Query<(&mut LinearVelocity, &Paddle)>) {
    // let left_paddle = "LeftPaddle".to_string();
    // let right_paddle = "RightPaddle".to_string();

    for (mut velocity, paddle) in query.iter_mut() {
        if keyboard.pressed(KeyCode::W) && paddle.side == PaddleSide::LEFT {
            velocity.0.y += paddle.speed;
        } else if keyboard.pressed(KeyCode::S) && paddle.side == PaddleSide::LEFT {
            velocity.0.y -= paddle.speed;
        } else if keyboard.pressed(KeyCode::Up) && paddle.side == PaddleSide::RIGHT {
            velocity.0.y += paddle.speed;
        } else if keyboard.pressed(KeyCode::Down) && paddle.side == PaddleSide::RIGHT {
            velocity.0.y -= paddle.speed;
        } else {
            velocity.0.y = 0.;
        }
    }
}

fn ball(mut query: Query<(&mut LinearVelocity, With<Ball>)>) {
    for velocity in query.iter() {
        println!("{:?}", velocity);
    }

    /*for (mut velocity, _) in query_ball.iter_mut() {
        if velocity.x > BALL_MAX_VELOCITY {
            velocity.x = BALL_MAX_VELOCITY
        } else if velocity.x < -BALL_MAX_VELOCITY {
            velocity.x = -BALL_MAX_VELOCITY
        }
    }*/
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(Gravity(Vec2::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, ball))
        .run();
}
