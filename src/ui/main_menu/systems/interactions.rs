use bevy::app::AppExit;
use bevy::prelude::*;

use crate::ui::main_menu::components::*;
use crate::ui::main_menu::styles::*;
use crate::AppState;

pub fn interact_with_play_button (
    mut button_query: Query<(&Interaction, &mut BackgroundColor),(Changed<Interaction>, With<PlayButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut backgroud_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *backgroud_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Game);
                println!("Entered AppState::Game");
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

