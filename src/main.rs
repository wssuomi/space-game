mod assets;
mod player;
mod rock;
mod star;
mod state;
mod wall;

use crate::{
    assets::AssetsPlugin,
    player::PlayerPlugin,
    state::{check_state, start_game, AppState},
    wall::{ARENA_HEIGHT, ARENA_WIDTH, CLEAR_COLOR},
};

use bevy::prelude::*;

pub const SCORE_COOLDOWN: f32 = 1.0;

fn main() {
    App::new()
        .add_state::<AppState>()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .init_resource::<rock::RockSpawnTimer>()
        .init_resource::<Score>()
        .init_resource::<ScoreTimer>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space game".into(),
                resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AssetsPlugin)
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_startup_system(star::spawn_stars)
        .add_startup_system(wall::spawn_arena_walls)
        .add_system(rock::spawn_rocks_over_time)
        .add_system(rock::tick_rock_spawn_timer)
        .add_system(rock::move_rocks)
        .add_system(rock::remove_rocks)
        .add_system(tick_score_timer)
        .add_system(add_score_over_timer)
        .add_system(star::move_stars)
        .add_system(star::send_star_to_top)
        .add_system(start_game)
        .add_system(check_state)
        .run();
}

#[derive(Resource)]
pub struct PlayerRockCollisionSound(Handle<AudioSource>);

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

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 5.0),
        ..default()
    });

    let player_rock_collision_sound = asset_server.load("audio/rock_hit.ogg");
    commands.insert_resource(PlayerRockCollisionSound(player_rock_collision_sound));
}
