use bevy::prelude::*;

use crate::state::AppState;

pub const SCORE_COOLDOWN: f32 = 1.0;

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

pub fn add_score_over_time(mut score: ResMut<Score>, score_timer: Res<ScoreTimer>) {
    if score_timer.timer.finished() {
        score.value += 5;
        println!("Score: {}", score.value);
    }
}

pub fn tick_score_timer(mut score_timer: ResMut<ScoreTimer>, time: Res<Time>) {
    score_timer.timer.tick(time.delta());
}

pub fn add_score_resource(mut commands: Commands) {
    commands.insert_resource(Score::default())
}

pub fn add_score_timer_resource(mut commands: Commands) {
    commands.insert_resource(ScoreTimer::default())
}

pub fn remove_score_resource(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn remove_score_timer_resource(mut commands: Commands) {
    commands.remove_resource::<ScoreTimer>();
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (add_score_resource, add_score_timer_resource).in_schedule(OnEnter(AppState::Game)),
        )
        .add_systems((add_score_over_time, tick_score_timer).in_set(OnUpdate(AppState::Game)))
        .add_systems(
            (remove_score_resource, remove_score_timer_resource)
                .in_schedule(OnExit(AppState::Game)),
        );
    }
}
