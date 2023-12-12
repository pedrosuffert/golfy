//! A simplified implementation of the classic game "Breakout".

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

pub mod maps;
use maps::*;

//Game Resolution
const RESOLUTION: Vec2 = Vec2::new(1920.0, 1080.0);

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_SIZE: Vec3 = Vec3::new(15.0, 15.0, 2.0);

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const GOLF_HOLE_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);

// const SCOREBOARD_FONT_SIZE: f32 = 40.0;
// const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.533333, 0.329412);
const BALL_COLOR: Color = Color::WHITE;
const GOLF_HOLE_COLOR: Color = Color::BLACK;

// const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
// const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
// const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
// const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    LoadingMap,
    DeadBall,
    BallMoving,
    UnloadingMap,
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
                check_ball_inside_hole,
                // play_collision_sound,
            )
                .chain()
                .run_if(in_state(AppState::BallMoving)), // `chain`ing systems together runs them in order
        )
        .add_systems(OnEnter(AppState::UnloadingMap), unload_map)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Levels(i32, Vec<fn()>);

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

// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut window_query: Query<&mut Window>,
    mut levels_query: Query<&mut Levels>,
) {
    //Resolution
    let mut window = window_query.single_mut();
    window.resolution.set(RESOLUTION.x, RESOLUTION.y);

    // Camera
    commands.spawn(Camera2dBundle::default());

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
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut window_query: Query<&mut Window>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let cursor_position = window_query
        .single_mut()
        .cursor_position()
        .unwrap_or(Vec2::new(0.0, 0.0));

    let ball_position = ball_transform.translation;
    let ball_velocity_vector = Vec2::new(
        cursor_position.x - (ball_position.x + window_query.single_mut().width() / 2.0),
        -cursor_position.y + (window_query.single_mut().height() / 2.0 - ball_position.y),
    );

    // println!("{} {} {} {}", cursor_position.x, ball_position.x + RESOLUTION.x/2.0, cursor_position.y, ball_position.y + RESOLUTION.y/2.0);

    // println!("{} {}", ball_velocity_vector.x, ball_velocity_vector.y);

    if mouse_input.pressed(MouseButton::Left) {
        *ball_velocity = Velocity(ball_velocity_vector);
        app_state_next_state.set(AppState::BallMoving);
        println!("Entered AppState::BallMoving");
    }
}

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
    mut levels_state_next_state: ResMut<NextState<LevelState>>,
    mut levels_queue: Query<&mut LevelsQueue>,
) {
    commands.entity(ball_query.single()).despawn();
    commands.entity(golf_hole_query.single()).despawn();
    for entity in wall_query.iter() {
        commands.entity(entity).despawn();
    }

    let next_level = levels_queue.single_mut().0.pop();

    levels_state_next_state.set(next_level.unwrap());
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
