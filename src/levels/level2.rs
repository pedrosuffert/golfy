
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::AppState;
use crate::GolfHole;
use crate::Ball;
use crate::Velocity;
use crate::Collider;
use crate::GOLF_HOLE_COLOR;
use crate::GOLF_HOLE_SIZE;
use crate::BALL_COLOR;
use crate::BALL_SIZE;

const GOLF_HOLE_STARTING_POSITION: Vec3 = Vec3::new(-350.0, 250.0, 0.0);
const BALL_SPEED: f32 = 500.0;
const BALL_STARTING_POSITION: Vec3 = Vec3::new(-350.0, -250.0, 2.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const WALL_THICKNESS: f32 = 30.0;
const WALL_THICKNESS_D: f32 = 60.0;

const LEFT_WALL_X: f32 = -400.;
const LEFT_WALL_Y: f32 = 200.;
const RIGHT_MIDDLE_WALL_X: f32 = 250.;
const RIGHT_MIDDLE_WALL_Y: f32 = 250.;
const RIGHT_WALL_X: f32 = 400.;
const RIGHT_WALL_Y: f32 = 400.;
const BOTTOM_WALL_X: f32 = 0.;
const BOTTOM_WALL_Y: f32 = -300.;
const BOTTOM_TOP_WALL_X: f32 = -75.;
const BOTTOM_TOP_WALL_Y: f32 = -100.;
const TOP_WALL_X: f32 = 0.;
const TOP_WALL_Y: f32 = 300.;
const TOP_BOTTOM_WALL_X: f32 = -75.;
const TOP_BOTTOM_WALL_Y: f32 = 100.;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);


// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}


impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(translation: Vec2, scale: Vec2) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: translation.extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: scale.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

pub fn load_level_2(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    // Golf Hole
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(GOLF_HOLE_COLOR)),
            transform: Transform::from_translation(GOLF_HOLE_STARTING_POSITION)
                .with_scale(GOLF_HOLE_SIZE),
            ..default()
        },
        GolfHole,
    ));

    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));

    let arena_height = TOP_WALL_Y - BOTTOM_WALL_Y;
    let arena_width = RIGHT_WALL_X - LEFT_WALL_X;

    // Walls
    commands.spawn(WallBundle::new(Vec2::new(LEFT_WALL_X, -LEFT_WALL_Y), Vec2::new(WALL_THICKNESS, 200.)));
    commands.spawn(WallBundle::new(Vec2::new(LEFT_WALL_X, LEFT_WALL_Y), Vec2::new(WALL_THICKNESS, 200.) ));
    commands.spawn(WallBundle::new(Vec2::new(RIGHT_WALL_X, 0.), Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)));
    commands.spawn(WallBundle::new(Vec2::new(RIGHT_MIDDLE_WALL_X, 0.), Vec2::new(WALL_THICKNESS, 200.)));
    commands.spawn(WallBundle::new(Vec2::new(0., BOTTOM_WALL_Y), Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)));
    commands.spawn(WallBundle::new(Vec2::new(BOTTOM_TOP_WALL_X, BOTTOM_TOP_WALL_Y), Vec2::new(arena_width + WALL_THICKNESS_D - 180., WALL_THICKNESS)));
    commands.spawn(WallBundle::new(Vec2::new(0., TOP_WALL_Y), Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)));
    commands.spawn(WallBundle::new(Vec2::new(TOP_BOTTOM_WALL_X, TOP_BOTTOM_WALL_Y), Vec2::new(arena_width + WALL_THICKNESS_D - 180., WALL_THICKNESS)));

    app_state_next_state.set(AppState::DeadBall);
    println!("Entered AppState::DeadBall");
}
