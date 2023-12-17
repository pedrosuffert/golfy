use bevy::{prelude::*, transform::commands};

use crate::AppState;
use crate::GameOver;

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}

impl Default for Scoreboard {
    fn default() -> Scoreboard {
        Scoreboard {score: 0}
    }
}

#[derive(Component)]
pub struct Seksu;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct SwingsPlugins;

impl Plugin for SwingsPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(OnEnter(AppState::Game), spawn_swings)
            .add_systems(OnExit(AppState::Game), despawn_swings)
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}

fn spawn_swings(mut commands: Commands) {
    // Scoreboard
    commands.spawn((
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
        Seksu,
    ));
}

fn despawn_swings(mut commands: Commands, swings_query: Query<Entity, With<Text>>) {
    if !swings_query.is_empty() {
        commands.entity(swings_query.single()).despawn();
    }
}

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Scoreboard::default())
}

pub fn remove_score(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    scoreboard: Res<Scoreboard>,
) {
    game_over_event_writer.send(GameOver {final_score: scoreboard.score});
    commands.remove_resource::<Scoreboard>();
}
