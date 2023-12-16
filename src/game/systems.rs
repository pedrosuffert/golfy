use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
    transform::commands,
};

use super::components::*;
use crate::game::levels::Level;
use crate::game::swings_count::Scoreboard;
use crate::game::swings_count::Seksu;
use crate::game::GameState;
use crate::*;
// use crate::game::levels::*;

const VELOCITY_VECTOR_QUERY: Color = Color::DARK_GRAY;
const VELOCITY_VECTOR_QUAD_SIZE: Vec3 = Vec3::new(4.0, 100.0, 0.0);
const VELOCITY_VECTOR_TRIANGLE_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);
const VELOCITY_VECTOR_SIZE: f32 = 275.;
const VELOCITY_FACTOR: f32 = 3.5;

const GOLF_COURSE_COLOR: Color = Color::rgb(0.0, 0.533333, 0.329412);

pub fn enter_game_state(
    mut next_app_state: ResMut<NextState<GameState>>,
    mut backgroung_color: ResMut<ClearColor>,
    mut window_query: Query<&mut Window>,
) {
    backgroung_color.0 = GOLF_COURSE_COLOR;
    window_query.single_mut().cursor.visible = false;
    next_app_state.set(GameState::LoadingMap);
    println!("Entered GameState::LoadingMap");
}

// pub fn exit_game_state(mut next_app_state: ResMut<NextState<GameState>>) {
//     next_app_state.set(GameState::OutOfGame);
//     println!("Entered GameState::OutOfGame");
// }

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn spawn_velocity_vector(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            material: materials.add(ColorMaterial::from(VELOCITY_VECTOR_QUERY)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                .with_scale(VELOCITY_VECTOR_QUAD_SIZE)
                .with_rotation(Quat::from_array([0.0, 0.0, 0.0, 0.0])),
            ..default()
        },
        VelocityVectorQuad,
        VelocityVector,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(0.5, 3).into()).into(),
            material: materials.add(ColorMaterial::from(VELOCITY_VECTOR_QUERY)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                .with_scale(VELOCITY_VECTOR_TRIANGLE_SIZE),
            ..default()
        },
        VelocityVectorTriangle,
        VelocityVector,
    ));
}

pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<Seksu>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

pub fn set_ball_velocity(
    mouse_input: Res<Input<MouseButton>>,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    mut app_state_next_state: ResMut<NextState<GameState>>,
    mut window_query: Query<&mut Window>,
    mut vector_quad_query: Query<
        &mut Transform,
        (
            With<VelocityVectorQuad>,
            Without<Ball>,
            Without<VelocityVectorTriangle>,
        ),
    >,
    mut vector_triangle_query: Query<
        &mut Transform,
        (
            With<VelocityVectorTriangle>,
            Without<Ball>,
            Without<VelocityVectorQuad>,
        ),
    >,
    mut scoreboard: ResMut<Scoreboard>,
) {
    // Calculate ball velocity vector
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

    let mut vector_quad = vector_quad_query.single_mut();
    let mut vector_triangle = vector_triangle_query.single_mut();

    let hipotenusa = ball_velocity_vector.length();

    if hipotenusa > VELOCITY_VECTOR_SIZE {
        ball_velocity_vector.x = VELOCITY_VECTOR_SIZE * (ball_velocity_vector.x / hipotenusa);
        ball_velocity_vector.y = VELOCITY_VECTOR_SIZE * (ball_velocity_vector.y / hipotenusa);
    }

    // Rotate vector arrow
    vector_quad.translation.x = ball_velocity_vector.x / 2.0 + ball_position.x;
    vector_quad.translation.y = ball_velocity_vector.y / 2.0 + ball_position.y;
    vector_quad.scale.y = ball_velocity_vector.length();

    vector_triangle.translation.x = ball_velocity_vector.x + ball_position.x;
    vector_triangle.translation.y = ball_velocity_vector.y + ball_position.y;

    let vector_orient = PI / 2.0;
    let mut angle_to_target = ball_velocity_vector.y.atan2(ball_velocity_vector.x);
    if angle_to_target < 0. {
        angle_to_target += 2.0 * PI;
    }
    let angle_to_rotate = angle_to_target - vector_orient;
    vector_quad.rotation = Quat::from_rotation_z(angle_to_rotate);
    vector_triangle.rotation = Quat::from_rotation_z(angle_to_rotate);

    ball_velocity_vector.x *= VELOCITY_FACTOR;
    ball_velocity_vector.y *= VELOCITY_FACTOR;

    if mouse_input.pressed(MouseButton::Left) {
        scoreboard.score += 1;
        *ball_velocity = Velocity(ball_velocity_vector);
        app_state_next_state.set(GameState::BallMoving);
        println!("Entered AppState::BallMoving");
    }
}

pub fn unspawn_velocity_vector(
    mut commands: Commands,
    vector_query: Query<Entity, With<VelocityVector>>,
) {
    for entity in vector_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn check_ball_inside_hole(
    ball_query: Query<&Transform, With<Ball>>,
    hole_query: Query<&Transform, With<GolfHole>>,
    mut app_state_next_state: ResMut<NextState<GameState>>,
) {
    let ball_position = ball_query.single().translation;
    let hole_position = hole_query.single().translation;
    let distance = ball_position.distance(hole_position);

    if distance <= GOLF_HOLE_SIZE.length() / 5.0 {
        app_state_next_state.set(GameState::UnloadingMap);
        println!("Entered AppState::UnloadingMap");
    }
}

pub fn unload_map(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    golf_hole_query: Query<Entity, With<GolfHole>>,
    wall_query: Query<Entity, With<Collider>>,
) {
    if !ball_query.is_empty() {
        commands.entity(ball_query.single()).despawn();
    }
    if !golf_hole_query.is_empty() {
        commands.entity(golf_hole_query.single()).despawn();
        for entity in wall_query.iter() {
            commands.entity(entity).despawn();
        }
    }
    // level_resource.0 += 1;
}

pub fn set_load_map_state(
    mut app_state_next_state: ResMut<NextState<GameState>>,
    mut level_resource: ResMut<Level>,
) {
    level_resource.0 += 1;
    app_state_next_state.set(GameState::LoadingMap);
    println!("Entered AppState::LoadingMap");
}

pub fn uptade_ball_velocity(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut app_state_next_state: ResMut<NextState<GameState>>,
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

    if ball_velocity.x.abs() <= 5.0 && ball_velocity.y.abs() <= 5.0 {
        app_state_next_state.set(GameState::DeadBall);
        println!("Entered AppState::DeadBall");
    } else {
        let sum = ball_velocity.x.abs() + ball_velocity.y.abs();
        if sum >= 0.0 {
            ball_velocity.x -= 2.75 * ball_velocity.x.signum() * (ball_velocity.x.abs() / sum);
            ball_velocity.y -= 2.75 * ball_velocity.y.signum() * (ball_velocity.y.abs() / sum);
        }
    }
}
