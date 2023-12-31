use bevy::{prelude::*, transform::commands};

mod level1;
use level1::*;
mod level2;
use level2::*;
mod level3;
use level3::*;

use crate::GameState;

#[derive(Resource, PartialEq)]
pub struct Level(pub i32);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct LevelsPlugins;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.533333, 0.329412);

impl Plugin for LevelsPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level(1))
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            // OnEnter Systems
            .add_systems(
                OnEnter(GameState::LoadingMap),
                (
                    load_level_1.run_if(resource_equals(Level(1))),
                    load_level_2.run_if(resource_equals(Level(2))),
                    load_level_3.run_if(resource_equals(Level(3)))
                ),
            );
    }
}

