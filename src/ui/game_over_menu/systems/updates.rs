use bevy::prelude::*;

use crate::game::swings_count::Scoreboard;
use crate::{GameOver, swings_count};
use crate::ui::game_over_menu::components::FinalScoreText;

pub fn update_final_score_text(
    mut swings_count: ResMut<Scoreboard>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Final Score: {}", swings_count.score.to_string());
        }
}
