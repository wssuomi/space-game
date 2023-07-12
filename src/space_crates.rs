use bevy::prelude::*;
use rand::random;

use crate::{
    arena::{ARENA_HEIGHT, ARENA_WIDTH},
    assets::SpriteAssets,
    state::AppState,
};

pub const CRATE_SPEED: f32 = 100.0;
pub const CRATE_WIDTH: f32 = 90.0;
pub const CRATE_HEIGHT: f32 = 54.0;
pub const CRATE_COOLDOWN: f32 = 10.0;
pub const CRATE_HEAL: f32 = 20.0;

#[derive(Component)]
pub struct HealthCrate;

#[derive(Component)]
pub struct SpaceCrate;

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
                transform: Transform::from_xyz(random_x, CRATE_HEIGHT + ARENA_HEIGHT, 0.0),
                texture: handles.health_crate.clone(),
                ..default()
            },
            HealthCrate {},
            SpaceCrate {},
        ));
        println!("crate spawned");
    }
}

pub fn tick_crate_spawn_timer(mut crate_spawn_timer: ResMut<CrateSpawnTimer>, time: Res<Time>) {
    crate_spawn_timer.timer.tick(time.delta());
}

pub fn move_crates(mut crate_query: Query<&mut Transform, With<SpaceCrate>>, time: Res<Time>) {
    for mut transform in crate_query.iter_mut() {
        transform.translation.y -= CRATE_SPEED * time.delta_seconds();
    }
}

pub fn remove_off_screen_crates(
    mut commands: Commands,
    crate_query: Query<(Entity, &Transform), With<SpaceCrate>>,
) {
    for (entity, transform) in crate_query.iter() {
        if transform.translation.y < 0.0 - CRATE_HEIGHT {
            commands.entity(entity).despawn();
            println!("crate despawned");
        }
    }
}

pub fn despawn_crates(mut commands: Commands, mut crate_query: Query<Entity, With<SpaceCrate>>) {
    for entity in crate_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

pub fn add_crate_timer_resource(mut commands: Commands) {
    commands.insert_resource(CrateSpawnTimer::default())
}

pub fn remove_crate_timer_resource(mut commands: Commands) {
    commands.remove_resource::<CrateSpawnTimer>();
}

pub struct CratePlugin;

impl Plugin for CratePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_crate_timer_resource.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    spawn_crates,
                    tick_crate_spawn_timer,
                    move_crates,
                    remove_off_screen_crates,
                )
                    .in_set(OnUpdate(AppState::Game)),
            )
            .add_systems(
                (despawn_crates, remove_crate_timer_resource).in_schedule(OnExit(AppState::Game)),
            );
    }
}
