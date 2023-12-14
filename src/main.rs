//! A simplified implementation of the classic game "Breakout".

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
    transform::commands,
};

pub mod levels;
use levels::*;

mod ui;
use ui::GameUIPlugin;

//Game Resolution
const RESOLUTION: Vec2 = Vec2::new(1920.0, 1080.0);

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const GOLF_HOLE_SIZE: Vec3 = Vec3::new(45.0, 45.0, 0.0);

// const SCOREBOARD_FONT_SIZE: f32 = 40.0;
// const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.533333, 0.329412);
const BALL_COLOR: Color = Color::WHITE;
const GOLF_HOLE_COLOR: Color = Color::BLACK;

// const VELOCITY_VECTOR_QUERY: Color = Color::DARK_GRAY;
// const VELOCITY_VECTOR_QUAD_SIZE: Vec3 = Vec3::new(4.0, 100.0, 0.0);
// const VELOCITY_VECTOR_TRIANGLE_SIZE: Vec3 = Vec3::new(20.0, 20.0, 1.0);

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

// const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
// const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
// const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
// const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    QualquerCoisaRenan,
    MainMenu,
    LoadingMap,
    DeadBall,
    BallMoving,
    UnloadingMap,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_plugins(LevelsPlugins)
        .add_state::<AppState>()
        //.add_plugins(GameUIPlugin)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        // .add_systems(OnEnter(AppState::DeadBall), setup_swing)
        // .add_systems(OnEnter(AppState::DeadBall), spawn_velocity_vector)
        .add_systems(
            FixedUpdate,
            (set_ball_velocity,).run_if(in_state(AppState::DeadBall)),
        )
        // .add_systems(OnExit(AppState::DeadBall), unspawn_velocity_vector)
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                uptade_ball_velocity,
                check_ball_inside_hole,
                // play_collision_sound,
            )
                .chain()
                .run_if(in_state(AppState::BallMoving)), // `chain`ing systems together runs them in order
        )
        .add_systems(OnEnter(AppState::UnloadingMap), unload_map)
        .add_systems(Update, (update_scoreboard, bevy::window::close_on_esc))
        .run();
}

// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct VelocityVector;

#[derive(Component)]
struct VelocityVectorQuad;

#[derive(Component)]
struct VelocityVectorTriangle;

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

// Add the game's entities to our world
fn setup(mut commands: Commands, mut window_query: Query<&mut Window>) {
    //Resolution
    let mut window = window_query.single_mut();
    window.resolution.set(RESOLUTION.x, RESOLUTION.y);

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Swings: ",
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
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

// fn spawn_velocity_vector(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn((
//         MaterialMesh2dBundle {
//             mesh: meshes.add(shape::Quad::default().into()).into(),
//             material: materials.add(ColorMaterial::from(VELOCITY_VECTOR_QUERY)),
//             transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
//                 .with_scale(VELOCITY_VECTOR_QUAD_SIZE)
//                 .with_rotation(Quat::from_array([0.0, 0.0, 0.0, 0.0])),
//             ..default()
//         },
//         VelocityVectorQuad,
//         VelocityVector,
//     ));

//     commands.spawn((
//         MaterialMesh2dBundle {
//             mesh: meshes.add(shape::RegularPolygon::new(0.5, 3).into()).into(),
//             material: materials.add(ColorMaterial::from(VELOCITY_VECTOR_QUERY)),
//             transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
//                 .with_scale(VELOCITY_VECTOR_TRIANGLE_SIZE),
//             ..default()
//         },
//         VelocityVectorTriangle,
//         VelocityVector
//     ));
// }

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

fn set_ball_velocity(
    mouse_input: Res<Input<MouseButton>>,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut window_query: Query<&mut Window>,
    mut vector_quad_query: Query<&mut Transform, (With<VelocityVectorQuad>, Without<Ball>)>,
    mut scoreboard: ResMut<Scoreboard>,
    // mut vector_triangle_query: Query<&mut Transform, With<VelocityVectorTriangle>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let cursor_position = window_query
        .single_mut()
        .cursor_position()
        .unwrap_or(Vec2::new(0.0, 0.0));

    let ball_position = ball_transform.translation;
    let mut ball_velocity_vector = Vec2::new(
        cursor_position.x - (ball_position.x + window_query.single_mut().width() / 2.0),
        -cursor_position.y + (window_query.single_mut().height() / 2.0 - ball_position.y),
    );

    // let mut vector_quad = vector_quad_query.single_mut();
    // let mut vector_triangle = vector_triangle_query.single_mut();

    // vector_quad.translation.x = ball_velocity_vector.x / 2.0 + ball_position.x;

    // vector_quad.translation.y = ball_velocity_vector.y / 2.0 + ball_position.y;

    // vector_quad.look_at(Vec3::new(ball_velocity_vector.x, ball_velocity_vector.y, 0.0), Vec3::Y);

    // vector_quad.scale.y = ball_velocity_vector.length();

    // println!("{} {}", (ball_velocity_vector.y / ball_velocity_vector.x).atan(), vector_quad.rotation.to_axis_angle().1);

    // let angle = (ball_velocity_vector.y / ball_velocity_vector.x).atan()
    //     - vector_quad.rotation.to_axis_angle().1;

    // vector_quad.rotate_z(30.0);

    // println!("{}", angle);

    if ball_velocity_vector.x > 700. {
        ball_velocity_vector.x = 700.;
    }
    if ball_velocity_vector.y > 700. {
        ball_velocity_vector.y = 700.;
    }

    if mouse_input.pressed(MouseButton::Left) {
        scoreboard.score += 1;
        *ball_velocity = Velocity(ball_velocity_vector);
        app_state_next_state.set(AppState::BallMoving);
        println!("Entered AppState::BallMoving");
    }
}

// fn unspawn_velocity_vector(
//     mut commands: Commands,
//     vector_query: Query<Entity, With<VelocityVector>>,
// ) {
//     for entity in vector_query.iter() {
//         commands.entity(entity).despawn();
//     }
// }

fn check_ball_inside_hole(
    ball_query: Query<&Transform, With<Ball>>,
    hole_query: Query<&Transform, With<GolfHole>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    let ball_position = ball_query.single().translation;
    let hole_position = hole_query.single().translation;
    let distance = ball_position.distance(hole_position);

    if distance <= GOLF_HOLE_SIZE.length() / 2.0 {
        app_state_next_state.set(AppState::UnloadingMap);
        println!("Entered AppState::BallInHole");
    }
}

fn unload_map(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    golf_hole_query: Query<Entity, With<GolfHole>>,
    wall_query: Query<Entity, With<Collider>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut level_resource: ResMut<Level>,
) {
    commands.entity(ball_query.single()).despawn();
    commands.entity(golf_hole_query.single()).despawn();
    for entity in wall_query.iter() {
        commands.entity(entity).despawn();
    }

    level_resource.0 += 1;

    app_state_next_state.set(AppState::LoadingMap);
    println!("Entered AppState::LoadingMap");
}

fn uptade_ball_velocity(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, transform) in &collider_query {
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

    // println!("{} {}", ball_velocity.x, ball_velocity.y);

    if ball_velocity.x.abs() <= 5.0 && ball_velocity.y.abs() <= 5.0 {
        app_state_next_state.set(AppState::DeadBall);
        println!("Entered AppState::DeadBall");
    } else {
        let sum = ball_velocity.x.abs() + ball_velocity.y.abs();
        if sum >= 0.0 {
            ball_velocity.x -= 5.0 * ball_velocity.x.signum() * (ball_velocity.x.abs() / sum);
            ball_velocity.y -= 5.0 * ball_velocity.y.signum() * (ball_velocity.y.abs() / sum);
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
