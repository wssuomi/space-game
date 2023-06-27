use crate::{
    ARENA_HEIGHT, ARENA_WIDTH, BIG_ROCK_SIZE, FAST_ROCK_SPEED, NORMAL_ROCK_SIZE, NORMAL_ROCK_SPEED,
    ROCK_COOLDOWN, SLOW_ROCK_SPEED, SMALL_ROCK_SIZE,
};
use bevy::prelude::*;
use rand::prelude::*;

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
    pub size: RockSize,
    pub speed: RockSpeed,
}

pub fn spawn_rocks_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    rock_spawn_timer: Res<RockSpawnTimer>,
) {
    if rock_spawn_timer.timer.finished() {
        let random_x = random::<f32>() * ARENA_WIDTH;

        let mut rng = thread_rng();
        let (rock_size, rock_sprite) = match rng.gen_range(0..3) {
            0 => (RockSize::Small, "sprites/small_rock.png"),
            1 => (RockSize::Normal, "sprites/normal_rock.png"),
            _ => (RockSize::Big, "sprites/big_rock.png"),
        };

        let rock_speed: RockSpeed = match rng.gen_range(0..3) {
            0 => RockSpeed::Fast,
            1 => RockSpeed::Normal,
            _ => RockSpeed::Slow,
        };

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, BIG_ROCK_SIZE + ARENA_HEIGHT, 0.0),
                texture: asset_server.load(rock_sprite),
                ..default()
            },
            Rock {
                size: rock_size,
                speed: rock_speed,
            },
        ));
    }
}

pub fn move_rocks(mut rock_query: Query<(&mut Transform, &Rock)>, time: Res<Time>) {
    for (mut transform, rock) in rock_query.iter_mut() {
        let rock_speed = match rock.speed {
            RockSpeed::Fast => FAST_ROCK_SPEED,
            RockSpeed::Normal => NORMAL_ROCK_SPEED,
            RockSpeed::Slow => SLOW_ROCK_SPEED,
        };
        transform.translation.y -= rock_speed * time.delta_seconds();
    }
}

pub fn remove_rocks(
    mut commands: Commands,
    rock_query: Query<(Entity, &Transform, &Rock), With<Rock>>,
) {
    for (rock_entity, rock_transform, rock) in rock_query.iter() {
        let rock_size = match rock.size {
            RockSize::Big => BIG_ROCK_SIZE,
            RockSize::Normal => NORMAL_ROCK_SIZE,
            RockSize::Small => SMALL_ROCK_SIZE,
        };
        if rock_transform.translation.y < 0.0 - rock_size {
            commands.entity(rock_entity).despawn();
        }
    }
}

pub fn tick_rock_spawn_timer(mut rock_spawn_timer: ResMut<RockSpawnTimer>, time: Res<Time>) {
    rock_spawn_timer.timer.tick(time.delta());
}
