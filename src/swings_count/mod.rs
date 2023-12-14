use bevy::{prelude::*, transform::commands};

use crate::AppState;

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct SwingsPlugins;

impl Plugin for SwingsPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard { score: 0 })
            .add_systems(OnExit(AppState::MainMenu), spawn_swings)
            .add_systems(OnEnter(AppState::MainMenu), despawn_swings);
    }
}

fn spawn_swings(mut commands: Commands) {
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

fn despawn_swings(mut commands: Commands, swings_query: Query<Entity, With<Text>>) {
    if !swings_query.is_empty() {
        commands.entity(swings_query.single()).despawn();
    }
}
