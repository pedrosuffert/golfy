use bevy::prelude::*;

use crate::ui::main_menu::components::*;
use crate::ui::main_menu::styles::*;

const GOLF_COURSE_COLOR: Color = Color::rgb(0.0, 0.533333, 0.329412);

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut backgroung_color: ResMut<ClearColor>,
    mut window_query: Query<&mut Window>
) {
    backgroung_color.0 = Color::rgb_u8(43, 44, 47);
    window_query.single_mut().cursor.visible = true;
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
    mut backgroung_color: ResMut<ClearColor>,
    mut window_query: Query<&mut Window>
) {
    backgroung_color.0 = GOLF_COURSE_COLOR;
    window_query.single_mut().cursor.visible = false;
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entity = commands.spawn(
        (NodeBundle {
            style: MAIN_MENU_STYLE,
            //background_color: Color::BLUE.into(),
            ..default()
        }, MainMenu{})
    )
    .with_children(|parent| {
        // Title
        parent.spawn(
            NodeBundle {
                style: TITLE_STYLE,
                ..default()
            }
        ).with_children(|parent| {
            // Image 1
            parent.spawn(
                ImageBundle {
                    style: IMAGE_STYLE,
                    image: asset_server.load("sprites/golf-field.png").into(),
                    ..default()
                }
            );
            // Text
//            parent.spawn(
//                TextBundle {
//                    text: Text {
//                        sections: vec![
//                            TextSection::new("Golfy", get_title_text_style(&asset_server),)
//                        ],
//                        alignment: TextAlignment::Center,
//                        ..default()
//                    },
//                    ..default()
//                }
//            );

            parent.spawn(
                ImageBundle {
                    style: TITLE_IMAGE_STYLE,
                    image: asset_server.load("sprites/golfy-01.png").into(),
                    ..default()
                }
            );

            // Image 2
            parent.spawn(
                ImageBundle {
                    style: IMAGE_STYLE,
                    image: asset_server.load("sprites/cruzeiro.png").into(),
                    ..default()
                }
            );
        });

        // Play Button
        parent.spawn(
            (
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                PlayButton {}
            )
        ).with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new("Play", get_button_text_style(&asset_server),)
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
        // Quit Button
        parent.spawn(
            (
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                QuitButton {}
            )
        ).with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new("Quit", get_button_text_style(&asset_server),)
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
    })
    .id();

    main_menu_entity
}