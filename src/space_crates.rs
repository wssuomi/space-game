use bevy::prelude::*;
use rand::random;

use crate::{
    arena::{ARENA_HEIGHT, ARENA_WIDTH},
    assets::SpriteAssets,
};

pub const CRATE_SPEED: f32 = 100.0;
pub const CRATE_WIDTH: f32 = 90.0;
pub const CRATE_HEIGHT: f32 = 54.0;
pub const CRATE_COOLDOWN: f32 = 10.0;

#[derive(Component)]
pub struct HealthCrate;

#[derive(Resource)]
pub struct CrateSpawnTimer {
    pub timer: Timer,
}

impl Default for CrateSpawnTimer {
    fn default() -> Self {
        CrateSpawnTimer {
            timer: Timer::from_seconds(CRATE_COOLDOWN, TimerMode::Repeating),
        }
    }
}

pub fn spawn_crates(
    mut commands: Commands,
    handles: Res<SpriteAssets>,
    crate_spawn_timer: Res<CrateSpawnTimer>,
) {
    if crate_spawn_timer.timer.finished() {
        let random_x = random::<f32>() * ARENA_WIDTH;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, CRATE_HEIGHT + ARENA_HEIGHT - 200.0, 0.0),
                texture: handles.health_crate.clone(),
                ..default()
            },
            HealthCrate {},
        ));
        println!("crate spawned");
    }
}

pub fn tick_crate_spawn_timer(mut crate_spawn_timer: ResMut<CrateSpawnTimer>, time: Res<Time>) {
    crate_spawn_timer.timer.tick(time.delta());
}

pub struct CratePlugin;

impl Plugin for CratePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CrateSpawnTimer>()
            .add_systems((spawn_crates, tick_crate_spawn_timer));
    }
}
