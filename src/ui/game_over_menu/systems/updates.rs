use bevy::prelude::*;

use crate::game::swings_count::Scoreboard;
use crate::{GameOver, swings_count};
use crate::ui::game_over_menu::components::FinalScoreText;

//pub fn update_final_score_text(
//    swings_count: ResMut<Scoreboard>,
//    mut text_query: Query<&mut Text, With<FinalScoreText>>,
//) {
//        for mut text in text_query.iter_mut() {
//            text.sections[0].value = format!("Final Score: {}", swings_count.score.to_string());
//        }
//}

pub fn update_final_score_text(
    mut game_over_event_reader: EventReader<GameOver>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    for event in game_over_event_reader.read() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Final Score: {}", event.final_score.to_string());
        }
    }
}
