use bevy::{prelude::*, transform::commands};

mod level1;
use level1::*;
mod level2;
use level2::*;

use crate::AppState;

#[derive(Resource, PartialEq)]
pub struct Level(pub i32);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct LevelsPlugins;

impl Plugin for LevelsPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level(1))
            // OnEnter Systems
            .add_systems(
                OnEnter(AppState::LoadingMap),
                (
                    load_level_1.run_if(resource_equals(Level(1))),
                    load_level_2.run_if(resource_equals(Level(2))),
                ),
            );
    }
}
