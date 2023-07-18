use bevy::prelude::*;

use crate::{assets::UiAssets, state::AppState};

#[derive(Component)]
struct StartMenu;

fn spawn_start_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
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
        });
}

fn despawn_start_menu(mut commands: Commands, start_menu_query: Query<Entity, With<StartMenu>>) {
    if let Ok(start_menu) = start_menu_query.get_single() {
        commands.entity(start_menu).despawn_recursive();
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_start_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_start_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
