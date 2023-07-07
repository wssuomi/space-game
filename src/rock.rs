use crate::{
    arena::{ARENA_HEIGHT, ARENA_WIDTH},
    assets::SpriteAssets,
    state::AppState,
};
use bevy::prelude::*;
use rand::prelude::*;

pub const ROCK_COOLDOWN: f32 = 2.0;
pub const FAST_ROCK_SPEED: f32 = 100.0;
pub const NORMAL_ROCK_SPEED: f32 = 75.0;
pub const SLOW_ROCK_SPEED: f32 = 50.0;
pub const BIG_ROCK_SIZE: f32 = 150.0;
pub const NORMAL_ROCK_SIZE: f32 = 100.0;
pub const SMALL_ROCK_SIZE: f32 = 70.0;

#[derive(Resource)]
pub struct RockSpawnTimer {
    pub timer: Timer,
}

impl Default for RockSpawnTimer {
    fn default() -> Self {
        RockSpawnTimer {
            timer: Timer::from_seconds(ROCK_COOLDOWN, TimerMode::Repeating),
        }
    }
}

pub enum RockSize {
    Big,
    Normal,
    Small,
}

pub enum RockSpeed {
    Fast,
    Normal,
    Slow,
}

#[derive(Component)]
pub struct Rock {
    pub rock_size: RockSize,
    pub rock_speed: RockSpeed,
}

impl Rock {
    pub fn speed(&self) -> f32 {
        match self.rock_speed {
            RockSpeed::Fast => FAST_ROCK_SPEED,
            RockSpeed::Normal => NORMAL_ROCK_SPEED,
            RockSpeed::Slow => SLOW_ROCK_SPEED,
        }
    }

    pub fn size(&self) -> f32 {
        match self.rock_size {
            RockSize::Big => BIG_ROCK_SIZE,
            RockSize::Normal => NORMAL_ROCK_SIZE,
            RockSize::Small => SMALL_ROCK_SIZE,
        }
    }
    pub fn damage(&self) -> f32 {
        let base = match self.rock_speed {
            RockSpeed::Fast => 10.0,
            RockSpeed::Normal => 7.5,
            RockSpeed::Slow => 5.0,
        };

        let multiplier = match self.rock_size {
            RockSize::Big => 3.0,
            RockSize::Normal => 2.0,
            RockSize::Small => 1.0,
        };

        return base * multiplier;
    }
}

pub fn spawn_rocks_over_time(
    mut commands: Commands,
    handles: Res<SpriteAssets>,
    rock_spawn_timer: Res<RockSpawnTimer>,
) {
    if rock_spawn_timer.timer.finished() {
        let random_x = random::<f32>() * ARENA_WIDTH;

        let mut rng = thread_rng();
        let (rock_size, rock_sprite) = match rng.gen_range(0..3) {
            0 => (RockSize::Small, handles.small_rock.clone()),
            1 => (RockSize::Normal, handles.normal_rock.clone()),
            _ => (RockSize::Big, handles.big_rock.clone()),
        };

        let rock_speed: RockSpeed = match rng.gen_range(0..3) {
            0 => RockSpeed::Fast,
            1 => RockSpeed::Normal,
            _ => RockSpeed::Slow,
        };

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, BIG_ROCK_SIZE + ARENA_HEIGHT, 0.0),
                texture: rock_sprite,
                ..default()
            },
            Rock {
                rock_size,
                rock_speed,
            },
        ));
    }
}

pub fn move_rocks(mut rock_query: Query<(&mut Transform, &Rock)>, time: Res<Time>) {
    for (mut transform, rock) in rock_query.iter_mut() {
        transform.translation.y -= rock.speed() * time.delta_seconds();
    }
}

pub fn remove_off_screen_rocks(
    mut commands: Commands,
    rock_query: Query<(Entity, &Transform, &Rock), With<Rock>>,
) {
    for (rock_entity, rock_transform, rock) in rock_query.iter() {
        if rock_transform.translation.y < 0.0 - rock.size() {
            commands.entity(rock_entity).despawn();
        }
    }
}

pub fn tick_rock_spawn_timer(mut rock_spawn_timer: ResMut<RockSpawnTimer>, time: Res<Time>) {
    rock_spawn_timer.timer.tick(time.delta());
}

pub fn despawn_rocks(mut commands: Commands, rock_query: Query<Entity, With<Rock>>) {
    for entity in rock_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RockSpawnTimer>()
            .add_systems(
                (
                    spawn_rocks_over_time,
                    tick_rock_spawn_timer,
                    move_rocks,
                    remove_off_screen_rocks,
                )
                    .in_set(OnUpdate(AppState::Game)),
            )
            .add_system(despawn_rocks.in_schedule(OnExit(AppState::Game)));
    }
}
