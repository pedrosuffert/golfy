//! A simplified implementation of the classic game "Breakout".

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(15.0, 15.0, 0.0);
const BALL_SPEED: f32 = 500.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const GOLF_HOLE_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const GOLF_HOLE_SIZE: Vec3 = Vec3::new(15.0, 15.0, 0.0);

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.533333, 0.329412);
const BALL_COLOR: Color = Color::WHITE;
const GOLF_HOLE_COLOR: Color = Color::BLACK;

// const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    DeadBall,
    BallMoving,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_state::<AppState>()
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        // .add_systems(OnEnter(AppState::DeadBall), setup_swing)
        .add_systems(
            FixedUpdate,
            (set_ball_velocity,).run_if(in_state(AppState::DeadBall)),
        )
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                uptade_ball_velocity,
                // play_collision_sound,
            )
                .chain()
                .run_if(in_state(AppState::BallMoving)), // `chain`ing systems together runs them in order
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct GolfHole;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
struct Brick;

#[derive(Resource)]
struct CollisionSound(Handle<AudioSource>);

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

// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    // let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    // commands.insert_resource(CollisionSound(ball_collision_sound));

    // // Paddle
    // let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    // commands.spawn((
    //     SpriteBundle {
    //         transform: Transform {
    //             translation: Vec3::new(0.0, paddle_y, 0.0),
    //             scale: PADDLE_SIZE,
    //             ..default()
    //         },
    //         sprite: Sprite {
    //             color: PADDLE_COLOR,
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Paddle,
    //     Collider,
    // ));

    // Ball

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

    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    );

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    // // Bricks
    // let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    // let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
    // let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    // assert!(total_width_of_bricks > 0.0);
    // assert!(total_height_of_bricks > 0.0);

    // // Given the space available, compute how many rows and columns of bricks we can fit
    // let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    // let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
    // let n_vertical_gaps = n_columns - 1;

    // // Because we need to round the number of columns,
    // // the space on the top and sides of the bricks only captures a lower bound, not an exact value
    // let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    // let left_edge_of_bricks = center_of_bricks
    //     // Space taken up by the bricks
    //     - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
    //     // Space taken up by the gaps
    //     - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

    // // In Bevy, the `translation` of an entity describes the center point,
    // // not its bottom-left corner
    // let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
    // let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;

    // for row in 0..n_rows {
    //     for column in 0..n_columns {
    //         let brick_position = Vec2::new(
    //             offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
    //             offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
    //         );

    //         // brick
    //         commands.spawn((
    //             SpriteBundle {
    //                 sprite: Sprite {
    //                     color: BRICK_COLOR,
    //                     ..default()
    //                 },
    //                 transform: Transform {
    //                     translation: brick_position.extend(0.0),
    //                     scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
    //                     ..default()
    //                 },
    //                 ..default()
    //             },
    //             Brick,
    //             Collider,
    //         ));
    //     }
    // }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

// fn setup_swing(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     asset_server: Res<AssetServer>,
// ) {
//     commands.spawn((
//         MaterialMesh2dBundle {
//             mesh: meshes.add(shape::Circle::default().into()).into(),
//             material: materials.add(ColorMaterial::from(GOLF_HOLE_COLOR)),
//             transform: Transform::from_translation(GOLF_HOLE_STARTING_POSITION)
//                 .with_scale(GOLF_HOLE_SIZE),
//             ..default()
//         },
//         GolfHole,
//     ));
// }

fn set_ball_velocity(
    mouse_input: Res<Input<MouseButton>>,
    mut ball_query: Query<(&mut Velocity, & Transform), With<Ball>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut window_query: Query<&mut Window>
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let cursor_position = window_query.single_mut().physical_cursor_position().unwrap_or(Vec2::new(0.0, 0.0));

    let ball_position = ball_transform.translation;
    let ball_velocity_vector = Vec2::new(
            cursor_position.x - ball_position.x,
            cursor_position.y - ball_position.y
    );

    // println!("{} {}", ball_velocity_vector.x, ball_velocity_vector.y);

    if mouse_input.pressed(MouseButton::Left) {
        *ball_velocity = Velocity(ball_velocity_vector);
        app_state_next_state.set(AppState::BallMoving);
        println!("Entered AppState::BallMoving");
    }
}

fn uptade_ball_velocity(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, transform, maybe_brick) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }

    if ball_velocity.x.abs() == 0.0 && ball_velocity.y.abs() == 0.0 {
        app_state_next_state.set(AppState::DeadBall);
        println!("Entered AppState::DeadBall");
    } else {
        if ball_velocity.x.abs() >= 0.0 {
            ball_velocity.x -= ball_velocity.x.signum();
        }
        if ball_velocity.y.abs() >= 0.0 {
            ball_velocity.y -= ball_velocity.y.signum();
        }
    }
}

// fn play_collision_sound(
//     mut commands: Commands,
//     mut collision_events: EventReader<CollisionEvent>,
//     sound: Res<CollisionSound>,
// ) {
//     // Play a sound once per frame if a collision occurred.
//     if !collision_events.is_empty() {
//         // This prevents events staying active on the next frame.
//         collision_events.clear();
//         commands.spawn(AudioBundle {
//             source: sound.0.clone(),
//             // auto-despawn the entity when playback finishes
//             settings: PlaybackSettings::DESPAWN,
//         });
//     }
// }
