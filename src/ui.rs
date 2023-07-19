use bevy::prelude::*;

use crate::{
    assets::UiAssets,
    player::{Player, UpdatePlayerHealth},
    score::{Highscore, Score},
    state::AppState,
};

#[derive(Component)]
struct StartMenu;

#[derive(Component)]
struct HUD;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HealthText;

fn spawn_start_menu(mut commands: Commands, ui_assets: Res<UiAssets>, highscore: Res<Highscore>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            StartMenu {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Space Game",
                    TextStyle {
                        font: ui_assets.menu_font.clone(),
                        font_size: 100.0,
                        color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                    },
                ),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    "Press Enter",
                    TextStyle {
                        font: ui_assets.menu_font.clone(),
                        font_size: 50.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                ),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style { ..default() },
                text: Text::from_section(
                    format!("Highscore: {}", highscore.value),
                    TextStyle {
                        font: ui_assets.menu_font.clone(),
                        font_size: 50.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                ),
                ..default()
            });
        });
}

fn despawn_start_menu(mut commands: Commands, start_menu_query: Query<Entity, With<StartMenu>>) {
    if let Ok(start_menu) = start_menu_query.get_single() {
        commands.entity(start_menu).despawn_recursive();
    }
}

fn hud(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::SpaceBetween,
                        size: Size::new(Val::Px(900.0), Val::Px(900.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            style: Style {
                                size: Size {
                                    width: Val::Auto,
                                    height: Val::Percent(5.0),
                                    ..default()
                                },
                                ..default()
                            },
                            text: Text::from_section(
                                "Score: 0",
                                TextStyle {
                                    font: ui_assets.menu_font.clone(),
                                    font_size: 50.0,
                                    color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                                },
                            ),
                            ..default()
                        },
                        ScoreText {},
                    ));
                    parent.spawn((
                        TextBundle {
                            style: Style {
                                size: Size {
                                    width: Val::Auto,
                                    height: Val::Percent(5.0),
                                    ..default()
                                },
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Health: 100",
                                    TextStyle {
                                        font: ui_assets.menu_font.clone(),
                                        font_size: 50.0,
                                        color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                                    },
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        HealthText {},
                    ));
                });
        });
}

fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    if let Ok(hud) = hud_query.get_single() {
        commands.entity(hud).despawn_recursive();
    }
}

fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Score: {}", score.value.to_string());
        }
    }
}

fn update_health_text(
    mut text_query: Query<&mut Text, With<HealthText>>,
    player_query: Query<&Player, With<Player>>,
    mut event_reader: EventReader<UpdatePlayerHealth>,
) {
    if let Ok(player) = player_query.get_single() {
        for _ in event_reader.iter() {
            for mut text in text_query.iter_mut() {
                text.sections[0].value = format!("Health: {}", player.health.to_string());
            }
        }
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_start_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(hud.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_start_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(update_score_text.in_set(OnUpdate(AppState::Game)))
            .add_system(update_health_text.in_set(OnUpdate(AppState::Game)))
            .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
    }
}
