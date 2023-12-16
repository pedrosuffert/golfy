use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
    transform::commands,
};

#[derive(Resource)]
pub struct BackgroundColor(Color);

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct VelocityVector;

#[derive(Component)]
pub struct VelocityVectorQuad;

#[derive(Component)]
pub struct VelocityVectorTriangle;

#[derive(Component)]
pub struct GolfHole;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;
