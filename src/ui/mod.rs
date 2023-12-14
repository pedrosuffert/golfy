mod game_over_menu;
mod main_menu;

use game_over_menu::GameOverMenuPlugin;
use main_menu::MainMenuPlugin;

use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(MainMenuPlugin)
            .add_plugins(GameOverMenuPlugin);
    }
}
