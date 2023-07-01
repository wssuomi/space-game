mod arena;
mod assets;
mod player;
mod rock;
mod star;
mod state;

use crate::{
    arena::{ArenaPlugin, ARENA_HEIGHT, ARENA_WIDTH},
    assets::AssetsPlugin,
    player::PlayerPlugin,
    rock::RockPlugin,
    star::StarPlugin,
    state::{start_game, AppState},
};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

pub const SCORE_COOLDOWN: f32 = 1.0;

pub const CLEAR_COLOR: Color = Color::rgb(0.0, 0.0, 0.2);

fn main() {
    App::new()
        .add_state::<AppState>()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .init_resource::<Score>()
        .init_resource::<ScoreTimer>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space game".into(),
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(AssetsPlugin)
        .add_plugin(ArenaPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(RockPlugin)
        .add_startup_system(setup)
        .add_system(tick_score_timer)
        .add_system(add_score_over_timer)
        .add_system(start_game)
        .run();
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct ScoreTimer {
    pub timer: Timer,
}

impl Default for ScoreTimer {
    fn default() -> Self {
        ScoreTimer {
            timer: Timer::from_seconds(SCORE_COOLDOWN, TimerMode::Repeating),
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 5.0),
        ..default()
    });
}

pub fn add_score_over_timer(mut score: ResMut<Score>, score_timer: Res<ScoreTimer>) {
    if score_timer.timer.finished() {
        score.value += 5;
        println!("Score: {}", score.value);
    }
}

pub fn tick_score_timer(mut score_timer: ResMut<ScoreTimer>, time: Res<Time>) {
    score_timer.timer.tick(time.delta());
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 5.0),
        ..default()
    });
}
