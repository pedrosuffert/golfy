
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

const GOLF_HOLE_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const BALL_SPEED: f32 = 500.0;
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -100.0, 1.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);


// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
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

    // // Scoreboard
    // commands.spawn(
    //     TextBundle::from_sections([
    //         TextSection::new(
    //             "Score: ",
    //             TextStyle {
    //                 font_size: SCOREBOARD_FONT_SIZE,
    //                 color: TEXT_COLOR,
    //                 ..default()
    //             },
    //         ),
    //         TextSection::from_style(TextStyle {
    //             font_size: SCOREBOARD_FONT_SIZE,
    //             color: SCORE_COLOR,
    //             ..default()
    //         }),
    //     ])
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         top: SCOREBOARD_TEXT_PADDING,
    //         left: SCOREBOARD_TEXT_PADDING,
    //         ..default()
    //     }),
    // );

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    // commands.spawn(WallBundle::new(WallLocation::Top));

    app_state_next_state.set(AppState::DeadBall);
    println!("Entered AppState::DeadBall");
}
