use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_inspector_egui::InspectorOptions;
use bevy_xpbd_2d::prelude::{
    CoefficientCombine, Collider, LinearVelocity, LockedAxes, Restitution, RigidBody,
};

use crate::paddle::BASE_SIZE;

pub const BALL_SIZE: f32 = BASE_SIZE;

pub const BALL_DEFAULT_SPEED: f32 = 50.;
pub const BALL_MAX_VELOCITY: f32 = 400.;

#[derive(Component, Debug, Clone, InspectorOptions)]
pub struct Ball {
    #[inspector(min = 0.)]
    pub speed: f32,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            speed: BALL_DEFAULT_SPEED,
        }
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub material: MaterialMesh2dBundle<ColorMaterial>,
    pub body: RigidBody,
    pub collider: Collider,
    pub velocity: LinearVelocity,
    pub restitution: Restitution,
    axes: LockedAxes,
}

impl BallBundle {
    pub fn new(
        position: Vec3,
        size: f32,
        mesh: Mesh2dHandle,
        material: Handle<ColorMaterial>,
    ) -> BallBundle {
        return BallBundle {
            ball: Ball::default(),
            material: MaterialMesh2dBundle {
                mesh,
                material,
                transform: Transform::from_translation(position),
                ..default() /*mesh: meshes
                            .add(shape::Quad::new(Vec2::new(BALL_SIZE, BALL_SIZE)).into())
                            .into(),
                            material: Color::WHITE,
                            ..default()*/
            },
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(size, size),
            velocity: LinearVelocity::from(Vec2::new(1000., 0.)),
            restitution: Restitution::new(3.).with_combine_rule(CoefficientCombine::Max),
            axes: LockedAxes::ROTATION_LOCKED,
        };
    }
}
