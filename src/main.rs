use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::{quick::WorldInspectorPlugin, InspectorOptions};
use bevy_xpbd_2d::{parry::mass_properties::MassProperties, prelude::*};

#[derive(Component, Debug, Clone)]
struct MainCamera;

#[derive(Component, Debug, Clone, InspectorOptions)]
struct Paddle {
    #[inspector(min = 0.)]
    pub speed: f32,
}

const PADDLE_DEFAULT_SPEED: f32 = 50.;

impl Default for Paddle {
    fn default() -> Self {
        Self {
            speed: PADDLE_DEFAULT_SPEED,
        }
    }
}

#[derive(Component, Debug, Clone, InspectorOptions)]
struct Ball {
    #[inspector(min = 0.)]
    pub speed: f32,
}

const BALL_DEFAULT_SPEED: f32 = 50.;

impl Default for Ball {
    fn default() -> Self {
        Self {
            speed: BALL_DEFAULT_SPEED,
        }
    }
}

const WINDOW_WIDTH: f32 = 1000.;
const WINDOW_HEIGHT: f32 = 720.;

const BASE_SIZE: f32 = 25.;
const PADDLE_WIDTH: f32 = BASE_SIZE;
const PADDLE_HEIGHT: f32 = BASE_SIZE * 5.;
const BALL_SIZE: f32 = BASE_SIZE;

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

    let white_material = materials.add(Color::WHITE.into());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)).into())
                .into(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                -(WINDOW_WIDTH / 2.) + (PADDLE_WIDTH / 2.),
                0.,
                0.,
            )),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::cuboid(PADDLE_WIDTH, PADDLE_HEIGHT),
        Restitution::new(2.),
        Paddle::default(),
        Name::new("LeftPaddle"),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)).into())
                .into(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(
                (WINDOW_WIDTH / 2.) - (PADDLE_WIDTH / 2.),
                0.,
                0.,
            )),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::cuboid(PADDLE_WIDTH, PADDLE_HEIGHT),
        Restitution::new(2.),
        Paddle::default(),
        Name::new("RightPaddle"),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(BALL_SIZE, BALL_SIZE)).into())
                .into(),
            material: white_material.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(BALL_SIZE, BALL_SIZE),
        LinearVelocity::from(Vec2::new(300., 0.)),
        LockedAxes::ROTATION_LOCKED,
        Restitution::new(3.).with_combine_rule(CoefficientCombine::Max),
        Ball::default(),
        Name::new("Ball"),
    ));
}

fn movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &mut Paddle, &Name)>,
) {
    let left_paddle = "LeftPaddle".to_string();
    let right_paddle = "RightPaddle".to_string();

    for (mut velocity, paddle, name) in query.iter_mut() {
        if keyboard.pressed(KeyCode::W) && name.to_string() == left_paddle {
            velocity.0.y += paddle.speed;
        } else if keyboard.pressed(KeyCode::S) && name.to_string() == left_paddle {
            velocity.0.y -= paddle.speed;
        } else if keyboard.pressed(KeyCode::Up) && name.to_string() == right_paddle {
            velocity.0.y += paddle.speed;
        } else if keyboard.pressed(KeyCode::Down) && name.to_string() == right_paddle {
            velocity.0.y -= paddle.speed;
        } else {
            velocity.0.y = 0.;
        }
    }
}

const BALL_MAX_VELOCITY: f32 = 400.;

// set max velocity for the ball
fn ball(mut query_ball: Query<(&mut LinearVelocity, &Ball)>) {
    for (mut velocity, _) in query_ball.iter_mut() {
        if velocity.x > BALL_MAX_VELOCITY {
            velocity.x = BALL_MAX_VELOCITY
        } else if velocity.x < -BALL_MAX_VELOCITY {
            velocity.x = -BALL_MAX_VELOCITY
        }
    }
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
