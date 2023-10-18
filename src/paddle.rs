use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_2d::prelude::{Collider, LinearVelocity, RigidBody};

pub const PADDLE_DEFAULT_SPEED: f32 = 50.;

pub const BASE_SIZE: f32 = 25.;
pub const PADDLE_WIDTH: f32 = BASE_SIZE;
pub const PADDLE_HEIGH: f32 = BASE_SIZE * 5.;

use crate::WINDOW_WIDTH;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PaddleSide {
    LEFT,
    RIGHT,
}

#[derive(Component, Debug, Clone, InspectorOptions)]
pub struct Paddle {
    #[inspector(min = 0.)]
    pub speed: f32,
    pub side: PaddleSide,
}

impl Paddle {
    pub fn new(speed: f32, side: PaddleSide) -> Self {
        return Paddle { speed, side };
    }
}

impl Default for Paddle {
    fn default() -> Self {
        Self {
            speed: PADDLE_DEFAULT_SPEED,
            side: PaddleSide::LEFT,
        }
    }
}

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub material: MaterialMesh2dBundle<ColorMaterial>,
    pub body: RigidBody,
    pub collider: Collider,
    pub velocity: LinearVelocity,
}

impl PaddleBundle {
    pub fn new(
        speed: f32,
        side: PaddleSide,
        mesh: Mesh2dHandle,
        material: Handle<ColorMaterial>,
    ) -> Self {
        return PaddleBundle {
            paddle: Paddle::new(speed, side.clone()),
            material: MaterialMesh2dBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec3::new(
                    match side {
                        PaddleSide::LEFT => -(WINDOW_WIDTH / 2.) + (PADDLE_WIDTH / 2.),
                        PaddleSide::RIGHT => (WINDOW_WIDTH / 2.) - (PADDLE_WIDTH / 2.),
                    },
                    0.,
                    0.,
                )),
                ..default()
            },
            body: RigidBody::Kinematic,
            collider: Collider::cuboid(PADDLE_WIDTH, PADDLE_HEIGH),
            velocity: LinearVelocity::from(Vec2::new(0., 0.)),
        };
    }
}
