use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
    transform::commands,
};

pub mod systems;
use systems::*;

pub mod components;
use components::*;

pub mod swings_count;
use swings_count::*;

pub mod levels;
use levels::*;

use crate::AppState;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    OutOfGame,
    LoadingMap,
    DeadBall,
    BallMoving,
    UnloadingMap,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct GamePlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.533333, 0.329412);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor::default())
            .add_event::<CollisionEvent>()
            .add_plugins(SwingsPlugins)
            .add_state::<GameState>()
            .add_plugins(LevelsPlugins)
            .add_systems(OnEnter(AppState::Game), enter_game_state)
            .add_systems(OnEnter(GameState::DeadBall), spawn_velocity_vector)
            .add_systems(
                FixedUpdate,
                (set_ball_velocity, update_scoreboard).chain()
                    .run_if(in_state(GameState::DeadBall))
                    .chain(),
            )
            .add_systems(OnExit(GameState::DeadBall), unspawn_velocity_vector)
            .add_systems(
                FixedUpdate,
                (
                    apply_velocity,
                    uptade_ball_velocity,
                    check_ball_inside_hole,
                    // play_collision_sound,
                )
                    .chain()
                    .run_if(in_state(GameState::BallMoving)), // `chain`ing systems together runs them in order
            )
            .add_systems(OnEnter(GameState::UnloadingMap), (unload_map, set_load_map_state).chain())
            .add_systems(OnEnter(GameState::OutOfGame), unload_map)
            ;
    }
}
