use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
    transform::commands,
};

mod game;
use crate::game::*;
use crate::game::systems::*;

mod ui;
use crate::ui::GameUIPlugin;

use std::f32::consts::PI;

//Game Resolution
const RESOLUTION: Vec2 = Vec2::new(1920.0, 1080.0);

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_SIZE: Vec3 = Vec3::new(15.0, 15.0, 0.0);

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const GOLF_HOLE_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);

const BALL_COLOR: Color = Color::WHITE;
const GOLF_HOLE_COLOR: Color = Color::BLACK;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_state::<AppState>()
        .add_plugins(GamePlugin)
        .add_plugins(GameUIPlugin)
        .add_systems(Update, transition_to_main_menu_state)
        .add_systems(Update, transition_to_game_over_menu_state)
        .add_systems(OnEnter(AppState::Game), enter_game_state)
        .add_systems(OnExit(AppState::Game), exit_game_state)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        // .add_systems(OnEnter(AppState::DeadBall), setup_swing)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Brick;

#[derive(Resource)]
struct CollisionSound(Handle<AudioSource>);

#[derive(Event)]
pub struct GameOver {
    pub final_score: usize,
}

// Add the game's entities to our world
fn setup(mut commands: Commands, mut window_query: Query<&mut Window>) {
    //Resolution
    let mut window = window_query.single_mut();
    window.resolution.set(RESOLUTION.x, RESOLUTION.y);

    // Camera
    commands.spawn(Camera2dBundle::default());
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.get() != &AppState::MainMenu {
            
            next_game_state.set(GameState::OutOfGame);
            println!("GameState::OutOfGame");
            next_app_state.set(AppState::MainMenu);
            println!("Entered AppState::MainMenu");
        }
    }
}

pub fn transition_to_game_over_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.get() != &AppState::GameOver {
            
            next_game_state.set(GameState::OutOfGame);
            println!("GameState::OutOfGame");
            next_app_state.set(AppState::GameOver);
            println!("Entered AppState::GameOverMenu");
        }
    }
}

pub fn exit_game_state(
    mut backgroung_color: ResMut<ClearColor>,
    mut window_query: Query<&mut Window>,
){
    backgroung_color.0 = Color::rgb_u8(43, 44, 47);
    window_query.single_mut().cursor.visible = true;
}
