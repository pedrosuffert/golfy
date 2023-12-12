use bevy::{prelude::*, transform::commands};

mod level1;
use level1::*;
mod level2;
use level2::*;

use crate::AppState;

#[derive(Component)]
pub struct LevelsQueue(pub Vec<LevelState>);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum LevelState {
    #[default]
    Level1,
    Level2,
}
pub struct LevelsPlugins;

impl Plugin for LevelsPlugins {
    fn build(&self, app: &mut App) {
        app
            // States
            .add_state::<LevelState>()
            .add_systems(Startup, create_levels_queue)
            // OnEnter Systems
            .add_systems(
                OnEnter(AppState::LoadingMap),
                (   
                    load_level_1.run_if(in_state(LevelState::Level1)),
                    load_level_2.run_if(in_state(LevelState::Level2)),
                )
            );
    }
}

fn create_levels_queue(
    mut commands: Commands,
){
    let vec = vec![LevelState::Level2];
    commands.spawn(LevelsQueue(vec));
}
