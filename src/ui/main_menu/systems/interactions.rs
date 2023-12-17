use bevy::app::AppExit;
use bevy::prelude::*;

use crate::ui::main_menu::components::*;
use crate::ui::main_menu::styles::*;
use crate::AppState;

#[derive(Resource)]
pub struct PlayClicked {
    pub value: bool,
}

impl Default for PlayClicked {
    fn default() -> PlayClicked {
        PlayClicked {value: false}
    }
}

pub fn insert_play_clicked(mut commands: Commands) {
    commands.insert_resource(PlayClicked::default())
}

pub fn remove_play_clicked(mut commands: Commands) {
    commands.remove_resource::<PlayClicked>();
}

pub fn interact_with_play_button (
    mut play_clicked: ResMut<PlayClicked>,
    mut button_query: Query<(&Interaction, &mut BackgroundColor),(Changed<Interaction>, With<PlayButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut backgroud_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                play_clicked.value = true;
                *backgroud_color = PRESSED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                if play_clicked.value {
                    app_state_next_state.set(AppState::Game);
                    println!("Entered AppState::Game");
                }
                *backgroud_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                play_clicked.value = false;
                *backgroud_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button (
    mut button_query: Query<(&Interaction, &mut BackgroundColor),(Changed<Interaction>, With<QuitButton>)>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut backgroud_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *backgroud_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *backgroud_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *backgroud_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

