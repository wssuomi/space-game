mod arena;
mod assets;
mod gun;
mod player;
mod rock;
mod score;
mod space_crates;
mod star;
mod state;

use crate::{
    arena::{ArenaPlugin, ARENA_HEIGHT, ARENA_WIDTH},
    assets::AssetsPlugin,
    gun::GunPlugin,
    player::PlayerPlugin,
    rock::RockPlugin,
    score::ScorePlugin,
    space_crates::CratePlugin,
    star::StarPlugin,
    state::{go_to_main_menu, start_game, AppState},
};

use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

pub const CLEAR_COLOR: Color = Color::rgb(0.0, 0.0, 0.2);

fn main() {
    App::new()
        .add_state::<AppState>()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space game".into(),
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(AssetsPlugin)
        .add_plugin(ArenaPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(RockPlugin)
        .add_plugin(CratePlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(GunPlugin)
        .add_startup_system(setup)
        .add_system(start_game)
        .add_system(go_to_main_menu)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 5.0),
        ..default()
    });
}
