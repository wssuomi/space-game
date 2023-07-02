use bevy::prelude::*;

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

pub fn add_score_over_timer(mut score: ResMut<Score>, score_timer: Res<ScoreTimer>) {
    if score_timer.timer.finished() {
        score.value += 5;
        println!("Score: {}", score.value);
    }
}

pub fn tick_score_timer(mut score_timer: ResMut<ScoreTimer>, time: Res<Time>) {
    score_timer.timer.tick(time.delta());
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<ScoreTimer>()
            .add_systems((add_score_over_timer, tick_score_timer));
    }
}
