use bevy::app::AppExit;
use bevy::prelude::*;

use crate::ui::game_over_menu::components::*;
use crate::ui::game_over_menu::styles::*;
use crate::AppState;

#[derive(Resource)]
pub struct RestartClicked {
    pub value: bool,
}

impl Default for RestartClicked {
    fn default() -> RestartClicked {
        RestartClicked {value: false}
    }
}

pub fn insert_restart_clicked(mut commands: Commands) {
    commands.insert_resource(RestartClicked::default())
}

pub fn remove_restart_clicked(mut commands: Commands) {
    commands.remove_resource::<RestartClicked>();
}

pub fn interact_with_restart_button(
    mut restart_clicked: ResMut<RestartClicked>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RestartButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                restart_clicked.value = true;
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                if restart_clicked.value {
                    app_state_next_state.set(AppState::Game);
                    println!("Entered AppState::Game");
                }
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                restart_clicked.value = false;
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                app_state_next_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
