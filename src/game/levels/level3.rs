
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::game::GameState;
use crate::game::GolfHole;
use crate::game::Ball;
use crate::game::Velocity;
use crate::game::Collider;
use crate::GOLF_HOLE_COLOR;
use crate::GOLF_HOLE_SIZE;
use crate::BALL_COLOR;
use crate::BALL_SIZE;

const GOLF_HOLE_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const BALL_SPEED: f32 = 500.0;
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -250.0, 2.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const WALL_THICKNESS: f32 = 30.0;
const WALL_THICKNESS_H: f32 = 15.0;
const WALL_THICKNESS_D: f32 = 60.0;
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

pub fn load_level_3(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut app_state_next_state: ResMut<NextState<GameState>>,
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

    //let arena_height = TOP_WALL_Y - BOTTOM_WALL_Y;
    //let arena_width = RIGHT_WALL_X - LEFT_WALL_X;
    //let mut walls: Vec<[[f32; 2]; 2]> 

    let mut walls = vec![[[0.,-300.],[900.,WALL_THICKNESS]],[[0.,300.],[900.,WALL_THICKNESS]],
                                             [[0.,-200.],[700.,WALL_THICKNESS]],[[0.,200.],[700.,WALL_THICKNESS]],
                                             [[0.,-100.],[500.,WALL_THICKNESS]],[[0.,100.],[500.,WALL_THICKNESS]],
                                             [[0.,-35.],[300.,WALL_THICKNESS_H]],[[0.,35.],[300.,WALL_THICKNESS_H]],
                                             [[-450.,0.],[WALL_THICKNESS, 600. + WALL_THICKNESS]],
                                             [[450.,0.],[WALL_THICKNESS,600. + WALL_THICKNESS]]];

    // Walls
    for wall in walls {
        let wall_pos = wall[0];
        let wall_size = wall[1];
        commands.spawn(WallBundle::new(Vec2::new(wall_pos[0], wall_pos[1]), Vec2::new(wall_size[0], wall_size[1])));
    }


    app_state_next_state.set(GameState::DeadBall);
    println!("Entered AppState::DeadBall");
}
